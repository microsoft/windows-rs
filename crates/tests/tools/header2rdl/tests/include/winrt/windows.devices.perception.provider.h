
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
#ifndef __windows2Edevices2Eperception2Eprovider_h__
#define __windows2Edevices2Eperception2Eprovider_h__
#ifndef __windows2Edevices2Eperception2Eprovider_p_h__
#define __windows2Edevices2Eperception2Eprovider_p_h__


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

#if !defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Perception.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Media.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionStartFaceAuthenticationHandler;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler ABI::Windows::Devices::Perception::Provider::IPerceptionStartFaceAuthenticationHandler

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionStopFaceAuthenticationHandler;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler ABI::Windows::Devices::Perception::Provider::IPerceptionStopFaceAuthenticationHandler

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IKnownPerceptionFrameKindStatics;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics ABI::Windows::Devices::Perception::Provider::IKnownPerceptionFrameKindStatics

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionControlGroup;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup ABI::Windows::Devices::Perception::Provider::IPerceptionControlGroup

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionControlGroupFactory;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory ABI::Windows::Devices::Perception::Provider::IPerceptionControlGroupFactory

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionCorrelation;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelation

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionCorrelationFactory;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelationFactory

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionCorrelationGroup;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelationGroup

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionCorrelationGroupFactory;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelationGroupFactory

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFaceAuthenticationGroup;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroup

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFaceAuthenticationGroupFactory;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroupFactory

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFrame;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame ABI::Windows::Devices::Perception::Provider::IPerceptionFrame

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFrameProvider;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProvider

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFrameProviderInfo;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderInfo

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFrameProviderManager;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionFrameProviderManagerServiceStatics;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManagerServiceStatics

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionPropertyChangeRequest;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest ABI::Windows::Devices::Perception::Provider::IPerceptionPropertyChangeRequest

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionVideoFrameAllocator;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator ABI::Windows::Devices::Perception::Provider::IPerceptionVideoFrameAllocator

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    interface IPerceptionVideoFrameAllocatorFactory;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory ABI::Windows::Devices::Perception::Provider::IPerceptionVideoFrameAllocatorFactory

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionCorrelation;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE
#define DEF___FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c4db1093-d705-5503-8bce-68535cd42ffa"))
IIterator<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*, ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Perception.Provider.PerceptionCorrelation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*> __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_t;
#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE
#define DEF___FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ca6bf87e-1745-5cd0-aee2-59736f5a206d"))
IIterable<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*, ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Perception.Provider.PerceptionCorrelation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*> __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_t;
#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE */

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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE
#define DEF___FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("244cad66-afbe-5394-b7b7-43a61fcbfc6d"))
IVectorView<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*, ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Perception.Provider.PerceptionCorrelation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Perception::Provider::PerceptionCorrelation*> __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_t;
#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                typedef enum PerceptionFrameSourcePropertyChangeStatus : int PerceptionFrameSourcePropertyChangeStatus;
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

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
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IMemoryBuffer;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer ABI::Windows::Foundation::IMemoryBuffer

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Quaternion Quaternion;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapAlphaMode : int BitmapAlphaMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapPixelFormat : int BitmapPixelFormat;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class VideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrame ABI::Windows::Media::IVideoFrame

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionControlGroup;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionCorrelationGroup;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionFaceAuthenticationGroup;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionFrame;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionFrameProviderInfo;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionPropertyChangeRequest;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    class PerceptionVideoFrameAllocator;
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Delegate Windows.Devices.Perception.Provider.PerceptionStartFaceAuthenticationHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("74816d2a-2090-4670-8c48-ef39e7ff7c26")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionStartFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionStartFaceAuthenticationHandler : public IUnknown
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionStartFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroup* sender,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionStartFaceAuthenticationHandler = __uuidof(IPerceptionStartFaceAuthenticationHandler);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Devices.Perception.Provider.PerceptionStopFaceAuthenticationHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("387ee6aa-89cd-481e-aade-dd92f70b2ad7")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionStopFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionStopFaceAuthenticationHandler : public IUnknown
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionStopFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroup* sender
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionStopFaceAuthenticationHandler = __uuidof(IPerceptionStopFaceAuthenticationHandler);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.KnownPerceptionFrameKind
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IKnownPerceptionFrameKindStatics[] = L"Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("3ae651d6-9669-4106-9fae-4835c1b96104")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IKnownPerceptionFrameKindStatics : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Color(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Depth(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Infrared(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKnownPerceptionFrameKindStatics = __uuidof(IKnownPerceptionFrameKindStatics);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionControlGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionControlGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionControlGroup[] = L"Windows.Devices.Perception.Provider.IPerceptionControlGroup";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("172c4882-2fd9-4c4e-ba34-fdf20a73dde5")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionControlGroup : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_FrameProviderIds(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionControlGroup = __uuidof(IPerceptionControlGroup);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionControlGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionControlGroupFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("2f1af2e0-baf1-453b-bed4-cd9d4619154c")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionControlGroupFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            __FIIterable_1_HSTRING* ids,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionControlGroup** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionControlGroupFactory = __uuidof(IPerceptionControlGroupFactory);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelation[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelation";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("b4131a82-dff5-4047-8a19-3b4d805f7176")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionCorrelation : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_TargetId(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                            ABI::Windows::Foundation::Numerics::Quaternion* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionCorrelation = __uuidof(IPerceptionCorrelation);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelationFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("d4a6c425-2884-4a8f-8134-2835d7286cbf")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionCorrelationFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            HSTRING targetId,
                            ABI::Windows::Foundation::Numerics::Vector3 position,
                            ABI::Windows::Foundation::Numerics::Quaternion orientation,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelation** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionCorrelationFactory = __uuidof(IPerceptionCorrelationFactory);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelationGroup[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("752a0906-36a7-47bb-9b79-56cc6b746770")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionCorrelationGroup : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_RelativeLocations(
                            __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionCorrelationGroup = __uuidof(IPerceptionCorrelationGroup);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelationGroupFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("7dfe2088-63df-48ed-83b1-4ab829132995")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionCorrelationGroupFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* relativeLocations,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelationGroup** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionCorrelationGroupFactory = __uuidof(IPerceptionCorrelationGroupFactory);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFaceAuthenticationGroup[] = L"Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("e8019814-4a91-41b0-83a6-881a1775353e")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFaceAuthenticationGroup : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_FrameProviderIds(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFaceAuthenticationGroup = __uuidof(IPerceptionFaceAuthenticationGroup);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFaceAuthenticationGroupFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("e68a05d4-b60c-40f4-bcb9-f24d46467320")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFaceAuthenticationGroupFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            __FIIterable_1_HSTRING* ids,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionStartFaceAuthenticationHandler* startHandler,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionStopFaceAuthenticationHandler* stopHandler,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroup** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFaceAuthenticationGroupFactory = __uuidof(IPerceptionFaceAuthenticationGroupFactory);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrame[] = L"Windows.Devices.Perception.Provider.IPerceptionFrame";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("7cfe7825-54bb-4d9d-bec5-8ef66151d2ac")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFrame : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_RelativeTime(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_RelativeTime(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            ABI::Windows::Foundation::Collections::IPropertySet** value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_FrameData(
                            ABI::Windows::Foundation::IMemoryBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFrame = __uuidof(IPerceptionFrame);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProvider[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("794f7ab9-b37d-3b33-a10d-30626419ce65")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFrameProvider : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_FrameProviderInfo(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderInfo** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Available(
                            boolean* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            ABI::Windows::Foundation::Collections::IPropertySet** value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE SetProperty(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionPropertyChangeRequest* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFrameProvider = __uuidof(IPerceptionFrameProvider);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProviderInfo[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("cca959e8-797e-4e83-9b87-036a74142fc4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFrameProviderInfo : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_Id(
                            HSTRING value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                            HSTRING value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceKind(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_DeviceKind(
                            HSTRING value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_FrameKind(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_FrameKind(
                            HSTRING value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Hidden(
                            boolean* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_Hidden(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFrameProviderInfo = __uuidof(IPerceptionFrameProviderInfo);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProviderManager[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("a959ce07-ead3-33df-8ec1-b924abe019c4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("IPerceptionFrameProviderManager may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFrameProviderManager : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("IPerceptionFrameProviderManager may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE GetFrameProvider(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderInfo* frameProviderInfo,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProvider** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFrameProviderManager = __uuidof(IPerceptionFrameProviderManager);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProviderManagerServiceStatics[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("ae8386e6-cad9-4359-8f96-8eae51810526")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionFrameProviderManagerServiceStatics : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE RegisterFrameProviderInfo(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderInfo* frameProviderInfo
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE UnregisterFrameProviderInfo(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderInfo* frameProviderInfo
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE RegisterFaceAuthenticationGroup(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroup* faceAuthenticationGroup
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE UnregisterFaceAuthenticationGroup(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFaceAuthenticationGroup* faceAuthenticationGroup
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE RegisterControlGroup(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionControlGroup* controlGroup
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE UnregisterControlGroup(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionControlGroup* controlGroup
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE RegisterCorrelationGroup(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelationGroup* correlationGroup
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE UnregisterCorrelationGroup(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProviderManager* manager,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionCorrelationGroup* correlationGroup
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE UpdateAvailabilityForProvider(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProvider* provider,
                            boolean available
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE PublishFrameForProvider(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrameProvider* provider,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrame* frame
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionFrameProviderManagerServiceStatics = __uuidof(IPerceptionFrameProviderManagerServiceStatics);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionPropertyChangeRequest[] = L"Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("3c5aeb51-350b-4df8-9414-59e09815510b")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionPropertyChangeRequest : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            IInspectable** value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Perception::PerceptionFrameSourcePropertyChangeStatus* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE put_Status(
                            ABI::Windows::Devices::Perception::PerceptionFrameSourcePropertyChangeStatus value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionPropertyChangeRequest = __uuidof(IPerceptionPropertyChangeRequest);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionVideoFrameAllocator[] = L"Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("4c38a7da-fdd8-4ed4-a039-2a6f9b235038")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionVideoFrameAllocator : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE AllocateFrame(
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrame** value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE CopyFromVideoFrame(
                            ABI::Windows::Media::IVideoFrame* frame,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionFrame** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionVideoFrameAllocator = __uuidof(IPerceptionVideoFrameAllocator);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionVideoFrameAllocatorFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Perception {
                namespace Provider {
                    MIDL_INTERFACE("1a58b0e1-e91a-481e-b876-a89e2bbc6b33")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    IPerceptionVideoFrameAllocatorFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            UINT32 maxOutstandingFrameCountForWrite,
                            ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                            ABI::Windows::Foundation::Size resolution,
                            ABI::Windows::Graphics::Imaging::BitmapAlphaMode alpha,
                            ABI::Windows::Devices::Perception::Provider::IPerceptionVideoFrameAllocator** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerceptionVideoFrameAllocatorFactory = __uuidof(IPerceptionVideoFrameAllocatorFactory);
                } /* Provider */
            } /* Perception */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.KnownPerceptionFrameKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_KnownPerceptionFrameKind_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_KnownPerceptionFrameKind_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_KnownPerceptionFrameKind[] = L"Windows.Devices.Perception.Provider.KnownPerceptionFrameKind";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionControlGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionControlGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionControlGroup_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionControlGroup_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionControlGroup[] = L"Windows.Devices.Perception.Provider.PerceptionControlGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionCorrelation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionCorrelation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelation_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionCorrelation[] = L"Windows.Devices.Perception.Provider.PerceptionCorrelation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionCorrelationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelationGroup_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelationGroup_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionCorrelationGroup[] = L"Windows.Devices.Perception.Provider.PerceptionCorrelationGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFaceAuthenticationGroup_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFaceAuthenticationGroup_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFaceAuthenticationGroup[] = L"Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrame_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrame_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFrame[] = L"Windows.Devices.Perception.Provider.PerceptionFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderInfo_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFrameProviderInfo[] = L"Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderManagerService_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderManagerService_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFrameProviderManagerService[] = L"Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionPropertyChangeRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionPropertyChangeRequest_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionPropertyChangeRequest[] = L"Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionVideoFrameAllocator_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionVideoFrameAllocator_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionVideoFrameAllocator[] = L"Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory;

#endif // ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation;

typedef struct __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl;

interface __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation;

typedef struct __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        __FIIterator_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl;

interface __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation;

typedef struct __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl;

interface __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CPerception_CPerceptionFrameSourcePropertyChangeStatus __x_ABI_CWindows_CDevices_CPerception_CPerceptionFrameSourcePropertyChangeStatus;

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame __x_ABI_CWindows_CMedia_CIVideoFrame;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

/*
 *
 * Delegate Windows.Devices.Perception.Provider.PerceptionStartFaceAuthenticationHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionStartFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionStartFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* sender,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandlerVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionStartFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_Invoke(This, sender, result) \
    ((This)->lpVtbl->Invoke(This, sender, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Devices.Perception.Provider.PerceptionStopFaceAuthenticationHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionStopFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionStopFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* sender);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandlerVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionStopFaceAuthenticationHandler may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.KnownPerceptionFrameKind
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IKnownPerceptionFrameKindStatics[] = L"Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Depth)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Infrared)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_get_Depth(This, value) \
    ((This)->lpVtbl->get_Depth(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_get_Infrared(This, value) \
    ((This)->lpVtbl->get_Infrared(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIKnownPerceptionFrameKindStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionControlGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionControlGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionControlGroup[] = L"Windows.Devices.Perception.Provider.IPerceptionControlGroup";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_FrameProviderIds)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_get_FrameProviderIds(This, value) \
    ((This)->lpVtbl->get_FrameProviderIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionControlGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionControlGroupFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory* This,
        __FIIterable_1_HSTRING* ids,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_Create(This, ids, result) \
    ((This)->lpVtbl->Create(This, ids, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelation[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelation";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_TargetId)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_get_TargetId(This, value) \
    ((This)->lpVtbl->get_TargetId(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelationFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory* This,
        HSTRING targetId,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion orientation,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelation** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_Create(This, targetId, position, orientation, result) \
    ((This)->lpVtbl->Create(This, targetId, position, orientation, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelationGroup[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_RelativeLocations)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* This,
        __FIVectorView_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_get_RelativeLocations(This, value) \
    ((This)->lpVtbl->get_RelativeLocations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionCorrelationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionCorrelationGroupFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory* This,
        __FIIterable_1_Windows__CDevices__CPerception__CProvider__CPerceptionCorrelation* relativeLocations,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_Create(This, relativeLocations, result) \
    ((This)->lpVtbl->Create(This, relativeLocations, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFaceAuthenticationGroup[] = L"Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_FrameProviderIds)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_get_FrameProviderIds(This, value) \
    ((This)->lpVtbl->get_FrameProviderIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFaceAuthenticationGroupFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory* This,
        __FIIterable_1_HSTRING* ids,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStartFaceAuthenticationHandler* startHandler,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionStopFaceAuthenticationHandler* stopHandler,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_Create(This, ids, startHandler, stopHandler, result) \
    ((This)->lpVtbl->Create(This, ids, startHandler, stopHandler, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrame[] = L"Windows.Devices.Perception.Provider.IPerceptionFrame";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_RelativeTime)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_RelativeTime)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_FrameData)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* This,
        __x_ABI_CWindows_CFoundation_CIMemoryBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_get_RelativeTime(This, value) \
    ((This)->lpVtbl->get_RelativeTime(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_put_RelativeTime(This, value) \
    ((This)->lpVtbl->put_RelativeTime(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_get_FrameData(This, value) \
    ((This)->lpVtbl->get_FrameData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProvider[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_FrameProviderInfo)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Available)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* SetProperty)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_get_FrameProviderInfo(This, result) \
    ((This)->lpVtbl->get_FrameProviderInfo(This, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_get_Available(This, value) \
    ((This)->lpVtbl->get_Available(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_Start(This) \
    ((This)->lpVtbl->Start(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProvider may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_SetProperty(This, value) \
    ((This)->lpVtbl->SetProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProviderInfo[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_DeviceKind)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_DeviceKind)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_FrameKind)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_FrameKind)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Hidden)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_Hidden)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfoVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_get_DeviceKind(This, value) \
    ((This)->lpVtbl->get_DeviceKind(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_put_DeviceKind(This, value) \
    ((This)->lpVtbl->put_DeviceKind(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_get_FrameKind(This, value) \
    ((This)->lpVtbl->get_FrameKind(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_put_FrameKind(This, value) \
    ((This)->lpVtbl->put_FrameKind(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_get_Hidden(This, value) \
    ((This)->lpVtbl->get_Hidden(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_put_Hidden(This, value) \
    ((This)->lpVtbl->put_Hidden(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProviderManager[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("IPerceptionFrameProviderManager may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProviderManager may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* GetFrameProvider)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* frameProviderInfo,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("IPerceptionFrameProviderManager may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_GetFrameProvider(This, frameProviderInfo, result) \
    ((This)->lpVtbl->GetFrameProvider(This, frameProviderInfo, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionFrameProviderManagerServiceStatics[] = L"Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* RegisterFrameProviderInfo)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* frameProviderInfo);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* UnregisterFrameProviderInfo)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderInfo* frameProviderInfo);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* RegisterFaceAuthenticationGroup)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* faceAuthenticationGroup);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* UnregisterFaceAuthenticationGroup)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFaceAuthenticationGroup* faceAuthenticationGroup);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* RegisterControlGroup)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* controlGroup);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* UnregisterControlGroup)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionControlGroup* controlGroup);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* RegisterCorrelationGroup)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* correlationGroup);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* UnregisterCorrelationGroup)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManager* manager,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionCorrelationGroup* correlationGroup);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* UpdateAvailabilityForProvider)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* provider,
        boolean available);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* PublishFrameForProvider)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProvider* provider,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame* frame);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_RegisterFrameProviderInfo(This, manager, frameProviderInfo) \
    ((This)->lpVtbl->RegisterFrameProviderInfo(This, manager, frameProviderInfo))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_UnregisterFrameProviderInfo(This, manager, frameProviderInfo) \
    ((This)->lpVtbl->UnregisterFrameProviderInfo(This, manager, frameProviderInfo))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_RegisterFaceAuthenticationGroup(This, manager, faceAuthenticationGroup) \
    ((This)->lpVtbl->RegisterFaceAuthenticationGroup(This, manager, faceAuthenticationGroup))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_UnregisterFaceAuthenticationGroup(This, manager, faceAuthenticationGroup) \
    ((This)->lpVtbl->UnregisterFaceAuthenticationGroup(This, manager, faceAuthenticationGroup))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_RegisterControlGroup(This, manager, controlGroup) \
    ((This)->lpVtbl->RegisterControlGroup(This, manager, controlGroup))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_UnregisterControlGroup(This, manager, controlGroup) \
    ((This)->lpVtbl->UnregisterControlGroup(This, manager, controlGroup))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_RegisterCorrelationGroup(This, manager, correlationGroup) \
    ((This)->lpVtbl->RegisterCorrelationGroup(This, manager, correlationGroup))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_UnregisterCorrelationGroup(This, manager, correlationGroup) \
    ((This)->lpVtbl->UnregisterCorrelationGroup(This, manager, correlationGroup))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_UpdateAvailabilityForProvider(This, provider, available) \
    ((This)->lpVtbl->UpdateAvailabilityForProvider(This, provider, available))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_PublishFrameForProvider(This, provider, frame) \
    ((This)->lpVtbl->PublishFrameForProvider(This, provider, frame))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrameProviderManagerServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionPropertyChangeRequest[] = L"Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        IInspectable** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        enum __x_ABI_CWindows_CDevices_CPerception_CPerceptionFrameSourcePropertyChangeStatus* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* put_Status)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        enum __x_ABI_CWindows_CDevices_CPerception_CPerceptionFrameSourcePropertyChangeStatus value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_put_Status(This, value) \
    ((This)->lpVtbl->put_Status(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionPropertyChangeRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionVideoFrameAllocator[] = L"Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* AllocateFrame)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* CopyFromVideoFrame)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame* frame,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_AllocateFrame(This, value) \
    ((This)->lpVtbl->AllocateFrame(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_CopyFromVideoFrame(This, frame, value) \
    ((This)->lpVtbl->CopyFromVideoFrame(This, frame, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Perception_Provider_IPerceptionVideoFrameAllocatorFactory[] = L"Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory* This,
        UINT32 maxOutstandingFrameCountForWrite,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        struct __x_ABI_CWindows_CFoundation_CSize resolution,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alpha,
        __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocator** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_Create(This, maxOutstandingFrameCountForWrite, format, resolution, alpha, result) \
    ((This)->lpVtbl->Create(This, maxOutstandingFrameCountForWrite, format, resolution, alpha, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPerception_CProvider_CIPerceptionVideoFrameAllocatorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.KnownPerceptionFrameKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Perception.Provider.IKnownPerceptionFrameKindStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_KnownPerceptionFrameKind_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_KnownPerceptionFrameKind_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("KnownPerceptionFrameKind may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_KnownPerceptionFrameKind[] = L"Windows.Devices.Perception.Provider.KnownPerceptionFrameKind";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionControlGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionControlGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionControlGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionControlGroup_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionControlGroup_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionControlGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionControlGroup[] = L"Windows.Devices.Perception.Provider.PerceptionControlGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionCorrelation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionCorrelationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionCorrelation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelation_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelation may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionCorrelation[] = L"Windows.Devices.Perception.Provider.PerceptionCorrelation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionCorrelationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionCorrelationGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionCorrelationGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelationGroup_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionCorrelationGroup_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionCorrelationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionCorrelationGroup[] = L"Windows.Devices.Perception.Provider.PerceptionCorrelationGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionFaceAuthenticationGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFaceAuthenticationGroup_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFaceAuthenticationGroup_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFaceAuthenticationGroup may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFaceAuthenticationGroup[] = L"Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrame_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrame_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrame may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFrame[] = L"Windows.Devices.Perception.Provider.PerceptionFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionFrameProviderInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderInfo_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrameProviderInfo may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFrameProviderInfo[] = L"Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Perception.Provider.IPerceptionFrameProviderManagerServiceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderManagerService_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionFrameProviderManagerService_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionFrameProviderManagerService may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionFrameProviderManagerService[] = L"Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionPropertyChangeRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionPropertyChangeRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionPropertyChangeRequest_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionPropertyChangeRequest may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionPropertyChangeRequest[] = L"Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocatorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Perception.Provider.IPerceptionVideoFrameAllocator ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionVideoFrameAllocator_DEFINED
#define RUNTIMECLASS_Windows_Devices_Perception_Provider_PerceptionVideoFrameAllocator_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
DEPRECATED("PerceptionVideoFrameAllocator may be unavailable after Windows Creator Update. Please refer to AVStream on MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Perception_Provider_PerceptionVideoFrameAllocator[] = L"Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator";
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
#endif // __windows2Edevices2Eperception2Eprovider_p_h__

#endif // __windows2Edevices2Eperception2Eprovider_h__
