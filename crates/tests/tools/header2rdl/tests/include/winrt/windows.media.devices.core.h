
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
#ifndef __windows2Emedia2Edevices2Ecore_h__
#define __windows2Emedia2Edevices2Ecore_h__
#ifndef __windows2Emedia2Edevices2Ecore_p_h__
#define __windows2Emedia2Edevices2Ecore_p_h__


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
#include "Windows.Media.MediaProperties.h"
#include "Windows.Perception.Spatial.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface ICameraIntrinsics;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics ABI::Windows::Media::Devices::Core::ICameraIntrinsics

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface ICameraIntrinsics2;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2 ABI::Windows::Media::Devices::Core::ICameraIntrinsics2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface ICameraIntrinsicsFactory;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory ABI::Windows::Media::Devices::Core::ICameraIntrinsicsFactory

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IDepthCorrelatedCoordinateMapper;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper ABI::Windows::Media::Devices::Core::IDepthCorrelatedCoordinateMapper

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameControlCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities ABI::Windows::Media::Devices::Core::IFrameControlCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameControlCapabilities2;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2 ABI::Windows::Media::Devices::Core::IFrameControlCapabilities2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameController;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController ABI::Windows::Media::Devices::Core::IFrameController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameController2;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2 ABI::Windows::Media::Devices::Core::IFrameController2

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameExposureCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities ABI::Windows::Media::Devices::Core::IFrameExposureCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameExposureCompensationCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities ABI::Windows::Media::Devices::Core::IFrameExposureCompensationCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameExposureCompensationControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl ABI::Windows::Media::Devices::Core::IFrameExposureCompensationControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameExposureControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl ABI::Windows::Media::Devices::Core::IFrameExposureControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameFlashCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities ABI::Windows::Media::Devices::Core::IFrameFlashCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameFlashControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl ABI::Windows::Media::Devices::Core::IFrameFlashControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameFocusCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities ABI::Windows::Media::Devices::Core::IFrameFocusCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameFocusControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl ABI::Windows::Media::Devices::Core::IFrameFocusControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameIsoSpeedCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities ABI::Windows::Media::Devices::Core::IFrameIsoSpeedCapabilities

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IFrameIsoSpeedControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl ABI::Windows::Media::Devices::Core::IFrameIsoSpeedControl

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IVariablePhotoSequenceController;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController ABI::Windows::Media::Devices::Core::IVariablePhotoSequenceController

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameController;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#define DEF___FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1440dc88-63ff-5a01-bb93-390c76742488"))
IIterator<ABI::Windows::Media::Devices::Core::FrameController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::Core::FrameController*, ABI::Windows::Media::Devices::Core::IFrameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Devices.Core.FrameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Devices::Core::FrameController*> __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_t;
#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#define DEF___FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bd8eeadc-2dd9-5ad8-ac5d-f3b13b94b9c2"))
IIterable<ABI::Windows::Media::Devices::Core::FrameController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::Core::FrameController*, ABI::Windows::Media::Devices::Core::IFrameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Devices.Core.FrameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Devices::Core::FrameController*> __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_t;
#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#define DEF___FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fcd6a2a3-b6ff-5572-912b-99ebaf95165d"))
IVectorView<ABI::Windows::Media::Devices::Core::FrameController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::Core::FrameController*, ABI::Windows::Media::Devices::Core::IFrameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Devices.Core.FrameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Devices::Core::FrameController*> __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_t;
#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#define DEF___FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c8f8276-b89a-5093-a1ed-af49dfb72a89"))
IVector<ABI::Windows::Media::Devices::Core::FrameController*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Devices::Core::FrameController*, ABI::Windows::Media::Devices::Core::IFrameController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Devices.Core.FrameController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Devices::Core::FrameController*> __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_t;
#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_boolean_USE
#define DEF___FIReference_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c00fd60-2950-5939-a21a-2d12c5a01b8a"))
IReference<bool> : IReference_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<bool> __FIReference_1_boolean_t;
#define __FIReference_1_boolean ABI::Windows::Foundation::__FIReference_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_boolean_USE */



