
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
#ifndef __windows2Eweb2Ehttp2Ediagnostics_h__
#define __windows2Eweb2Ehttp2Ediagnostics_h__
#ifndef __windows2Eweb2Ehttp2Ediagnostics_p_h__
#define __windows2Eweb2Ehttp2Ediagnostics_p_h__


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

#if !defined(WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION)
#define WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.System.Diagnostics.h"
#include "Windows.Web.Http.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticProvider;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProvider

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticProviderRequestResponseCompletedEventArgs;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderRequestResponseCompletedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticProviderRequestResponseTimestamps;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderRequestResponseTimestamps

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticProviderRequestSentEventArgs;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderRequestSentEventArgs

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticProviderResponseReceivedEventArgs;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderResponseReceivedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticProviderStatics;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderStatics

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    interface IHttpDiagnosticSourceLocation;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticSourceLocation

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    class HttpDiagnosticSourceLocation;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE
#define DEF___FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("807b6ed4-5f42-5199-b231-60cce2c76940"))
IIterator<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticSourceLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*> __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_t;
#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE */

#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE
#define DEF___FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4286ca1a-a4c5-5ae8-9da9-5bfa24768e22"))
IIterable<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticSourceLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*> __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_t;
#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE */

#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE
#define DEF___FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9af27d3-80e8-5aff-a7e5-1b221787f096"))
IVectorView<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticSourceLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticSourceLocation*> __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_t;
#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_USE */

#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    class HttpDiagnosticProvider;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    class HttpDiagnosticProviderRequestResponseCompletedEventArgs;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2281ad23-9c8d-5d82-9b20-bcf157b04fd9"))
ITypedEventHandler<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderRequestResponseCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderRequestResponseCompletedEventArgs*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderRequestResponseCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.Http.Diagnostics.HttpDiagnosticProvider, Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderRequestResponseCompletedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_USE */

#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    class HttpDiagnosticProviderRequestSentEventArgs;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f8e10321-47bd-526a-a9b2-3bf12f725f8b"))
ITypedEventHandler<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderRequestSentEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderRequestSentEventArgs*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderRequestSentEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.Http.Diagnostics.HttpDiagnosticProvider, Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderRequestSentEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_USE */

#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    class HttpDiagnosticProviderResponseReceivedEventArgs;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2eef1846-8176-52af-8d2b-e0c932512e9f"))
ITypedEventHandler<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderResponseReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderResponseReceivedEventArgs*, ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderResponseReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.Http.Diagnostics.HttpDiagnosticProvider, Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProvider*, ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticProviderResponseReceivedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_USE */

