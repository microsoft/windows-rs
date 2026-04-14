
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
#ifndef __windows2Esystem2Ediagnostics2Etracereporting_h__
#define __windows2Esystem2Ediagnostics2Etracereporting_h__
#ifndef __windows2Esystem2Ediagnostics2Etracereporting_p_h__
#define __windows2Esystem2Ediagnostics2Etracereporting_p_h__


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
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    interface IPlatformDiagnosticActionsStatics;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticActionsStatics

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    interface IPlatformDiagnosticTraceInfo;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticTraceInfo

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    interface IPlatformDiagnosticTraceRuntimeInfo;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticTraceRuntimeInfo

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIIterator_1_GUID_USE
#define DEF___FIIterator_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3d64048-82b3-53c7-9285-b0be18368482"))
IIterator<GUID> : IIterator_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<GUID> __FIIterator_1_GUID_t;
#define __FIIterator_1_GUID ABI::Windows::Foundation::Collections::__FIIterator_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_GUID_USE */



#ifndef DEF___FIIterable_1_GUID_USE
#define DEF___FIIterable_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4ca3045-5dd7-54be-982e-d88d8ca0876e"))
IIterable<GUID> : IIterable_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<GUID> __FIIterable_1_GUID_t;
#define __FIIterable_1_GUID ABI::Windows::Foundation::Collections::__FIIterable_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_GUID_USE */



