
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
#ifndef __windows2Eperception2Espatial2Esurfaces_h__
#define __windows2Eperception2Espatial2Esurfaces_h__
#ifndef __windows2Eperception2Espatial2Esurfaces_p_h__
#define __windows2Eperception2Espatial2Esurfaces_p_h__


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
#include "Windows.Foundation.Numerics.h"
#include "Windows.Graphics.DirectX.h"
#include "Windows.Perception.Spatial.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceInfo;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceInfo

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceMesh;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMesh

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceMeshBuffer;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshBuffer

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceMeshOptions;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshOptions

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceMeshOptionsStatics;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshOptionsStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceObserver;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceObserver

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceObserverStatics;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceObserverStatics

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    interface ISpatialSurfaceObserverStatics2;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2 ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceObserverStatics2

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef enum SpatialPerceptionAccessStatus : int SpatialPerceptionAccessStatus;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b425d126-1069-563f-a863-44a30a8f071d"))
IAsyncOperation<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.Spatial.SpatialPerceptionAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ced54c8-7689-525a-80e1-956a9d85cd83"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.Spatial.SpatialPerceptionAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Perception::Spatial::SpatialPerceptionAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    class SpatialSurfaceMesh;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_USE
#define DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f5938fad-a8a1-5f7e-9440-bdb781ad26b6"))
IAsyncOperation<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceMesh*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceMesh*, ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMesh*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceMesh*> __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_t;
#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4680f7f6-44c5-5fc6-8d51-d6962915fa23"))
IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceMesh*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceMesh*, ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMesh*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceMesh*> __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    class SpatialSurfaceInfo;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#define DEF___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a6bdf94a-2697-5ff2-89dc-a17cecdcda6c"))
IKeyValuePair<GUID, ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceInfo*> : IKeyValuePair_impl<GUID, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceInfo*, ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<Guid, Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<GUID, ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceInfo*> __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t;
#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6d328390-f279-5f39-9682-bba0cd81489b"))
IIterator<__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo*> : IIterator_impl<__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<Guid, Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo*> __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t;
#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("868757d1-be21-51d9-8dee-a958b9deec71"))
IIterable<__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo*> : IIterable_impl<__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<Guid, Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo*> __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t;
#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                typedef enum DirectXPixelFormat : int DirectXPixelFormat;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#define DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ea016190-ac80-5840-8f58-ff434c7b2907"))
IIterator<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> : IIterator_impl<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.DirectX.DirectXPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t;
#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#define DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3908f2c6-1aee-5129-b9a6-2a6e01d9507e"))
IIterable<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> : IIterable_impl<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.DirectX.DirectXPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t;
#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialBoundingVolume;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialBoundingVolume;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume ABI::Windows::Perception::Spatial::ISpatialBoundingVolume

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_USE
#define DEF___FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb8385c5-0775-5415-8f76-327e6e388ac5"))
IIterator<ABI::Windows::Perception::Spatial::SpatialBoundingVolume*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialBoundingVolume*, ABI::Windows::Perception::Spatial::ISpatialBoundingVolume*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Perception.Spatial.SpatialBoundingVolume>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Perception::Spatial::SpatialBoundingVolume*> __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_t;
#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_USE
#define DEF___FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("89e8f1ee-3a2a-5b69-a786-cddcf7456a3a"))
IIterable<ABI::Windows::Perception::Spatial::SpatialBoundingVolume*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::SpatialBoundingVolume*, ABI::Windows::Perception::Spatial::ISpatialBoundingVolume*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Perception.Spatial.SpatialBoundingVolume>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Perception::Spatial::SpatialBoundingVolume*> __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_t;
#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#define DEF___FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eaa722b9-2859-593d-bb66-0c538e415e71"))
IMapView<GUID, ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceInfo*> : IMapView_impl<GUID, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceInfo*, ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<Guid, Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<GUID, ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceInfo*> __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t;
#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo ABI::Windows::Foundation::Collections::__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1edda1c2-0f6e-516c-80b8-7687dcd1280e"))
IVectorView<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> : IVectorView_impl<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.DirectX.DirectXPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t;
#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingOrientedBox SpatialBoundingOrientedBox;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_USE
#define DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("09f88309-9f81-5207-bdb2-abef926db18f"))
IReference<struct ABI::Windows::Perception::Spatial::SpatialBoundingOrientedBox> : IReference_impl<struct ABI::Windows::Perception::Spatial::SpatialBoundingOrientedBox>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Perception.Spatial.SpatialBoundingOrientedBox>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Perception::Spatial::SpatialBoundingOrientedBox> __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_t;
#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox ABI::Windows::Foundation::__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    class SpatialSurfaceObserver;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8b31274a-7693-52be-9014-b0f5f65a3539"))
ITypedEventHandler<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceObserver*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceObserver*, ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceObserver*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Perception::Spatial::Surfaces::SpatialSurfaceObserver*, IInspectable*> __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            namespace Numerics {
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    class SpatialSurfaceMeshBuffer;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    class SpatialSurfaceMeshOptions;
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceInfo[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("f8e9ebe7-39b7-3962-bb03-57f56e1fb0a1")
                    ISpatialSurfaceInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UpdateTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetBounds(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryComputeLatestMeshAsync(
                            DOUBLE maxTrianglesPerCubicMeter,
                            __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryComputeLatestMeshWithOptionsAsync(
                            DOUBLE maxTrianglesPerCubicMeter,
                            ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshOptions* options,
                            __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceInfo = __uuidof(ISpatialSurfaceInfo);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMesh[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("108f57d9-df0d-3950-a0fd-f972c77c27b4")
                    ISpatialSurfaceMesh : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SurfaceInfo(
                            ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TriangleIndices(
                            ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VertexPositions(
                            ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VertexPositionScale(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VertexNormals(
                            ABI::Windows::Perception::Spatial::Surfaces::ISpatialSurfaceMeshBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceMesh = __uuidof(ISpatialSurfaceMesh);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMeshBuffer[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("93cf59e0-871f-33f8-98b2-03d101458f6f")
                    ISpatialSurfaceMeshBuffer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Format(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Stride(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementCount(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceMeshBuffer = __uuidof(ISpatialSurfaceMeshBuffer);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMeshOptions[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("d2759f89-3572-3d2d-a10d-5fee9394aa37")
                    ISpatialSurfaceMeshOptions : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_VertexPositionFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_VertexPositionFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TriangleIndexFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TriangleIndexFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VertexNormalFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_VertexNormalFormat(
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncludeVertexNormals(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncludeVertexNormals(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceMeshOptions = __uuidof(ISpatialSurfaceMeshOptions);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMeshOptionsStatics[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("9b340abf-9781-4505-8935-013575caae5e")
                    ISpatialSurfaceMeshOptionsStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedVertexPositionFormats(
                            __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedTriangleIndexFormats(
                            __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedVertexNormalFormats(
                            __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceMeshOptionsStatics = __uuidof(ISpatialSurfaceMeshOptionsStatics);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceObserver[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("10b69819-ddca-3483-ac3a-748fe8c86df5")
                    ISpatialSurfaceObserver : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetObservedSurfaces(
                            __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetBoundingVolume(
                            ABI::Windows::Perception::Spatial::ISpatialBoundingVolume* bounds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetBoundingVolumes(
                            __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* bounds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ObservedSurfacesChanged(
                            __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ObservedSurfacesChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceObserver = __uuidof(ISpatialSurfaceObserver);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceObserverStatics[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("165951ed-2108-4168-9175-87e027bc9285")
                    ISpatialSurfaceObserverStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                            __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceObserverStatics = __uuidof(ISpatialSurfaceObserverStatics);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceObserverStatics2[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                namespace Surfaces {
                    MIDL_INTERFACE("0f534261-c55d-4e6b-a895-a19de69a42e3")
                    ISpatialSurfaceObserverStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsSupported(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpatialSurfaceObserverStatics2 = __uuidof(ISpatialSurfaceObserverStatics2);
                } /* Surfaces */
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceInfo_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceInfo[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMesh_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMesh_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMesh[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshBuffer_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshBuffer[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshOptions_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshOptions[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceObserver_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceObserver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceObserver[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2 __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        enum __x_ABI_CWindows_CPerception_CSpatial_CSpatialPerceptionAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh;

typedef struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMeshVtbl;

interface __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMeshVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

typedef struct __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo** result);

    END_INTERFACE
} __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl;

interface __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo
{
    CONST_VTBL struct __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

typedef struct __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl;

interface __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

typedef struct __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        __FIIterator_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl;

interface __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

typedef struct __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl;

interface __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

typedef struct __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl;

interface __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume;

typedef struct __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolumeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolumeVtbl;

interface __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume
{
    CONST_VTBL struct __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolumeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume;

typedef struct __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolumeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* This,
        __FIIterator_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume** result);

    END_INTERFACE
} __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolumeVtbl;

interface __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume
{
    CONST_VTBL struct __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolumeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef interface __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__)
#define ____FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__

typedef interface __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo;

typedef struct __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        GUID key,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        GUID key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo* This,
        __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** first,
        __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** second);

    END_INTERFACE
} __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl;

interface __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo
{
    CONST_VTBL struct __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

typedef struct __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32 index,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl;

interface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox;

typedef struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox* This,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingOrientedBox* result);

    END_INTERFACE
} __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBoxVtbl;

interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox
{
    CONST_VTBL struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceInfo[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateTime)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* TryGetBounds)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingOrientedBox** value);
    HRESULT (STDMETHODCALLTYPE* TryComputeLatestMeshAsync)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        DOUBLE maxTrianglesPerCubicMeter,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh** value);
    HRESULT (STDMETHODCALLTYPE* TryComputeLatestMeshWithOptionsAsync)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo* This,
        DOUBLE maxTrianglesPerCubicMeter,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* options,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceMesh** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfoVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_get_UpdateTime(This, value) \
    ((This)->lpVtbl->get_UpdateTime(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_TryGetBounds(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetBounds(This, coordinateSystem, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_TryComputeLatestMeshAsync(This, maxTrianglesPerCubicMeter, value) \
    ((This)->lpVtbl->TryComputeLatestMeshAsync(This, maxTrianglesPerCubicMeter, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_TryComputeLatestMeshWithOptionsAsync(This, maxTrianglesPerCubicMeter, options, value) \
    ((This)->lpVtbl->TryComputeLatestMeshWithOptionsAsync(This, maxTrianglesPerCubicMeter, options, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMesh[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SurfaceInfo)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);
    HRESULT (STDMETHODCALLTYPE* get_TriangleIndices)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_VertexPositions)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_VertexPositionScale)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_VertexNormals)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh* This,
        __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_get_SurfaceInfo(This, value) \
    ((This)->lpVtbl->get_SurfaceInfo(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_get_TriangleIndices(This, value) \
    ((This)->lpVtbl->get_TriangleIndices(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_get_VertexPositions(This, value) \
    ((This)->lpVtbl->get_VertexPositions(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_get_VertexPositionScale(This, value) \
    ((This)->lpVtbl->get_VertexPositionScale(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_get_VertexNormals(This, value) \
    ((This)->lpVtbl->get_VertexNormals(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMesh_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMeshBuffer[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Format)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_Stride)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ElementCount)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBufferVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_get_Format(This, value) \
    ((This)->lpVtbl->get_Format(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_get_Stride(This, value) \
    ((This)->lpVtbl->get_Stride(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_get_ElementCount(This, value) \
    ((This)->lpVtbl->get_ElementCount(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMeshOptions[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VertexPositionFormat)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_VertexPositionFormat)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value);
    HRESULT (STDMETHODCALLTYPE* get_TriangleIndexFormat)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_TriangleIndexFormat)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value);
    HRESULT (STDMETHODCALLTYPE* get_VertexNormalFormat)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_VertexNormalFormat)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeVertexNormals)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeVertexNormals)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_get_VertexPositionFormat(This, value) \
    ((This)->lpVtbl->get_VertexPositionFormat(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_put_VertexPositionFormat(This, value) \
    ((This)->lpVtbl->put_VertexPositionFormat(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_get_TriangleIndexFormat(This, value) \
    ((This)->lpVtbl->get_TriangleIndexFormat(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_put_TriangleIndexFormat(This, value) \
    ((This)->lpVtbl->put_TriangleIndexFormat(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_get_VertexNormalFormat(This, value) \
    ((This)->lpVtbl->get_VertexNormalFormat(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_put_VertexNormalFormat(This, value) \
    ((This)->lpVtbl->put_VertexNormalFormat(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_get_IncludeVertexNormals(This, value) \
    ((This)->lpVtbl->get_IncludeVertexNormals(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_put_IncludeVertexNormals(This, value) \
    ((This)->lpVtbl->put_IncludeVertexNormals(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceMeshOptionsStatics[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedVertexPositionFormats)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedTriangleIndexFormats)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedVertexNormalFormats)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics* This,
        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_get_SupportedVertexPositionFormats(This, value) \
    ((This)->lpVtbl->get_SupportedVertexPositionFormats(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_get_SupportedTriangleIndexFormats(This, value) \
    ((This)->lpVtbl->get_SupportedTriangleIndexFormats(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_get_SupportedVertexNormalFormats(This, value) \
    ((This)->lpVtbl->get_SupportedVertexNormalFormats(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceMeshOptionsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceObserver[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetObservedSurfaces)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        __FIMapView_2_GUID_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceInfo** value);
    HRESULT (STDMETHODCALLTYPE* SetBoundingVolume)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialBoundingVolume* bounds);
    HRESULT (STDMETHODCALLTYPE* SetBoundingVolumes)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        __FIIterable_1_Windows__CPerception__CSpatial__CSpatialBoundingVolume* bounds);
    HRESULT (STDMETHODCALLTYPE* add_ObservedSurfacesChanged)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        __FITypedEventHandler_2_Windows__CPerception__CSpatial__CSurfaces__CSpatialSurfaceObserver_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ObservedSurfacesChanged)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_GetObservedSurfaces(This, value) \
    ((This)->lpVtbl->GetObservedSurfaces(This, value))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_SetBoundingVolume(This, bounds) \
    ((This)->lpVtbl->SetBoundingVolume(This, bounds))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_SetBoundingVolumes(This, bounds) \
    ((This)->lpVtbl->SetBoundingVolumes(This, bounds))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_add_ObservedSurfacesChanged(This, handler, token) \
    ((This)->lpVtbl->add_ObservedSurfacesChanged(This, handler, token))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_remove_ObservedSurfacesChanged(This, token) \
    ((This)->lpVtbl->remove_ObservedSurfacesChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceObserverStatics[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics* This,
        __FIAsyncOperation_1_Windows__CPerception__CSpatial__CSpatialPerceptionAccessStatus** result);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_RequestAccessAsync(This, result) \
    ((This)->lpVtbl->RequestAccessAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Spatial_Surfaces_ISpatialSurfaceObserverStatics2[] = L"Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2";
typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2Vtbl;

interface __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_IsSupported(This, value) \
    ((This)->lpVtbl->IsSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2;
#endif /* !defined(____x_ABI_CWindows_CPerception_CSpatial_CSurfaces_CISpatialSurfaceObserverStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceInfo_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceInfo[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMesh ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMesh_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMesh_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMesh[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshBuffer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshBuffer_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshBuffer[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptionsStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceMeshOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshOptions_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceMeshOptions[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserverStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.Spatial.Surfaces.ISpatialSurfaceObserver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceObserver_DEFINED
#define RUNTIMECLASS_Windows_Perception_Spatial_Surfaces_SpatialSurfaceObserver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Spatial_Surfaces_SpatialSurfaceObserver[] = L"Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eperception2Espatial2Esurfaces_p_h__

#endif // __windows2Eperception2Espatial2Esurfaces_h__