#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

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
        namespace System {
            namespace Diagnostics {
                class ProcessDiagnosticInfo;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiagnosticInfo;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage ABI::Windows::Web::Http::IHttpRequestMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage ABI::Windows::Web::Http::IHttpResponseMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    typedef enum HttpDiagnosticRequestInitiator : int HttpDiagnosticRequestInitiator;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    class HttpDiagnosticProviderRequestResponseTimestamps;
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    enum HttpDiagnosticRequestInitiator : int
                    {
                        HttpDiagnosticRequestInitiator_ParsedElement = 0,
                        HttpDiagnosticRequestInitiator_Script = 1,
                        HttpDiagnosticRequestInitiator_Image = 2,
                        HttpDiagnosticRequestInitiator_Link = 3,
                        HttpDiagnosticRequestInitiator_Style = 4,
                        HttpDiagnosticRequestInitiator_XmlHttpRequest = 5,
                        HttpDiagnosticRequestInitiator_Media = 6,
                        HttpDiagnosticRequestInitiator_HtmlDownload = 7,
                        HttpDiagnosticRequestInitiator_Prefetch = 8,
                        HttpDiagnosticRequestInitiator_Other = 9,
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
                        HttpDiagnosticRequestInitiator_CrossOriginPreFlight = 10,
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
                        HttpDiagnosticRequestInitiator_Fetch = 11,
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
                        HttpDiagnosticRequestInitiator_Beacon = 12,
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
                    };
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProvider
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProvider[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("bd811501-a056-4d39-b174-833b7b03b02c")
                    IHttpDiagnosticProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RequestSent(
                            __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RequestSent(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ResponseReceived(
                            __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ResponseReceived(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RequestResponseCompleted(
                            __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RequestResponseCompleted(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticProvider = __uuidof(IHttpDiagnosticProvider);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderRequestResponseCompletedEventArgs[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("735f98ee-94f6-4532-b26e-61e1b1e4efd4")
                    IHttpDiagnosticProviderRequestResponseCompletedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamps(
                            ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProviderRequestResponseTimestamps** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RequestedUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProcessId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ThreadId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Initiator(
                            ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticRequestInitiator* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceLocations(
                            __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticProviderRequestResponseCompletedEventArgs = __uuidof(IHttpDiagnosticProviderRequestResponseCompletedEventArgs);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderRequestResponseTimestamps[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("e0afde10-55cf-4c01-91d4-a20557d849f0")
                    IHttpDiagnosticProviderRequestResponseTimestamps : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CacheCheckedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ConnectionInitiatedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NameResolvedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SslNegotiatedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ConnectionCompletedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RequestSentTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RequestCompletedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseReceivedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseCompletedTimestamp(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticProviderRequestResponseTimestamps = __uuidof(IHttpDiagnosticProviderRequestResponseTimestamps);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderRequestSentEventArgs[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("3f5196d0-4c1f-4ebe-a57a-06930771c50d")
                    IHttpDiagnosticProviderRequestSentEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Message(
                            ABI::Windows::Web::Http::IHttpRequestMessage** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProcessId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ThreadId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Initiator(
                            ABI::Windows::Web::Http::Diagnostics::HttpDiagnosticRequestInitiator* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceLocations(
                            __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticProviderRequestSentEventArgs = __uuidof(IHttpDiagnosticProviderRequestSentEventArgs);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderResponseReceivedEventArgs[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("a0a2566c-ab5f-4d66-bb2d-084cf41635d0")
                    IHttpDiagnosticProviderResponseReceivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Message(
                            ABI::Windows::Web::Http::IHttpResponseMessage** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticProviderResponseReceivedEventArgs = __uuidof(IHttpDiagnosticProviderResponseReceivedEventArgs);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProvider
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderStatics[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("5b824ec1-6a6c-47cc-afec-1e86bc26053b")
                    IHttpDiagnosticProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateFromProcessDiagnosticInfo(
                            ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo* processDiagnosticInfo,
                            ABI::Windows::Web::Http::Diagnostics::IHttpDiagnosticProvider** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticProviderStatics = __uuidof(IHttpDiagnosticProviderStatics);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticSourceLocation[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Diagnostics {
                    MIDL_INTERFACE("54a9d260-8860-423f-b6fa-d77716f647a7")
                    IHttpDiagnosticSourceLocation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LineNumber(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColumnNumber(
                            UINT64* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpDiagnosticSourceLocation = __uuidof(IHttpDiagnosticSourceLocation);
                } /* Diagnostics */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProvider
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics interface starting with version 1.0 of the Windows.Web.Http.Diagnostics.HttpDiagnosticsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProvider_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProvider[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseCompletedEventArgs[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseTimestamps_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseTimestamps_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseTimestamps[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestSentEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestSentEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestSentEventArgs[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderResponseReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderResponseReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderResponseReceivedEventArgs[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticSourceLocation_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticSourceLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticSourceLocation[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation;

typedef struct __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl;

interface __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation;

typedef struct __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        __FIIterator_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl;

interface __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation;

typedef struct __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl;

interface __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation_INTERFACE_DEFINED__
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* sender,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* sender,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* sender,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CHttpDiagnosticRequestInitiator __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CHttpDiagnosticRequestInitiator;

/*
 *
 * Struct Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CHttpDiagnosticRequestInitiator
{
    HttpDiagnosticRequestInitiator_ParsedElement = 0,
    HttpDiagnosticRequestInitiator_Script = 1,
    HttpDiagnosticRequestInitiator_Image = 2,
    HttpDiagnosticRequestInitiator_Link = 3,
    HttpDiagnosticRequestInitiator_Style = 4,
    HttpDiagnosticRequestInitiator_XmlHttpRequest = 5,
    HttpDiagnosticRequestInitiator_Media = 6,
    HttpDiagnosticRequestInitiator_HtmlDownload = 7,
    HttpDiagnosticRequestInitiator_Prefetch = 8,
    HttpDiagnosticRequestInitiator_Other = 9,
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
    HttpDiagnosticRequestInitiator_CrossOriginPreFlight = 10,
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
    HttpDiagnosticRequestInitiator_Fetch = 11,
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
    HttpDiagnosticRequestInitiator_Beacon = 12,
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x20000
};
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProvider
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProvider[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This);
    HRESULT (STDMETHODCALLTYPE* add_RequestSent)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestSentEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RequestSent)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ResponseReceived)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderResponseReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ResponseReceived)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RequestResponseCompleted)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        __FITypedEventHandler_2_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProvider_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticProviderRequestResponseCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RequestResponseCompleted)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_add_RequestSent(This, handler, token) \
    ((This)->lpVtbl->add_RequestSent(This, handler, token))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_remove_RequestSent(This, token) \
    ((This)->lpVtbl->remove_RequestSent(This, token))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_add_ResponseReceived(This, handler, token) \
    ((This)->lpVtbl->add_ResponseReceived(This, handler, token))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_remove_ResponseReceived(This, token) \
    ((This)->lpVtbl->remove_ResponseReceived(This, token))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_add_RequestResponseCompleted(This, handler, token) \
    ((This)->lpVtbl->add_RequestResponseCompleted(This, handler, token))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_remove_RequestResponseCompleted(This, token) \
    ((This)->lpVtbl->remove_RequestResponseCompleted(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderRequestResponseCompletedEventArgs[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamps)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedUri)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ProcessId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ThreadId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Initiator)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CHttpDiagnosticRequestInitiator* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceLocations)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs* This,
        __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_Timestamps(This, value) \
    ((This)->lpVtbl->get_Timestamps(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_RequestedUri(This, value) \
    ((This)->lpVtbl->get_RequestedUri(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_ProcessId(This, value) \
    ((This)->lpVtbl->get_ProcessId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_ThreadId(This, value) \
    ((This)->lpVtbl->get_ThreadId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_Initiator(This, value) \
    ((This)->lpVtbl->get_Initiator(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_get_SourceLocations(This, value) \
    ((This)->lpVtbl->get_SourceLocations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderRequestResponseTimestamps[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestampsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CacheCheckedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionInitiatedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_NameResolvedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_SslNegotiatedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionCompletedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestSentTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestCompletedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseReceivedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseCompletedTimestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestampsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestampsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_CacheCheckedTimestamp(This, value) \
    ((This)->lpVtbl->get_CacheCheckedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_ConnectionInitiatedTimestamp(This, value) \
    ((This)->lpVtbl->get_ConnectionInitiatedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_NameResolvedTimestamp(This, value) \
    ((This)->lpVtbl->get_NameResolvedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_SslNegotiatedTimestamp(This, value) \
    ((This)->lpVtbl->get_SslNegotiatedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_ConnectionCompletedTimestamp(This, value) \
    ((This)->lpVtbl->get_ConnectionCompletedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_RequestSentTimestamp(This, value) \
    ((This)->lpVtbl->get_RequestSentTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_RequestCompletedTimestamp(This, value) \
    ((This)->lpVtbl->get_RequestCompletedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_ResponseReceivedTimestamp(This, value) \
    ((This)->lpVtbl->get_ResponseReceivedTimestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_get_ResponseCompletedTimestamp(This, value) \
    ((This)->lpVtbl->get_ResponseCompletedTimestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestResponseTimestamps_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderRequestSentEventArgs[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ProcessId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ThreadId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Initiator)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CHttpDiagnosticRequestInitiator* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceLocations)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs* This,
        __FIVectorView_1_Windows__CWeb__CHttp__CDiagnostics__CHttpDiagnosticSourceLocation** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_ProcessId(This, value) \
    ((This)->lpVtbl->get_ProcessId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_ThreadId(This, value) \
    ((This)->lpVtbl->get_ThreadId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_Initiator(This, value) \
    ((This)->lpVtbl->get_Initiator(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_get_SourceLocations(This, value) \
    ((This)->lpVtbl->get_SourceLocations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderRequestSentEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderResponseReceivedEventArgs[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderResponseReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticProvider
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticProviderStatics[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromProcessDiagnosticInfo)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* processDiagnosticInfo,
        __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStaticsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_CreateFromProcessDiagnosticInfo(This, processDiagnosticInfo, value) \
    ((This)->lpVtbl->CreateFromProcessDiagnosticInfo(This, processDiagnosticInfo, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Diagnostics_IHttpDiagnosticSourceLocation[] = L"Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceUri)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_LineNumber)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_ColumnNumber)(__x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocationVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_get_SourceUri(This, value) \
    ((This)->lpVtbl->get_SourceUri(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_get_LineNumber(This, value) \
    ((This)->lpVtbl->get_LineNumber(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_get_ColumnNumber(This, value) \
    ((This)->lpVtbl->get_ColumnNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CDiagnostics_CIHttpDiagnosticSourceLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProvider
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics interface starting with version 1.0 of the Windows.Web.Http.Diagnostics.HttpDiagnosticsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProvider_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProvider[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseCompletedEventArgs[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseTimestamps_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseTimestamps_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestResponseTimestamps[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestSentEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestSentEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderRequestSentEventArgs[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderResponseReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderResponseReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticProviderResponseReceivedEventArgs[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation
 *
 * Introduced to Windows.Web.Http.Diagnostics.HttpDiagnosticsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticSourceLocation_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Diagnostics_HttpDiagnosticSourceLocation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Diagnostics_HttpDiagnosticSourceLocation[] = L"Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
#endif
#endif // WINDOWS_WEB_HTTP_DIAGNOSTICS_HTTPDIAGNOSTICSCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eweb2Ehttp2Ediagnostics_p_h__

#endif // __windows2Eweb2Ehttp2Ediagnostics_h__