#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    class PlatformDiagnosticTraceInfo;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE
#define DEF___FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1af4598d-98bb-5e51-842b-cf691925b6c2"))
IIterator<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*, ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticTraceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*> __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_t;
#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE
#define DEF___FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ecb0c107-c97b-52fe-a5e6-a33e93493769"))
IIterable<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*, ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticTraceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*> __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_t;
#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIVectorView_1_GUID_USE
#define DEF___FIVectorView_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9520e64b-15b2-52a6-98ed-3191fa6cf68a"))
IVectorView<GUID> : IVectorView_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<GUID> __FIVectorView_1_GUID_t;
#define __FIVectorView_1_GUID ABI::Windows::Foundation::Collections::__FIVectorView_1_GUID_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_GUID_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE
#define DEF___FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8f1b3397-4dc3-5b72-91fa-0fdc915d950c"))
IVectorView<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*, ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticTraceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceInfo*> __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_t;
#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    typedef enum PlatformDiagnosticActionState : int PlatformDiagnosticActionState;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    typedef enum PlatformDiagnosticEscalationType : int PlatformDiagnosticEscalationType;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    typedef enum PlatformDiagnosticEventBufferLatencies : unsigned int PlatformDiagnosticEventBufferLatencies;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    typedef enum PlatformDiagnosticTracePriority : int PlatformDiagnosticTracePriority;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    typedef enum PlatformDiagnosticTraceSlotState : int PlatformDiagnosticTraceSlotState;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    typedef enum PlatformDiagnosticTraceSlotType : int PlatformDiagnosticTraceSlotType;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    class PlatformDiagnosticTraceRuntimeInfo;
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    enum PlatformDiagnosticActionState : int
                    {
                        PlatformDiagnosticActionState_Success = 0,
                        PlatformDiagnosticActionState_FreeNetworkNotAvailable = 1,
                        PlatformDiagnosticActionState_ACPowerNotAvailable = 2,
                    };
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEscalationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    enum PlatformDiagnosticEscalationType : int
                    {
                        PlatformDiagnosticEscalationType_OnCompletion = 0,
                        PlatformDiagnosticEscalationType_OnFailure = 1,
                    };
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEventBufferLatencies
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    enum PlatformDiagnosticEventBufferLatencies : unsigned int
                    {
                        PlatformDiagnosticEventBufferLatencies_Normal = 0x1,
                        PlatformDiagnosticEventBufferLatencies_CostDeferred = 0x2,
                        PlatformDiagnosticEventBufferLatencies_Realtime = 0x4,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(PlatformDiagnosticEventBufferLatencies)
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTracePriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    enum PlatformDiagnosticTracePriority : int
                    {
                        PlatformDiagnosticTracePriority_Normal = 0,
                        PlatformDiagnosticTracePriority_UserElevated = 1,
                    };
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    enum PlatformDiagnosticTraceSlotState : int
                    {
                        PlatformDiagnosticTraceSlotState_NotRunning = 0,
                        PlatformDiagnosticTraceSlotState_Running = 1,
                        PlatformDiagnosticTraceSlotState_Throttled = 2,
                    };
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    enum PlatformDiagnosticTraceSlotType : int
                    {
                        PlatformDiagnosticTraceSlotType_Alternative = 0,
                        PlatformDiagnosticTraceSlotType_AlwaysOn = 1,
                        PlatformDiagnosticTraceSlotType_Mini = 2,
                    };
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_TraceReporting_IPlatformDiagnosticActionsStatics[] = L"Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    MIDL_INTERFACE("c1145cfa-9292-4267-890a-9ea3ed072312")
                    IPlatformDiagnosticActionsStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsScenarioEnabled(
                            GUID scenarioId,
                            boolean* isActive
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryEscalateScenario(
                            GUID scenarioId,
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticEscalationType escalationType,
                            HSTRING outputDirectory,
                            boolean timestampOutputDirectory,
                            boolean forceEscalationUpload,
                            __FIMapView_2_HSTRING_HSTRING* triggers,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DownloadLatestSettingsForNamespace(
                            HSTRING partner,
                            HSTRING feature,
                            boolean isScenarioNamespace,
                            boolean downloadOverCostedNetwork,
                            boolean downloadOverBattery,
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticActionState* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetActiveScenarioList(
                            __FIVectorView_1_GUID** scenarioIds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ForceUpload(
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticEventBufferLatencies latency,
                            boolean uploadOverCostedNetwork,
                            boolean uploadOverBattery,
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticActionState* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsTraceRunning(
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceSlotType slotType,
                            GUID scenarioId,
                            UINT64 traceProfileHash,
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceSlotState* slotState
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetActiveTraceRuntime(
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceSlotType slotType,
                            ABI::Windows::System::Diagnostics::TraceReporting::IPlatformDiagnosticTraceRuntimeInfo** traceRuntimeInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetKnownTraceList(
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTraceSlotType slotType,
                            __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo** traceInfo
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlatformDiagnosticActionsStatics = __uuidof(IPlatformDiagnosticActionsStatics);
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_TraceReporting_IPlatformDiagnosticTraceInfo[] = L"Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    MIDL_INTERFACE("f870ed97-d597-4bf7-88dc-cf5c7dc2a1d2")
                    IPlatformDiagnosticTraceInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ScenarioId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProfileHash(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsExclusive(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAutoLogger(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxTraceDurationFileTime(
                            INT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Priority(
                            ABI::Windows::System::Diagnostics::TraceReporting::PlatformDiagnosticTracePriority* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlatformDiagnosticTraceInfo = __uuidof(IPlatformDiagnosticTraceInfo);
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_TraceReporting_IPlatformDiagnosticTraceRuntimeInfo[] = L"Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace TraceReporting {
                    MIDL_INTERFACE("3d4d5e2d-01d8-4768-8554-1eb1ca610986")
                    IPlatformDiagnosticTraceRuntimeInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RuntimeFileTime(
                            INT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EtwRuntimeFileTime(
                            INT64* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlatformDiagnosticTraceRuntimeInfo = __uuidof(IPlatformDiagnosticTraceRuntimeInfo);
                } /* TraceReporting */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticActions_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticActions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticActions[] = L"Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceInfo[] = L"Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceRuntimeInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceRuntimeInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceRuntimeInfo[] = L"Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIIterator_1_GUID_INTERFACE_DEFINED__)
#define ____FIIterator_1_GUID_INTERFACE_DEFINED__

typedef interface __FIIterator_1_GUID __FIIterator_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_GUID;

typedef struct __FIIterator_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_GUID* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_GUID* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_GUID* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_GUID* This,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_GUIDVtbl;

interface __FIIterator_1_GUID
{
    CONST_VTBL struct __FIIterator_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_GUID_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_GUID_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_GUID_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_GUID_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_GUID_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_GUID_INTERFACE_DEFINED__)
#define ____FIIterable_1_GUID_INTERFACE_DEFINED__

typedef interface __FIIterable_1_GUID __FIIterable_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_GUID;

typedef struct __FIIterable_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_GUID* This,
        __FIIterator_1_GUID** result);

    END_INTERFACE
} __FIIterable_1_GUIDVtbl;

interface __FIIterable_1_GUID
{
    CONST_VTBL struct __FIIterable_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_GUID_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_GUID_INTERFACE_DEFINED__

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo;

typedef struct __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl;

interface __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo;

typedef struct __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        __FIIterator_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl;

interface __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_GUID_INTERFACE_DEFINED__)
#define ____FIVectorView_1_GUID_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_GUID __FIVectorView_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_GUID;

typedef struct __FIVectorView_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_GUID* This,
        UINT32 index,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_GUID* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_GUID* This,
        GUID value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_GUID* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        GUID* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_GUIDVtbl;

interface __FIVectorView_1_GUID
{
    CONST_VTBL struct __FIVectorView_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_GUID_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_GUID_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_GUID_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_GUID_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_GUID_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo;

typedef struct __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl;

interface __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticActionState __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticActionState;

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEscalationType __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEscalationType;

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEventBufferLatencies __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEventBufferLatencies;

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTracePriority __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTracePriority;

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotState __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotState;

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotType __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotType;

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticActionState
{
    PlatformDiagnosticActionState_Success = 0,
    PlatformDiagnosticActionState_FreeNetworkNotAvailable = 1,
    PlatformDiagnosticActionState_ACPowerNotAvailable = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEscalationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEscalationType
{
    PlatformDiagnosticEscalationType_OnCompletion = 0,
    PlatformDiagnosticEscalationType_OnFailure = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticEventBufferLatencies
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEventBufferLatencies
{
    PlatformDiagnosticEventBufferLatencies_Normal = 0x1,
    PlatformDiagnosticEventBufferLatencies_CostDeferred = 0x2,
    PlatformDiagnosticEventBufferLatencies_Realtime = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTracePriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTracePriority
{
    PlatformDiagnosticTracePriority_Normal = 0,
    PlatformDiagnosticTracePriority_UserElevated = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotState
{
    PlatformDiagnosticTraceSlotState_NotRunning = 0,
    PlatformDiagnosticTraceSlotState_Running = 1,
    PlatformDiagnosticTraceSlotState_Throttled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceSlotType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotType
{
    PlatformDiagnosticTraceSlotType_Alternative = 0,
    PlatformDiagnosticTraceSlotType_AlwaysOn = 1,
    PlatformDiagnosticTraceSlotType_Mini = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_TraceReporting_IPlatformDiagnosticActionsStatics[] = L"Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsScenarioEnabled)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        GUID scenarioId,
        boolean* isActive);
    HRESULT (STDMETHODCALLTYPE* TryEscalateScenario)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        GUID scenarioId,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEscalationType escalationType,
        HSTRING outputDirectory,
        boolean timestampOutputDirectory,
        boolean forceEscalationUpload,
        __FIMapView_2_HSTRING_HSTRING* triggers,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* DownloadLatestSettingsForNamespace)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        HSTRING partner,
        HSTRING feature,
        boolean isScenarioNamespace,
        boolean downloadOverCostedNetwork,
        boolean downloadOverBattery,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticActionState* result);
    HRESULT (STDMETHODCALLTYPE* GetActiveScenarioList)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        __FIVectorView_1_GUID** scenarioIds);
    HRESULT (STDMETHODCALLTYPE* ForceUpload)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticEventBufferLatencies latency,
        boolean uploadOverCostedNetwork,
        boolean uploadOverBattery,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticActionState* result);
    HRESULT (STDMETHODCALLTYPE* IsTraceRunning)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotType slotType,
        GUID scenarioId,
        UINT64 traceProfileHash,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotState* slotState);
    HRESULT (STDMETHODCALLTYPE* GetActiveTraceRuntime)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotType slotType,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo** traceRuntimeInfo);
    HRESULT (STDMETHODCALLTYPE* GetKnownTraceList)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTraceSlotType slotType,
        __FIVectorView_1_Windows__CSystem__CDiagnostics__CTraceReporting__CPlatformDiagnosticTraceInfo** traceInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_IsScenarioEnabled(This, scenarioId, isActive) \
    ((This)->lpVtbl->IsScenarioEnabled(This, scenarioId, isActive))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_TryEscalateScenario(This, scenarioId, escalationType, outputDirectory, timestampOutputDirectory, forceEscalationUpload, triggers, result) \
    ((This)->lpVtbl->TryEscalateScenario(This, scenarioId, escalationType, outputDirectory, timestampOutputDirectory, forceEscalationUpload, triggers, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_DownloadLatestSettingsForNamespace(This, partner, feature, isScenarioNamespace, downloadOverCostedNetwork, downloadOverBattery, result) \
    ((This)->lpVtbl->DownloadLatestSettingsForNamespace(This, partner, feature, isScenarioNamespace, downloadOverCostedNetwork, downloadOverBattery, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_GetActiveScenarioList(This, scenarioIds) \
    ((This)->lpVtbl->GetActiveScenarioList(This, scenarioIds))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_ForceUpload(This, latency, uploadOverCostedNetwork, uploadOverBattery, result) \
    ((This)->lpVtbl->ForceUpload(This, latency, uploadOverCostedNetwork, uploadOverBattery, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_IsTraceRunning(This, slotType, scenarioId, traceProfileHash, slotState) \
    ((This)->lpVtbl->IsTraceRunning(This, slotType, scenarioId, traceProfileHash, slotState))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_GetActiveTraceRuntime(This, slotType, traceRuntimeInfo) \
    ((This)->lpVtbl->GetActiveTraceRuntime(This, slotType, traceRuntimeInfo))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_GetKnownTraceList(This, slotType, traceInfo) \
    ((This)->lpVtbl->GetKnownTraceList(This, slotType, traceInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticActionsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_TraceReporting_IPlatformDiagnosticTraceInfo[] = L"Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ScenarioId)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfileHash)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_IsExclusive)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAutoLogger)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxTraceDurationFileTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Priority)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CPlatformDiagnosticTracePriority* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfoVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_get_ScenarioId(This, value) \
    ((This)->lpVtbl->get_ScenarioId(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_get_ProfileHash(This, value) \
    ((This)->lpVtbl->get_ProfileHash(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_get_IsExclusive(This, value) \
    ((This)->lpVtbl->get_IsExclusive(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_get_IsAutoLogger(This, value) \
    ((This)->lpVtbl->get_IsAutoLogger(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_get_MaxTraceDurationFileTime(This, value) \
    ((This)->lpVtbl->get_MaxTraceDurationFileTime(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_get_Priority(This, value) \
    ((This)->lpVtbl->get_Priority(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_TraceReporting_IPlatformDiagnosticTraceRuntimeInfo[] = L"Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RuntimeFileTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_EtwRuntimeFileTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo* This,
        INT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfoVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_get_RuntimeFileTime(This, value) \
    ((This)->lpVtbl->get_RuntimeFileTime(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_get_EtwRuntimeFileTime(This, value) \
    ((This)->lpVtbl->get_EtwRuntimeFileTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTraceReporting_CIPlatformDiagnosticTraceRuntimeInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticActionsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticActions_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticActions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticActions[] = L"Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticActions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceInfo[] = L"Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.TraceReporting.IPlatformDiagnosticTraceRuntimeInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceRuntimeInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceRuntimeInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_TraceReporting_PlatformDiagnosticTraceRuntimeInfo[] = L"Windows.System.Diagnostics.TraceReporting.PlatformDiagnosticTraceRuntimeInfo";
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
#endif // __windows2Esystem2Ediagnostics2Etracereporting_p_h__

#endif // __windows2Esystem2Ediagnostics2Etracereporting_h__