#ifndef DEF___FIReference_1_float_USE
#define DEF___FIReference_1_float_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("719cc2ba-3e76-5def-9f1a-38d85a145ea8"))
IReference<float> : IReference_impl<float>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Single>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<float> __FIReference_1_float_t;
#define __FIReference_1_float ABI::Windows::Foundation::__FIReference_1_float_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_float_USE */



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
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#define DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("604d0c4c-91de-5c2a-935f-362f13eaf800"))
IReference<struct ABI::Windows::Foundation::TimeSpan> : IReference_impl<struct ABI::Windows::Foundation::TimeSpan>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.TimeSpan>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::TimeSpan> __FIReference_1_Windows__CFoundation__CTimeSpan_t;
#define __FIReference_1_Windows__CFoundation__CTimeSpan ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CTimeSpan_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

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
                typedef struct Matrix4x4 Matrix4x4;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector2 Vector2;
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
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties ABI::Windows::Media::MediaProperties::IMediaEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaRatio;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaRatio;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio ABI::Windows::Media::MediaProperties::IMediaRatio

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    typedef enum FrameFlashMode : int FrameFlashMode;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class CameraIntrinsics;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameControlCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameExposureCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameExposureCompensationCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameExposureCompensationControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameExposureControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameFlashCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameFlashControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameFocusCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameFocusControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameIsoSpeedCapabilities;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class FrameIsoSpeedControl;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Devices.Core.FrameFlashMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    enum FrameFlashMode : int
                    {
                        FrameFlashMode_Disable = 0,
                        FrameFlashMode_Enable = 1,
                        FrameFlashMode_Global = 2,
                    };
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.ICameraIntrinsics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.CameraIntrinsics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_ICameraIntrinsics[] = L"Windows.Media.Devices.Core.ICameraIntrinsics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("0aa6ed32-6589-49da-afde-594270ca0aac")
                    ICameraIntrinsics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FocalLength(
                            ABI::Windows::Foundation::Numerics::Vector2* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PrincipalPoint(
                            ABI::Windows::Foundation::Numerics::Vector2* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RadialDistortion(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TangentialDistortion(
                            ABI::Windows::Foundation::Numerics::Vector2* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ImageWidth(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ImageHeight(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProjectOntoFrame(
                            ABI::Windows::Foundation::Numerics::Vector3 coordinate,
                            ABI::Windows::Foundation::Point* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnprojectAtUnitDepth(
                            ABI::Windows::Foundation::Point pixelCoordinate,
                            ABI::Windows::Foundation::Numerics::Vector2* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProjectManyOntoFrame(
                            UINT32 coordinatesLength,
                            ABI::Windows::Foundation::Numerics::Vector3* coordinates,
                            UINT32 resultsLength,
                            ABI::Windows::Foundation::Point* results
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnprojectPixelsAtUnitDepth(
                            UINT32 pixelCoordinatesLength,
                            ABI::Windows::Foundation::Point* pixelCoordinates,
                            UINT32 resultsLength,
                            ABI::Windows::Foundation::Numerics::Vector2* results
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICameraIntrinsics = __uuidof(ICameraIntrinsics);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.ICameraIntrinsics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.CameraIntrinsics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_ICameraIntrinsics2[] = L"Windows.Media.Devices.Core.ICameraIntrinsics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("0cdaa447-0798-4b4d-839f-c5ec414db27a")
                    ICameraIntrinsics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UndistortedProjectionTransform(
                            ABI::Windows::Foundation::Numerics::Matrix4x4* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DistortPoint(
                            ABI::Windows::Foundation::Point input,
                            ABI::Windows::Foundation::Point* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DistortPoints(
                            UINT32 inputsLength,
                            ABI::Windows::Foundation::Point* inputs,
                            UINT32 resultsLength,
                            ABI::Windows::Foundation::Point* results
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UndistortPoint(
                            ABI::Windows::Foundation::Point input,
                            ABI::Windows::Foundation::Point* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UndistortPoints(
                            UINT32 inputsLength,
                            ABI::Windows::Foundation::Point* inputs,
                            UINT32 resultsLength,
                            ABI::Windows::Foundation::Point* results
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICameraIntrinsics2 = __uuidof(ICameraIntrinsics2);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Devices.Core.ICameraIntrinsicsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.CameraIntrinsics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_ICameraIntrinsicsFactory[] = L"Windows.Media.Devices.Core.ICameraIntrinsicsFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("c0ddc486-2132-4a34-a659-9bfe2a055712")
                    ICameraIntrinsicsFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::Foundation::Numerics::Vector2 focalLength,
                            ABI::Windows::Foundation::Numerics::Vector2 principalPoint,
                            ABI::Windows::Foundation::Numerics::Vector3 radialDistortion,
                            ABI::Windows::Foundation::Numerics::Vector2 tangentialDistortion,
                            UINT32 imageWidth,
                            UINT32 imageHeight,
                            ABI::Windows::Media::Devices::Core::ICameraIntrinsics** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICameraIntrinsicsFactory = __uuidof(ICameraIntrinsicsFactory);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IDepthCorrelatedCoordinateMapper[] = L"Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("f95d89fb-8af0-4cb0-926d-696866e5046a")
                    IDepthCorrelatedCoordinateMapper : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE UnprojectPoint(
                            ABI::Windows::Foundation::Point sourcePoint,
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* targetCoordinateSystem,
                            ABI::Windows::Foundation::Numerics::Vector3* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnprojectPoints(
                            UINT32 sourcePointsLength,
                            ABI::Windows::Foundation::Point* sourcePoints,
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* targetCoordinateSystem,
                            UINT32 resultsLength,
                            ABI::Windows::Foundation::Numerics::Vector3* results
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE MapPoint(
                            ABI::Windows::Foundation::Point sourcePoint,
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* targetCoordinateSystem,
                            ABI::Windows::Media::Devices::Core::ICameraIntrinsics* targetCameraIntrinsics,
                            ABI::Windows::Foundation::Point* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE MapPoints(
                            UINT32 sourcePointsLength,
                            ABI::Windows::Foundation::Point* sourcePoints,
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* targetCoordinateSystem,
                            ABI::Windows::Media::Devices::Core::ICameraIntrinsics* targetCameraIntrinsics,
                            UINT32 resultsLength,
                            ABI::Windows::Foundation::Point* results
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDepthCorrelatedCoordinateMapper = __uuidof(IDepthCorrelatedCoordinateMapper);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameControlCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameControlCapabilities[] = L"Windows.Media.Devices.Core.IFrameControlCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("a8ffae60-4e9e-4377-a789-e24c4ae7e544")
                    IFrameControlCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Exposure(
                            ABI::Windows::Media::Devices::Core::IFrameExposureCapabilities** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExposureCompensation(
                            ABI::Windows::Media::Devices::Core::IFrameExposureCompensationCapabilities** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsoSpeed(
                            ABI::Windows::Media::Devices::Core::IFrameIsoSpeedCapabilities** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Focus(
                            ABI::Windows::Media::Devices::Core::IFrameFocusCapabilities** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhotoConfirmationSupported(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameControlCapabilities = __uuidof(IFrameControlCapabilities);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameControlCapabilities2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameControlCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameControlCapabilities2[] = L"Windows.Media.Devices.Core.IFrameControlCapabilities2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("ce9b0464-4730-440f-bd3e-efe8a8f230a8")
                    IFrameControlCapabilities2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Flash(
                            ABI::Windows::Media::Devices::Core::IFrameFlashCapabilities** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameControlCapabilities2 = __uuidof(IFrameControlCapabilities2);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameController[] = L"Windows.Media.Devices.Core.IFrameController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("c16459d9-baef-4052-9177-48aff2af7522")
                    IFrameController : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExposureControl(
                            ABI::Windows::Media::Devices::Core::IFrameExposureControl** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExposureCompensationControl(
                            ABI::Windows::Media::Devices::Core::IFrameExposureCompensationControl** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsoSpeedControl(
                            ABI::Windows::Media::Devices::Core::IFrameIsoSpeedControl** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FocusControl(
                            ABI::Windows::Media::Devices::Core::IFrameFocusControl** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhotoConfirmationEnabled(
                            __FIReference_1_boolean** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PhotoConfirmationEnabled(
                            __FIReference_1_boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameController = __uuidof(IFrameController);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameController2[] = L"Windows.Media.Devices.Core.IFrameController2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("00d3bc75-d87c-485b-8a09-5c358568b427")
                    IFrameController2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FlashControl(
                            ABI::Windows::Media::Devices::Core::IFrameFlashControl** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameController2 = __uuidof(IFrameController2);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureCapabilities[] = L"Windows.Media.Devices.Core.IFrameExposureCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("bdbe9ce3-3985-4e72-97c2-0590d61307a1")
                    IFrameExposureCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Min(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Max(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Step(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameExposureCapabilities = __uuidof(IFrameExposureCapabilities);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureCompensationCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureCompensationCapabilities[] = L"Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("b988a823-8065-41ee-b04f-722265954500")
                    IFrameExposureCompensationCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Min(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Max(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Step(
                            FLOAT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameExposureCompensationCapabilities = __uuidof(IFrameExposureCompensationCapabilities);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureCompensationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureCompensationControl[] = L"Windows.Media.Devices.Core.IFrameExposureCompensationControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("e95896c9-f7f9-48ca-8591-a26531cb1578")
                    IFrameExposureCompensationControl : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIReference_1_float** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            __FIReference_1_float* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameExposureCompensationControl = __uuidof(IFrameExposureCompensationControl);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureControl[] = L"Windows.Media.Devices.Core.IFrameExposureControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("b1605a61-ffaf-4752-b621-f5b6f117f432")
                    IFrameExposureControl : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Auto(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Auto(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            __FIReference_1_Windows__CFoundation__CTimeSpan* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameExposureControl = __uuidof(IFrameExposureControl);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFlashCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFlashCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFlashCapabilities[] = L"Windows.Media.Devices.Core.IFrameFlashCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("bb9341a2-5ebe-4f62-8223-0e2b05bfbbd0")
                    IFrameFlashCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RedEyeReductionSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PowerSupported(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameFlashCapabilities = __uuidof(IFrameFlashCapabilities);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFlashControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFlashControl[] = L"Windows.Media.Devices.Core.IFrameFlashControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("75d5f6c7-bd45-4fab-9375-45ac04b332c2")
                    IFrameFlashControl : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Mode(
                            ABI::Windows::Media::Devices::Core::FrameFlashMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Mode(
                            ABI::Windows::Media::Devices::Core::FrameFlashMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Auto(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Auto(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RedEyeReduction(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RedEyeReduction(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PowerPercent(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PowerPercent(
                            FLOAT value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameFlashControl = __uuidof(IFrameFlashControl);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFocusCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFocusCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFocusCapabilities[] = L"Windows.Media.Devices.Core.IFrameFocusCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("7b25cd58-01c0-4065-9c40-c1a721425c1a")
                    IFrameFocusCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Min(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Max(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Step(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameFocusCapabilities = __uuidof(IFrameFocusCapabilities);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFocusControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFocusControl[] = L"Windows.Media.Devices.Core.IFrameFocusControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("272df1d0-d912-4214-a67b-e38a8d48d8c6")
                    IFrameFocusControl : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            __FIReference_1_UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameFocusControl = __uuidof(IFrameFocusControl);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameIsoSpeedCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameIsoSpeedCapabilities[] = L"Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("16bdff61-6df6-4ac9-b92a-9f6ecd1ad2fa")
                    IFrameIsoSpeedCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Min(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Max(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Step(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameIsoSpeedCapabilities = __uuidof(IFrameIsoSpeedCapabilities);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameIsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameIsoSpeedControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameIsoSpeedControl[] = L"Windows.Media.Devices.Core.IFrameIsoSpeedControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("1a03efed-786a-4c75-a557-7ab9a85f588c")
                    IFrameIsoSpeedControl : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Auto(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Auto(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            __FIReference_1_UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFrameIsoSpeedControl = __uuidof(IFrameIsoSpeedControl);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IVariablePhotoSequenceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.VariablePhotoSequenceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IVariablePhotoSequenceController[] = L"Windows.Media.Devices.Core.IVariablePhotoSequenceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    MIDL_INTERFACE("7fbff880-ed8c-43fd-a7c3-b35809e4229a")
                    IVariablePhotoSequenceController : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxPhotosPerSecond(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhotosPerSecondLimit(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PhotosPerSecondLimit(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHighestConcurrentFrameRate(
                            ABI::Windows::Media::MediaProperties::IMediaEncodingProperties* captureProperties,
                            ABI::Windows::Media::MediaProperties::IMediaRatio** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentFrameRate(
                            ABI::Windows::Media::MediaProperties::IMediaRatio** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FrameCapabilities(
                            ABI::Windows::Media::Devices::Core::IFrameControlCapabilities** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredFrameControllers(
                            __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController** items
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVariablePhotoSequenceController = __uuidof(IVariablePhotoSequenceController);
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.CameraIntrinsics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Devices.Core.ICameraIntrinsicsFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.ICameraIntrinsics ** Default Interface **
 *    Windows.Media.Devices.Core.ICameraIntrinsics2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_CameraIntrinsics_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_CameraIntrinsics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_CameraIntrinsics[] = L"Windows.Media.Devices.Core.CameraIntrinsics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_DepthCorrelatedCoordinateMapper_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_DepthCorrelatedCoordinateMapper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_DepthCorrelatedCoordinateMapper[] = L"Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Devices.Core.FrameControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameControlCapabilities ** Default Interface **
 *    Windows.Media.Devices.Core.IFrameControlCapabilities2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameControlCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameControlCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameControlCapabilities[] = L"Windows.Media.Devices.Core.FrameControlCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameController ** Default Interface **
 *    Windows.Media.Devices.Core.IFrameController2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameController[] = L"Windows.Media.Devices.Core.FrameController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureCapabilities[] = L"Windows.Media.Devices.Core.FrameExposureCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureCompensationCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureCompensationCapabilities[] = L"Windows.Media.Devices.Core.FrameExposureCompensationCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureCompensationControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureCompensationControl[] = L"Windows.Media.Devices.Core.FrameExposureCompensationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureControl[] = L"Windows.Media.Devices.Core.FrameExposureControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFlashCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFlashCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFlashCapabilities[] = L"Windows.Media.Devices.Core.FrameFlashCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFlashControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFlashControl[] = L"Windows.Media.Devices.Core.FrameFlashControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFocusCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFocusCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFocusCapabilities[] = L"Windows.Media.Devices.Core.FrameFocusCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFocusControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFocusControl[] = L"Windows.Media.Devices.Core.FrameFocusControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameIsoSpeedCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameIsoSpeedCapabilities[] = L"Windows.Media.Devices.Core.FrameIsoSpeedCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameIsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameIsoSpeedControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameIsoSpeedControl[] = L"Windows.Media.Devices.Core.FrameIsoSpeedControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.VariablePhotoSequenceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IVariablePhotoSequenceController ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_VariablePhotoSequenceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_VariablePhotoSequenceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_VariablePhotoSequenceController[] = L"Windows.Media.Devices.Core.VariablePhotoSequenceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2 __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2 __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2 __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController;

typedef struct __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl;

interface __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController;

typedef struct __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        __FIIterator_1_Windows__CMedia__CDevices__CCore__CFrameController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl;

interface __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController;

typedef struct __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl;

interface __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController;

typedef struct __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        __FIVectorView_1_Windows__CMedia__CDevices__CCore__CFrameController** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl;

interface __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_boolean_INTERFACE_DEFINED__)
#define ____FIReference_1_boolean_INTERFACE_DEFINED__

typedef interface __FIReference_1_boolean __FIReference_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_boolean;

typedef struct __FIReference_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIReference_1_booleanVtbl;

interface __FIReference_1_boolean
{
    CONST_VTBL struct __FIReference_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_boolean_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIReference_1_float_INTERFACE_DEFINED__)
#define ____FIReference_1_float_INTERFACE_DEFINED__

typedef interface __FIReference_1_float __FIReference_1_float;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_float;

typedef struct __FIReference_1_floatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_float* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_float* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_float* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_float* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_float* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_float* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_float* This,
        FLOAT* result);

    END_INTERFACE
} __FIReference_1_floatVtbl;

interface __FIReference_1_float
{
    CONST_VTBL struct __FIReference_1_floatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_float_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_float_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_float_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_float_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_float_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_float_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_float_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_float_INTERFACE_DEFINED__

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

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CTimeSpan __FIReference_1_Windows__CFoundation__CTimeSpan;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CTimeSpan;

typedef struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CTimeSpanVtbl;

interface __FIReference_1_Windows__CFoundation__CTimeSpan
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CTimeSpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 __x_ABI_CWindows_CFoundation_CNumerics_CVector2;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CDevices_CCore_CFrameFlashMode __x_ABI_CWindows_CMedia_CDevices_CCore_CFrameFlashMode;

/*
 *
 * Struct Windows.Media.Devices.Core.FrameFlashMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDevices_CCore_CFrameFlashMode
{
    FrameFlashMode_Disable = 0,
    FrameFlashMode_Enable = 1,
    FrameFlashMode_Global = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.ICameraIntrinsics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.CameraIntrinsics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_ICameraIntrinsics[] = L"Windows.Media.Devices.Core.ICameraIntrinsics";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FocalLength)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2* value);
    HRESULT (STDMETHODCALLTYPE* get_PrincipalPoint)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2* value);
    HRESULT (STDMETHODCALLTYPE* get_RadialDistortion)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_TangentialDistortion)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2* value);
    HRESULT (STDMETHODCALLTYPE* get_ImageWidth)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ImageHeight)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* ProjectOntoFrame)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 coordinate,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* UnprojectAtUnitDepth)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        struct __x_ABI_CWindows_CFoundation_CPoint pixelCoordinate,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2* result);
    HRESULT (STDMETHODCALLTYPE* ProjectManyOntoFrame)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        UINT32 coordinatesLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* coordinates,
        UINT32 resultsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* results);
    HRESULT (STDMETHODCALLTYPE* UnprojectPixelsAtUnitDepth)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* This,
        UINT32 pixelCoordinatesLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* pixelCoordinates,
        UINT32 resultsLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2* results);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_get_FocalLength(This, value) \
    ((This)->lpVtbl->get_FocalLength(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_get_PrincipalPoint(This, value) \
    ((This)->lpVtbl->get_PrincipalPoint(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_get_RadialDistortion(This, value) \
    ((This)->lpVtbl->get_RadialDistortion(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_get_TangentialDistortion(This, value) \
    ((This)->lpVtbl->get_TangentialDistortion(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_get_ImageWidth(This, value) \
    ((This)->lpVtbl->get_ImageWidth(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_get_ImageHeight(This, value) \
    ((This)->lpVtbl->get_ImageHeight(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_ProjectOntoFrame(This, coordinate, result) \
    ((This)->lpVtbl->ProjectOntoFrame(This, coordinate, result))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_UnprojectAtUnitDepth(This, pixelCoordinate, result) \
    ((This)->lpVtbl->UnprojectAtUnitDepth(This, pixelCoordinate, result))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_ProjectManyOntoFrame(This, coordinatesLength, coordinates, resultsLength, results) \
    ((This)->lpVtbl->ProjectManyOntoFrame(This, coordinatesLength, coordinates, resultsLength, results))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_UnprojectPixelsAtUnitDepth(This, pixelCoordinatesLength, pixelCoordinates, resultsLength, results) \
    ((This)->lpVtbl->UnprojectPixelsAtUnitDepth(This, pixelCoordinatesLength, pixelCoordinates, resultsLength, results))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.ICameraIntrinsics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.CameraIntrinsics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_ICameraIntrinsics2[] = L"Windows.Media.Devices.Core.ICameraIntrinsics2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UndistortedProjectionTransform)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4* value);
    HRESULT (STDMETHODCALLTYPE* DistortPoint)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        struct __x_ABI_CWindows_CFoundation_CPoint input,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* DistortPoints)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        UINT32 inputsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* inputs,
        UINT32 resultsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* results);
    HRESULT (STDMETHODCALLTYPE* UndistortPoint)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        struct __x_ABI_CWindows_CFoundation_CPoint input,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* UndistortPoints)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2* This,
        UINT32 inputsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* inputs,
        UINT32 resultsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* results);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_get_UndistortedProjectionTransform(This, value) \
    ((This)->lpVtbl->get_UndistortedProjectionTransform(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_DistortPoint(This, input, result) \
    ((This)->lpVtbl->DistortPoint(This, input, result))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_DistortPoints(This, inputsLength, inputs, resultsLength, results) \
    ((This)->lpVtbl->DistortPoints(This, inputsLength, inputs, resultsLength, results))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_UndistortPoint(This, input, result) \
    ((This)->lpVtbl->UndistortPoint(This, input, result))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_UndistortPoints(This, inputsLength, inputs, resultsLength, results) \
    ((This)->lpVtbl->UndistortPoints(This, inputsLength, inputs, resultsLength, results))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Devices.Core.ICameraIntrinsicsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.CameraIntrinsics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_ICameraIntrinsicsFactory[] = L"Windows.Media.Devices.Core.ICameraIntrinsicsFactory";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 focalLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 principalPoint,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 radialDistortion,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 tangentialDistortion,
        UINT32 imageWidth,
        UINT32 imageHeight,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_Create(This, focalLength, principalPoint, radialDistortion, tangentialDistortion, imageWidth, imageHeight, result) \
    ((This)->lpVtbl->Create(This, focalLength, principalPoint, radialDistortion, tangentialDistortion, imageWidth, imageHeight, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsicsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IDepthCorrelatedCoordinateMapper[] = L"Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapperVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UnprojectPoint)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        struct __x_ABI_CWindows_CFoundation_CPoint sourcePoint,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* targetCoordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* result);
    HRESULT (STDMETHODCALLTYPE* UnprojectPoints)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        UINT32 sourcePointsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* sourcePoints,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* targetCoordinateSystem,
        UINT32 resultsLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* results);
    HRESULT (STDMETHODCALLTYPE* MapPoint)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        struct __x_ABI_CWindows_CFoundation_CPoint sourcePoint,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* targetCoordinateSystem,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* targetCameraIntrinsics,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* MapPoints)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper* This,
        UINT32 sourcePointsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* sourcePoints,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* targetCoordinateSystem,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* targetCameraIntrinsics,
        UINT32 resultsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* results);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapperVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapperVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_UnprojectPoint(This, sourcePoint, targetCoordinateSystem, result) \
    ((This)->lpVtbl->UnprojectPoint(This, sourcePoint, targetCoordinateSystem, result))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_UnprojectPoints(This, sourcePointsLength, sourcePoints, targetCoordinateSystem, resultsLength, results) \
    ((This)->lpVtbl->UnprojectPoints(This, sourcePointsLength, sourcePoints, targetCoordinateSystem, resultsLength, results))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_MapPoint(This, sourcePoint, targetCoordinateSystem, targetCameraIntrinsics, result) \
    ((This)->lpVtbl->MapPoint(This, sourcePoint, targetCoordinateSystem, targetCameraIntrinsics, result))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_MapPoints(This, sourcePointsLength, sourcePoints, targetCoordinateSystem, targetCameraIntrinsics, resultsLength, results) \
    ((This)->lpVtbl->MapPoints(This, sourcePointsLength, sourcePoints, targetCoordinateSystem, targetCameraIntrinsics, resultsLength, results))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameControlCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameControlCapabilities[] = L"Windows.Media.Devices.Core.IFrameControlCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Exposure)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* get_ExposureCompensation)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* get_IsoSpeed)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* get_Focus)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* get_PhotoConfirmationSupported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_get_Exposure(This, value) \
    ((This)->lpVtbl->get_Exposure(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_get_ExposureCompensation(This, value) \
    ((This)->lpVtbl->get_ExposureCompensation(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_get_IsoSpeed(This, value) \
    ((This)->lpVtbl->get_IsoSpeed(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_get_Focus(This, value) \
    ((This)->lpVtbl->get_Focus(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_get_PhotoConfirmationSupported(This, value) \
    ((This)->lpVtbl->get_PhotoConfirmationSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameControlCapabilities2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameControlCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameControlCapabilities2[] = L"Windows.Media.Devices.Core.IFrameControlCapabilities2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Flash)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_get_Flash(This, value) \
    ((This)->lpVtbl->get_Flash(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameController[] = L"Windows.Media.Devices.Core.IFrameController";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExposureControl)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl** value);
    HRESULT (STDMETHODCALLTYPE* get_ExposureCompensationControl)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl** value);
    HRESULT (STDMETHODCALLTYPE* get_IsoSpeedControl)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl** value);
    HRESULT (STDMETHODCALLTYPE* get_FocusControl)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl** value);
    HRESULT (STDMETHODCALLTYPE* get_PhotoConfirmationEnabled)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_PhotoConfirmationEnabled)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController* This,
        __FIReference_1_boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControllerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_get_ExposureControl(This, value) \
    ((This)->lpVtbl->get_ExposureControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_get_ExposureCompensationControl(This, value) \
    ((This)->lpVtbl->get_ExposureCompensationControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_get_IsoSpeedControl(This, value) \
    ((This)->lpVtbl->get_IsoSpeedControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_get_FocusControl(This, value) \
    ((This)->lpVtbl->get_FocusControl(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_get_PhotoConfirmationEnabled(This, value) \
    ((This)->lpVtbl->get_PhotoConfirmationEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_put_PhotoConfirmationEnabled(This, value) \
    ((This)->lpVtbl->put_PhotoConfirmationEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameController2[] = L"Windows.Media.Devices.Core.IFrameController2";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FlashControl)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2Vtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_get_FlashControl(This, value) \
    ((This)->lpVtbl->get_FlashControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureCapabilities[] = L"Windows.Media.Devices.Core.IFrameExposureCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureCompensationCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureCompensationCapabilities[] = L"Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities* This,
        FLOAT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureCompensationControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureCompensationControl[] = L"Windows.Media.Devices.Core.IFrameExposureCompensationControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This,
        __FIReference_1_float** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl* This,
        __FIReference_1_float* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureCompensationControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameExposureControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameExposureControl[] = L"Windows.Media.Devices.Core.IFrameExposureControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Auto)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Auto)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_get_Auto(This, value) \
    ((This)->lpVtbl->get_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_put_Auto(This, value) \
    ((This)->lpVtbl->put_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameExposureControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFlashCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFlashCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFlashCapabilities[] = L"Windows.Media.Devices.Core.IFrameFlashCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_RedEyeReductionSupported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PowerSupported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_get_RedEyeReductionSupported(This, value) \
    ((This)->lpVtbl->get_RedEyeReductionSupported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_get_PowerSupported(This, value) \
    ((This)->lpVtbl->get_PowerSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFlashControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFlashControl[] = L"Windows.Media.Devices.Core.IFrameFlashControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCore_CFrameFlashMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        enum __x_ABI_CWindows_CMedia_CDevices_CCore_CFrameFlashMode value);
    HRESULT (STDMETHODCALLTYPE* get_Auto)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Auto)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RedEyeReduction)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RedEyeReduction)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PowerPercent)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_PowerPercent)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_get_Auto(This, value) \
    ((This)->lpVtbl->get_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_put_Auto(This, value) \
    ((This)->lpVtbl->put_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_get_RedEyeReduction(This, value) \
    ((This)->lpVtbl->get_RedEyeReduction(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_put_RedEyeReduction(This, value) \
    ((This)->lpVtbl->put_RedEyeReduction(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_get_PowerPercent(This, value) \
    ((This)->lpVtbl->get_PowerPercent(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_put_PowerPercent(This, value) \
    ((This)->lpVtbl->put_PowerPercent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFlashControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFocusCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFocusCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFocusCapabilities[] = L"Windows.Media.Devices.Core.IFrameFocusCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameFocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameFocusControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameFocusControl[] = L"Windows.Media.Devices.Core.IFrameFocusControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl* This,
        __FIReference_1_UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameFocusControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameIsoSpeedCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameIsoSpeedCapabilities[] = L"Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Step)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilitiesVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_get_Step(This, value) \
    ((This)->lpVtbl->get_Step(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IFrameIsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.FrameIsoSpeedControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IFrameIsoSpeedControl[] = L"Windows.Media.Devices.Core.IFrameIsoSpeedControl";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Auto)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Auto)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl* This,
        __FIReference_1_UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControlVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_get_Auto(This, value) \
    ((This)->lpVtbl->get_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_put_Auto(This, value) \
    ((This)->lpVtbl->put_Auto(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameIsoSpeedControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Devices.Core.IVariablePhotoSequenceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Devices.Core.VariablePhotoSequenceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Devices_Core_IVariablePhotoSequenceController[] = L"Windows.Media.Devices.Core.IVariablePhotoSequenceController";
typedef struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Supported)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPhotosPerSecond)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosPerSecondLimit)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_PhotosPerSecondLimit)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* GetHighestConcurrentFrameRate)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* captureProperties,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentFrameRate)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* get_FrameCapabilities)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIFrameControlCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredFrameControllers)(__x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController* This,
        __FIVector_1_Windows__CMedia__CDevices__CCore__CFrameController** items);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_get_Supported(This, value) \
    ((This)->lpVtbl->get_Supported(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_get_MaxPhotosPerSecond(This, value) \
    ((This)->lpVtbl->get_MaxPhotosPerSecond(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_get_PhotosPerSecondLimit(This, value) \
    ((This)->lpVtbl->get_PhotosPerSecondLimit(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_put_PhotosPerSecondLimit(This, value) \
    ((This)->lpVtbl->put_PhotosPerSecondLimit(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_GetHighestConcurrentFrameRate(This, captureProperties, value) \
    ((This)->lpVtbl->GetHighestConcurrentFrameRate(This, captureProperties, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_GetCurrentFrameRate(This, value) \
    ((This)->lpVtbl->GetCurrentFrameRate(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_get_FrameCapabilities(This, value) \
    ((This)->lpVtbl->get_FrameCapabilities(This, value))

#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_get_DesiredFrameControllers(This, items) \
    ((This)->lpVtbl->get_DesiredFrameControllers(This, items))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDevices_CCore_CIVariablePhotoSequenceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.CameraIntrinsics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Devices.Core.ICameraIntrinsicsFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.ICameraIntrinsics ** Default Interface **
 *    Windows.Media.Devices.Core.ICameraIntrinsics2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_CameraIntrinsics_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_CameraIntrinsics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_CameraIntrinsics[] = L"Windows.Media.Devices.Core.CameraIntrinsics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_DepthCorrelatedCoordinateMapper_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_DepthCorrelatedCoordinateMapper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_DepthCorrelatedCoordinateMapper[] = L"Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Devices.Core.FrameControlCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameControlCapabilities ** Default Interface **
 *    Windows.Media.Devices.Core.IFrameControlCapabilities2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameControlCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameControlCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameControlCapabilities[] = L"Windows.Media.Devices.Core.FrameControlCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameController ** Default Interface **
 *    Windows.Media.Devices.Core.IFrameController2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameController[] = L"Windows.Media.Devices.Core.FrameController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureCapabilities[] = L"Windows.Media.Devices.Core.FrameExposureCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureCompensationCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureCompensationCapabilities[] = L"Windows.Media.Devices.Core.FrameExposureCompensationCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureCompensationControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureCompensationControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureCompensationControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureCompensationControl[] = L"Windows.Media.Devices.Core.FrameExposureCompensationControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameExposureControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameExposureControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameExposureControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameExposureControl[] = L"Windows.Media.Devices.Core.FrameExposureControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFlashCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFlashCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFlashCapabilities[] = L"Windows.Media.Devices.Core.FrameFlashCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFlashControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFlashControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFlashControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFlashControl[] = L"Windows.Media.Devices.Core.FrameFlashControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFocusCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFocusCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFocusCapabilities[] = L"Windows.Media.Devices.Core.FrameFocusCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameFocusControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameFocusControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameFocusControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameFocusControl[] = L"Windows.Media.Devices.Core.FrameFocusControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameIsoSpeedCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameIsoSpeedCapabilities[] = L"Windows.Media.Devices.Core.FrameIsoSpeedCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.FrameIsoSpeedControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IFrameIsoSpeedControl ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_FrameIsoSpeedControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_FrameIsoSpeedControl[] = L"Windows.Media.Devices.Core.FrameIsoSpeedControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Devices.Core.VariablePhotoSequenceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Devices.Core.IVariablePhotoSequenceController ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Devices_Core_VariablePhotoSequenceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Devices_Core_VariablePhotoSequenceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Devices_Core_VariablePhotoSequenceController[] = L"Windows.Media.Devices.Core.VariablePhotoSequenceController";
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
#endif // __windows2Emedia2Edevices2Ecore_p_h__

#endif // __windows2Emedia2Edevices2Ecore_h__
