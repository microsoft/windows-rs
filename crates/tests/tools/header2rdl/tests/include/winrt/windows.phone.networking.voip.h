
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
#ifndef __windows2Ephone2Enetworking2Evoip_h__
#define __windows2Ephone2Enetworking2Evoip_h__
#ifndef __windows2Ephone2Enetworking2Evoip_p_h__
#define __windows2Ephone2Enetworking2Evoip_p_h__


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

#if !defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)
#define WINDOWS_PHONE_PHONECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)
#define WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Phone.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface ICallAnswerEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs ABI::Windows::Phone::Networking::Voip::ICallAnswerEventArgs

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface ICallAnswerEventArgs2;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2 ABI::Windows::Phone::Networking::Voip::ICallAnswerEventArgs2

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface ICallRejectEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs ABI::Windows::Phone::Networking::Voip::ICallRejectEventArgs

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface ICallStateChangeEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs ABI::Windows::Phone::Networking::Voip::ICallStateChangeEventArgs

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IMuteChangeEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs ABI::Windows::Phone::Networking::Voip::IMuteChangeEventArgs

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IQuerySeamlessUpgradeSupportOperation;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation ABI::Windows::Phone::Networking::Voip::IQuerySeamlessUpgradeSupportOperation

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipCallCoordinator;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinator

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipCallCoordinator2;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2 ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinator2

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipCallCoordinator3;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3 ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinator3

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipCallCoordinatorStatics;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinatorStatics

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipCallCoordinatorWithAppDeterminedUpgrade;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinatorWithAppDeterminedUpgrade

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipCallCoordinatorWithUpgrade;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinatorWithUpgrade

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipOperation;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation ABI::Windows::Phone::Networking::Voip::IVoipOperation

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipOperationsManager;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager ABI::Windows::Phone::Networking::Voip::IVoipOperationsManager

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipPhoneCall;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipPhoneCall2;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2 ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall2

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipPhoneCall3;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3 ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall3

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipPhoneCall4;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4 ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall4

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    interface IVoipPhoneCallReady;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady ABI::Windows::Phone::Networking::Voip::IVoipPhoneCallReady

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_FWD_DEFINED__

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



#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    class VoipCallCoordinator;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    class MuteChangeEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4603702c-d932-5c2b-b617-66846277a5dc"))
ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipCallCoordinator*, ABI::Windows::Phone::Networking::Voip::MuteChangeEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::VoipCallCoordinator*, ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::MuteChangeEventArgs*, ABI::Windows::Phone::Networking::Voip::IMuteChangeEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Phone.Networking.Voip.VoipCallCoordinator, Windows.Phone.Networking.Voip.MuteChangeEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipCallCoordinator*, ABI::Windows::Phone::Networking::Voip::MuteChangeEventArgs*> __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_USE */

#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    class VoipPhoneCall;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    class CallAnswerEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d31bfb3a-4430-5d06-80c8-7b87bba6dac1"))
ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::CallAnswerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::CallAnswerEventArgs*, ABI::Windows::Phone::Networking::Voip::ICallAnswerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Phone.Networking.Voip.VoipPhoneCall, Windows.Phone.Networking.Voip.CallAnswerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::CallAnswerEventArgs*> __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_USE */

#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    class CallRejectEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("141863d2-c712-5abe-bf93-36655c935e63"))
ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::CallRejectEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::CallRejectEventArgs*, ABI::Windows::Phone::Networking::Voip::ICallRejectEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Phone.Networking.Voip.VoipPhoneCall, Windows.Phone.Networking.Voip.CallRejectEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::CallRejectEventArgs*> __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_USE */

#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    class CallStateChangeEventArgs;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("80e8f32d-0b5b-568e-8861-250183b27e8e"))
ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::CallStateChangeEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Phone::Networking::Voip::CallStateChangeEventArgs*, ABI::Windows::Phone::Networking::Voip::ICallStateChangeEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Phone.Networking.Voip.VoipPhoneCall, Windows.Phone.Networking.Voip.CallStateChangeEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Phone::Networking::Voip::VoipPhoneCall*, ABI::Windows::Phone::Networking::Voip::CallStateChangeEventArgs*> __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_t;
#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_USE */

#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
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
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    typedef enum SeamlessCallUpgradeSupport : int SeamlessCallUpgradeSupport;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    typedef enum VoipCallMedia : unsigned int VoipCallMedia;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    typedef enum VoipCallRejectReason : int VoipCallRejectReason;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    typedef enum VoipCallState : int VoipCallState;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    typedef enum VoipOperationType : int VoipOperationType;
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Phone.Networking.Voip.SeamlessCallUpgradeSupport
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    enum SeamlessCallUpgradeSupport : int
                    {
                        SeamlessCallUpgradeSupport_Unknown = 0,
                        SeamlessCallUpgradeSupport_NotSupported = 1,
                        SeamlessCallUpgradeSupport_Supported = 2,
                    };
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipCallMedia
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    enum VoipCallMedia : unsigned int
                    {
                        VoipCallMedia_None = 0,
                        VoipCallMedia_Audio = 0x1,
                        VoipCallMedia_Video = 0x2,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(VoipCallMedia)
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipCallRejectReason
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    enum VoipCallRejectReason : int
                    {
                        VoipCallRejectReason_UserIgnored = 0,
                        VoipCallRejectReason_TimedOut = 1,
                        VoipCallRejectReason_OtherIncomingCall = 2,
                        VoipCallRejectReason_EmergencyCallExists = 3,
                        VoipCallRejectReason_InvalidCallState = 4,
                    };
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipCallState
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    enum VoipCallState : int
                    {
                        VoipCallState_Ended = 0,
                        VoipCallState_Held = 1,
                        VoipCallState_Active = 2,
                        VoipCallState_Incoming = 3,
                        VoipCallState_Outgoing = 4,
                    };
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipOperationType
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    enum VoipOperationType : int
                    {
                        VoipOperationType_QueryRemotePartySeamless = 0,
                    };
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallAnswerEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallAnswerEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallAnswerEventArgs[] = L"Windows.Phone.Networking.Voip.ICallAnswerEventArgs";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("61f132aa-f92a-48fa-aa8f-4f3a17662980")
                    ICallAnswerEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AcceptedMedia(
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICallAnswerEventArgs = __uuidof(ICallAnswerEventArgs);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallAnswerEventArgs2
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallAnswerEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.ICallAnswerEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallAnswerEventArgs2[] = L"Windows.Phone.Networking.Voip.ICallAnswerEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("ec5dcbbe-3c84-4de4-8817-fc788173406e")
                    ICallAnswerEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceDeviceId(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICallAnswerEventArgs2 = __uuidof(ICallAnswerEventArgs2);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallRejectEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallRejectEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallRejectEventArgs[] = L"Windows.Phone.Networking.Voip.ICallRejectEventArgs";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("3e04061a-ce7d-49c2-a2b8-7f49b6eaebc5")
                    ICallRejectEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RejectReason(
                            ABI::Windows::Phone::Networking::Voip::VoipCallRejectReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICallRejectEventArgs = __uuidof(ICallRejectEventArgs);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallStateChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallStateChangeEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallStateChangeEventArgs[] = L"Windows.Phone.Networking.Voip.ICallStateChangeEventArgs";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("1f3e87b0-d371-4395-8b6c-1786a42e2f18")
                    ICallStateChangeEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_State(
                            ABI::Windows::Phone::Networking::Voip::VoipCallState* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICallStateChangeEventArgs = __uuidof(ICallStateChangeEventArgs);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IMuteChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.MuteChangeEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IMuteChangeEventArgs[] = L"Windows.Phone.Networking.Voip.IMuteChangeEventArgs";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("f914ead4-1c94-458c-acb7-1926a233f74c")
                    IMuteChangeEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Muted(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMuteChangeEventArgs = __uuidof(IMuteChangeEventArgs);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IQuerySeamlessUpgradeSupportOperation
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.QuerySeamlessUpgradeSupportOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipOperation
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IQuerySeamlessUpgradeSupportOperation[] = L"Windows.Phone.Networking.Voip.IQuerySeamlessUpgradeSupportOperation";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("7e7adcce-cf2b-4ea0-8475-e1bdb4140379")
                    IQuerySeamlessUpgradeSupportOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyCompletion(
                            boolean succeeded,
                            ABI::Windows::Phone::Networking::Voip::SeamlessCallUpgradeSupport seamlessCallUpgradeSupport
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IQuerySeamlessUpgradeSupportOperation = __uuidof(IQuerySeamlessUpgradeSupportOperation);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinator
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinator[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinator";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("0d5f3579-f6dd-4e10-9f25-3d8c91376ee8")
                    IVoipCallCoordinator : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_MuteRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* muteChangeHandler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_MuteRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_UnmuteRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* muteChangeHandler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_UnmuteRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestNewIncomingCall(
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING contactNumber,
                            ABI::Windows::Foundation::IUriRuntimeClass* contactImage,
                            HSTRING serviceName,
                            ABI::Windows::Foundation::IUriRuntimeClass* brandingImage,
                            HSTRING callDetails,
                            ABI::Windows::Foundation::IUriRuntimeClass* ringtone,
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia media,
                            ABI::Windows::Foundation::TimeSpan ringTimeout,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestNewOutgoingCall(
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING serviceName,
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia media,
                            __FIVector_1_HSTRING* pAssociatedDeviceIds,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyMuted(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyUnmuted(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipCallCoordinator = __uuidof(IVoipCallCoordinator);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinator2
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinator2[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinator2";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("0290a9b8-645d-4711-8e32-926bfabf6928")
                    IVoipCallCoordinator2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetupNewAcceptedCall(
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING contactNumber,
                            HSTRING serviceName,
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia media,
                            __FIVector_1_HSTRING* pAssociatedDeviceIds,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipCallCoordinator2 = __uuidof(IVoipCallCoordinator2);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinator3
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinator3[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinator3";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("80a2ede6-0d02-40cd-8891-b4cec6472c82")
                    IVoipCallCoordinator3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestNewIncomingCallWithContactRemoteId(
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING contactNumber,
                            ABI::Windows::Foundation::IUriRuntimeClass* contactImage,
                            HSTRING serviceName,
                            ABI::Windows::Foundation::IUriRuntimeClass* brandingImage,
                            HSTRING callDetails,
                            ABI::Windows::Foundation::IUriRuntimeClass* ringtone,
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia media,
                            ABI::Windows::Foundation::TimeSpan ringTimeout,
                            HSTRING contactRemoteId,
                            __FIVector_1_HSTRING* pAssociatedDeviceIds,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestNewAppInitiatedCall(
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING contactNumber,
                            HSTRING serviceName,
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia media,
                            __FIVector_1_HSTRING* pAssociatedDeviceIds,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipCallCoordinator3 = __uuidof(IVoipCallCoordinator3);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinatorStatics
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinatorStatics[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinatorStatics";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("10fb55bb-e07d-407e-bc39-f7cc3641d979")
                    IVoipCallCoordinatorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDefault(
                            ABI::Windows::Phone::Networking::Voip::IVoipCallCoordinator** coordinator
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipCallCoordinatorStatics = __uuidof(IVoipCallCoordinatorStatics);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithAppDeterminedUpgrade
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinatorWithAppDeterminedUpgrade[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithAppDeterminedUpgrade";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("4ec3eda6-9cf0-405c-bccd-cb7a8632a456")
                    IVoipCallCoordinatorWithAppDeterminedUpgrade : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ConfirmNonSeamlessUpgrade(
                            GUID callUpgradeGuid
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CancelUpgrade(
                            GUID callUpgradeGuid
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipCallCoordinatorWithAppDeterminedUpgrade = __uuidof(IVoipCallCoordinatorWithAppDeterminedUpgrade);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithUpgrade
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinatorWithUpgrade[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithUpgrade";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("6172ccbc-6dd1-4f8e-b938-5393530c31ca")
                    IVoipCallCoordinatorWithUpgrade : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestOutgoingUpgradeToVideoCall(
                            GUID callUpgradeGuid,
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING serviceName,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestIncomingUpgradeToVideoCall(
                            HSTRING context,
                            HSTRING contactName,
                            HSTRING contactNumber,
                            ABI::Windows::Foundation::IUriRuntimeClass* contactImage,
                            HSTRING serviceName,
                            ABI::Windows::Foundation::IUriRuntimeClass* brandingImage,
                            HSTRING callDetails,
                            ABI::Windows::Foundation::IUriRuntimeClass* ringtone,
                            ABI::Windows::Foundation::TimeSpan ringTimeout,
                            ABI::Windows::Phone::Networking::Voip::IVoipPhoneCall** call
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipCallCoordinatorWithUpgrade = __uuidof(IVoipCallCoordinatorWithUpgrade);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipOperation
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipOperation[] = L"Windows.Phone.Networking.Voip.IVoipOperation";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("559b526d-c2cb-44d9-83a5-60cda4b71d36")
                    IVoipOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            UINT32* operationId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Type(
                            ABI::Windows::Phone::Networking::Voip::VoipOperationType* operationType
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipOperation = __uuidof(IVoipOperation);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipOperationsManager
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipOperationsManager[] = L"Windows.Phone.Networking.Voip.IVoipOperationsManager";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("ba77e23c-38d9-4dfb-853f-f901978ff7fa")
                    IVoipOperationsManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetNextOperation(
                            ABI::Windows::Phone::Networking::Voip::IVoipOperation** voipOperation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipOperationsManager = __uuidof(IVoipOperationsManager);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("716654be-28e8-495e-b657-8053074b4150")
                    IVoipPhoneCall : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_EndRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_EndRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_HoldRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_HoldRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ResumeRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ResumeRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AnswerRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs* acceptHandler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AnswerRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RejectRequested(
                            __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs* rejectHandler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RejectRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyCallHeld(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyCallActive(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyCallEnded(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContactName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContactName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                            ABI::Windows::Foundation::DateTime value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CallMedia(
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CallMedia(
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipPhoneCall = __uuidof(IVoipPhoneCall);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall2
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall2[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall2";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("1ac5b7dc-c83f-4862-8393-1ea14af7efea")
                    IVoipPhoneCall2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryShowAppUI(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipPhoneCall2 = __uuidof(IVoipPhoneCall2);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall3
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipPhoneCall2
 *     Windows.Phone.Networking.Voip.IVoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall3[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall3";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("a92ab1d1-52fb-4adf-8225-4d289b866fe0")
                    IVoipPhoneCall3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE NotifyCallAccepted(
                            ABI::Windows::Phone::Networking::Voip::VoipCallMedia media
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipPhoneCall3 = __uuidof(IVoipPhoneCall3);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall4
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall4[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall4";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("5f593d10-24ef-5006-b846-276132294a37")
                    IVoipPhoneCall4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsUsingAssociatedDevicesList(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyCallActiveOnDevices(
                            __FIIterable_1_HSTRING* associatedDeviceIds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddAssociatedCallControlDevice(
                            HSTRING deviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveAssociatedCallControlDevice(
                            HSTRING deviceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAssociatedCallControlDevices(
                            __FIIterable_1_HSTRING* associatedDeviceIds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAssociatedCallControlDevices(
                            __FIVectorView_1_HSTRING** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipPhoneCall4 = __uuidof(IVoipPhoneCall4);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCallReady
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCallReady[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCallReady";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace Networking {
                namespace Voip {
                    MIDL_INTERFACE("b3f5ac79-f40f-4e52-a8ee-e8a9a71f511a")
                    IVoipPhoneCallReady : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE NotifyCallReady(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVoipPhoneCallReady = __uuidof(IVoipPhoneCallReady);
                } /* Voip */
            } /* Networking */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.CallAnswerEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.ICallAnswerEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_CallAnswerEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_CallAnswerEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_CallAnswerEventArgs[] = L"Windows.Phone.Networking.Voip.CallAnswerEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.CallRejectEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.ICallRejectEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_CallRejectEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_CallRejectEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_CallRejectEventArgs[] = L"Windows.Phone.Networking.Voip.CallRejectEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.CallStateChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.ICallStateChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_CallStateChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_CallStateChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_CallStateChangeEventArgs[] = L"Windows.Phone.Networking.Voip.CallStateChangeEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.MuteChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IMuteChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_MuteChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_MuteChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_MuteChangeEventArgs[] = L"Windows.Phone.Networking.Voip.MuteChangeEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.QuerySeamlessUpgradeSupportOperation
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IVoipOperation
 *    Windows.Phone.Networking.Voip.IQuerySeamlessUpgradeSupportOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_QuerySeamlessUpgradeSupportOperation_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_QuerySeamlessUpgradeSupportOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_QuerySeamlessUpgradeSupportOperation[] = L"Windows.Phone.Networking.Voip.QuerySeamlessUpgradeSupportOperation";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Phone.Networking.Voip.IVoipCallCoordinatorStatics interface starting with version 1.0 of the Windows.Phone.PhoneInternalContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinator ** Default Interface **
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithUpgrade
 *    Windows.Phone.Networking.Voip.IVoipOperationsManager
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithAppDeterminedUpgrade
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinator2
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinator3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipCallCoordinator_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipCallCoordinator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_VoipCallCoordinator[] = L"Windows.Phone.Networking.Voip.VoipCallCoordinator";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall ** Default Interface **
 *    Windows.Phone.Networking.Voip.IVoipPhoneCallReady
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall2
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall3
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipPhoneCall_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipPhoneCall_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_VoipPhoneCall[] = L"Windows.Phone.Networking.Voip.VoipPhoneCall";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2 __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2 __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3 __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2 __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3 __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4 __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady;

#endif // ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_FWD_DEFINED__

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

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* This,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* sender,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs* This,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* sender,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs* This,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* sender,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* This,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* sender,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CSeamlessCallUpgradeSupport __x_ABI_CWindows_CPhone_CNetworking_CVoip_CSeamlessCallUpgradeSupport;

typedef enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia;

typedef enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallRejectReason __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallRejectReason;

typedef enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallState __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallState;

typedef enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipOperationType __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipOperationType;

/*
 *
 * Struct Windows.Phone.Networking.Voip.SeamlessCallUpgradeSupport
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CSeamlessCallUpgradeSupport
{
    SeamlessCallUpgradeSupport_Unknown = 0,
    SeamlessCallUpgradeSupport_NotSupported = 1,
    SeamlessCallUpgradeSupport_Supported = 2,
};
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipCallMedia
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia
{
    VoipCallMedia_None = 0,
    VoipCallMedia_Audio = 0x1,
    VoipCallMedia_Video = 0x2,
};
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipCallRejectReason
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallRejectReason
{
    VoipCallRejectReason_UserIgnored = 0,
    VoipCallRejectReason_TimedOut = 1,
    VoipCallRejectReason_OtherIncomingCall = 2,
    VoipCallRejectReason_EmergencyCallExists = 3,
    VoipCallRejectReason_InvalidCallState = 4,
};
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipCallState
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallState
{
    VoipCallState_Ended = 0,
    VoipCallState_Held = 1,
    VoipCallState_Active = 2,
    VoipCallState_Incoming = 3,
    VoipCallState_Outgoing = 4,
};
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Phone.Networking.Voip.VoipOperationType
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipOperationType
{
    VoipOperationType_QueryRemotePartySeamless = 0,
};
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallAnswerEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallAnswerEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallAnswerEventArgs[] = L"Windows.Phone.Networking.Voip.ICallAnswerEventArgs";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AcceptedMedia)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia* value);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgsVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_get_AcceptedMedia(This, value) \
    ((This)->lpVtbl->get_AcceptedMedia(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallAnswerEventArgs2
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallAnswerEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.ICallAnswerEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallAnswerEventArgs2[] = L"Windows.Phone.Networking.Voip.ICallAnswerEventArgs2";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceDeviceId)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2Vtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_get_SourceDeviceId(This, value) \
    ((This)->lpVtbl->get_SourceDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallAnswerEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallRejectEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallRejectEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallRejectEventArgs[] = L"Windows.Phone.Networking.Voip.ICallRejectEventArgs";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RejectReason)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallRejectReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgsVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_get_RejectReason(This, value) \
    ((This)->lpVtbl->get_RejectReason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallRejectEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.ICallStateChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.CallStateChangeEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_ICallStateChangeEventArgs[] = L"Windows.Phone.Networking.Voip.ICallStateChangeEventArgs";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallState* value);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgsVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CICallStateChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IMuteChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.MuteChangeEventArgs
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IMuteChangeEventArgs[] = L"Windows.Phone.Networking.Voip.IMuteChangeEventArgs";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Muted)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgsVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_get_Muted(This, value) \
    ((This)->lpVtbl->get_Muted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIMuteChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IQuerySeamlessUpgradeSupportOperation
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.QuerySeamlessUpgradeSupportOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipOperation
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IQuerySeamlessUpgradeSupportOperation[] = L"Windows.Phone.Networking.Voip.IQuerySeamlessUpgradeSupportOperation";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* NotifyCompletion)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation* This,
        boolean succeeded,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CSeamlessCallUpgradeSupport seamlessCallUpgradeSupport);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperationVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_NotifyCompletion(This, succeeded, seamlessCallUpgradeSupport) \
    ((This)->lpVtbl->NotifyCompletion(This, succeeded, seamlessCallUpgradeSupport))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIQuerySeamlessUpgradeSupportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinator
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinator[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinator";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MuteRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* muteChangeHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MuteRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UnmuteRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipCallCoordinator_Windows__CPhone__CNetworking__CVoip__CMuteChangeEventArgs* muteChangeHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UnmuteRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* RequestNewIncomingCall)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contactImage,
        HSTRING serviceName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* brandingImage,
        HSTRING callDetails,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* ringtone,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia media,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan ringTimeout,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* RequestNewOutgoingCall)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING serviceName,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia media,
        __FIVector_1_HSTRING* pAssociatedDeviceIds,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* NotifyMuted)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* NotifyUnmuted)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator* This);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_add_MuteRequested(This, muteChangeHandler, token) \
    ((This)->lpVtbl->add_MuteRequested(This, muteChangeHandler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_remove_MuteRequested(This, token) \
    ((This)->lpVtbl->remove_MuteRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_add_UnmuteRequested(This, muteChangeHandler, token) \
    ((This)->lpVtbl->add_UnmuteRequested(This, muteChangeHandler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_remove_UnmuteRequested(This, token) \
    ((This)->lpVtbl->remove_UnmuteRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_RequestNewIncomingCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, call) \
    ((This)->lpVtbl->RequestNewIncomingCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, call))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_RequestNewOutgoingCall(This, context, contactName, serviceName, media, pAssociatedDeviceIds, call) \
    ((This)->lpVtbl->RequestNewOutgoingCall(This, context, contactName, serviceName, media, pAssociatedDeviceIds, call))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_NotifyMuted(This) \
    ((This)->lpVtbl->NotifyMuted(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_NotifyUnmuted(This) \
    ((This)->lpVtbl->NotifyUnmuted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinator2
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinator2[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinator2";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetupNewAcceptedCall)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        HSTRING serviceName,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia media,
        __FIVector_1_HSTRING* pAssociatedDeviceIds,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2Vtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_SetupNewAcceptedCall(This, context, contactName, contactNumber, serviceName, media, pAssociatedDeviceIds, call) \
    ((This)->lpVtbl->SetupNewAcceptedCall(This, context, contactName, contactNumber, serviceName, media, pAssociatedDeviceIds, call))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinator3
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinator3[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinator3";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestNewIncomingCallWithContactRemoteId)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contactImage,
        HSTRING serviceName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* brandingImage,
        HSTRING callDetails,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* ringtone,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia media,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan ringTimeout,
        HSTRING contactRemoteId,
        __FIVector_1_HSTRING* pAssociatedDeviceIds,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* RequestNewAppInitiatedCall)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        HSTRING serviceName,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia media,
        __FIVector_1_HSTRING* pAssociatedDeviceIds,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3Vtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_RequestNewIncomingCallWithContactRemoteId(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, contactRemoteId, pAssociatedDeviceIds, call) \
    ((This)->lpVtbl->RequestNewIncomingCallWithContactRemoteId(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, contactRemoteId, pAssociatedDeviceIds, call))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_RequestNewAppInitiatedCall(This, context, contactName, contactNumber, serviceName, media, pAssociatedDeviceIds, call) \
    ((This)->lpVtbl->RequestNewAppInitiatedCall(This, context, contactName, contactNumber, serviceName, media, pAssociatedDeviceIds, call))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator3_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinatorStatics
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinatorStatics[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinatorStatics";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics* This,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinator** coordinator);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStaticsVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_GetDefault(This, coordinator) \
    ((This)->lpVtbl->GetDefault(This, coordinator))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithAppDeterminedUpgrade
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinatorWithAppDeterminedUpgrade[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithAppDeterminedUpgrade";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgradeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ConfirmNonSeamlessUpgrade)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This,
        GUID callUpgradeGuid);
    HRESULT (STDMETHODCALLTYPE* CancelUpgrade)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade* This,
        GUID callUpgradeGuid);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgradeVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgradeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_ConfirmNonSeamlessUpgrade(This, callUpgradeGuid) \
    ((This)->lpVtbl->ConfirmNonSeamlessUpgrade(This, callUpgradeGuid))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_CancelUpgrade(This, callUpgradeGuid) \
    ((This)->lpVtbl->CancelUpgrade(This, callUpgradeGuid))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithAppDeterminedUpgrade_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithUpgrade
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipCallCoordinatorWithUpgrade[] = L"Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithUpgrade";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgradeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestOutgoingUpgradeToVideoCall)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This,
        GUID callUpgradeGuid,
        HSTRING context,
        HSTRING contactName,
        HSTRING serviceName,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* RequestIncomingUpgradeToVideoCall)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contactImage,
        HSTRING serviceName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* brandingImage,
        HSTRING callDetails,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* ringtone,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan ringTimeout,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall** call);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgradeVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgradeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_RequestOutgoingUpgradeToVideoCall(This, callUpgradeGuid, context, contactName, serviceName, call) \
    ((This)->lpVtbl->RequestOutgoingUpgradeToVideoCall(This, callUpgradeGuid, context, contactName, serviceName, call))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_RequestIncomingUpgradeToVideoCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, ringTimeout, call) \
    ((This)->lpVtbl->RequestIncomingUpgradeToVideoCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, ringTimeout, call))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipCallCoordinatorWithUpgrade_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipOperation
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipOperation[] = L"Windows.Phone.Networking.Voip.IVoipOperation";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This,
        UINT32* operationId);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipOperationType* operationType);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_get_Id(This, operationId) \
    ((This)->lpVtbl->get_Id(This, operationId))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_get_Type(This, operationType) \
    ((This)->lpVtbl->get_Type(This, operationType))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipOperationsManager
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipOperationsManager[] = L"Windows.Phone.Networking.Voip.IVoipOperationsManager";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetNextOperation)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager* This,
        __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperation** voipOperation);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManagerVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_GetNextOperation(This, voipOperation) \
    ((This)->lpVtbl->GetNextOperation(This, voipOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipOperationsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_EndRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EndRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HoldRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HoldRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ResumeRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallStateChangeEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ResumeRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AnswerRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallAnswerEventArgs* acceptHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AnswerRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RejectRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CPhone__CNetworking__CVoip__CVoipPhoneCall_Windows__CPhone__CNetworking__CVoip__CCallRejectEventArgs* rejectHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RejectRequested)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* NotifyCallHeld)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* NotifyCallActive)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* NotifyCallEnded)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* get_ContactName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_CallMedia)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_CallMedia)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia value);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_add_EndRequested(This, handler, token) \
    ((This)->lpVtbl->add_EndRequested(This, handler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_remove_EndRequested(This, token) \
    ((This)->lpVtbl->remove_EndRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_add_HoldRequested(This, handler, token) \
    ((This)->lpVtbl->add_HoldRequested(This, handler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_remove_HoldRequested(This, token) \
    ((This)->lpVtbl->remove_HoldRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_add_ResumeRequested(This, handler, token) \
    ((This)->lpVtbl->add_ResumeRequested(This, handler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_remove_ResumeRequested(This, token) \
    ((This)->lpVtbl->remove_ResumeRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_add_AnswerRequested(This, acceptHandler, token) \
    ((This)->lpVtbl->add_AnswerRequested(This, acceptHandler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_remove_AnswerRequested(This, token) \
    ((This)->lpVtbl->remove_AnswerRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_add_RejectRequested(This, rejectHandler, token) \
    ((This)->lpVtbl->add_RejectRequested(This, rejectHandler, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_remove_RejectRequested(This, token) \
    ((This)->lpVtbl->remove_RejectRequested(This, token))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_NotifyCallHeld(This) \
    ((This)->lpVtbl->NotifyCallHeld(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_NotifyCallActive(This) \
    ((This)->lpVtbl->NotifyCallActive(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_NotifyCallEnded(This) \
    ((This)->lpVtbl->NotifyCallEnded(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_get_ContactName(This, value) \
    ((This)->lpVtbl->get_ContactName(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_put_ContactName(This, value) \
    ((This)->lpVtbl->put_ContactName(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_get_CallMedia(This, value) \
    ((This)->lpVtbl->get_CallMedia(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_put_CallMedia(This, value) \
    ((This)->lpVtbl->put_CallMedia(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall2
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall2[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall2";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryShowAppUI)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2* This);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2Vtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_TryShowAppUI(This) \
    ((This)->lpVtbl->TryShowAppUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall2_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall3
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Phone.Networking.Voip.IVoipPhoneCall2
 *     Windows.Phone.Networking.Voip.IVoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall3[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall3";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* NotifyCallAccepted)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3* This,
        enum __x_ABI_CWindows_CPhone_CNetworking_CVoip_CVoipCallMedia media);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3Vtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_NotifyCallAccepted(This, media) \
    ((This)->lpVtbl->NotifyCallAccepted(This, media))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall3_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCall4
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCall4[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCall4";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsUsingAssociatedDevicesList)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* NotifyCallActiveOnDevices)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        __FIIterable_1_HSTRING* associatedDeviceIds);
    HRESULT (STDMETHODCALLTYPE* AddAssociatedCallControlDevice)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        HSTRING deviceId);
    HRESULT (STDMETHODCALLTYPE* RemoveAssociatedCallControlDevice)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        HSTRING deviceId);
    HRESULT (STDMETHODCALLTYPE* SetAssociatedCallControlDevices)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        __FIIterable_1_HSTRING* associatedDeviceIds);
    HRESULT (STDMETHODCALLTYPE* GetAssociatedCallControlDevices)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4Vtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_get_IsUsingAssociatedDevicesList(This, value) \
    ((This)->lpVtbl->get_IsUsingAssociatedDevicesList(This, value))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_NotifyCallActiveOnDevices(This, associatedDeviceIds) \
    ((This)->lpVtbl->NotifyCallActiveOnDevices(This, associatedDeviceIds))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_AddAssociatedCallControlDevice(This, deviceId) \
    ((This)->lpVtbl->AddAssociatedCallControlDevice(This, deviceId))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_RemoveAssociatedCallControlDevice(This, deviceId) \
    ((This)->lpVtbl->RemoveAssociatedCallControlDevice(This, deviceId))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_SetAssociatedCallControlDevices(This, associatedDeviceIds) \
    ((This)->lpVtbl->SetAssociatedCallControlDevices(This, associatedDeviceIds))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_GetAssociatedCallControlDevices(This, result) \
    ((This)->lpVtbl->GetAssociatedCallControlDevices(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCall4_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.Networking.Voip.IVoipPhoneCallReady
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_Networking_Voip_IVoipPhoneCallReady[] = L"Windows.Phone.Networking.Voip.IVoipPhoneCallReady";
typedef struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReadyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* NotifyCallReady)(__x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady* This);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReadyVtbl;

interface __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReadyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_NotifyCallReady(This) \
    ((This)->lpVtbl->NotifyCallReady(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady;
#endif /* !defined(____x_ABI_CWindows_CPhone_CNetworking_CVoip_CIVoipPhoneCallReady_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.CallAnswerEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.ICallAnswerEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_CallAnswerEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_CallAnswerEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_CallAnswerEventArgs[] = L"Windows.Phone.Networking.Voip.CallAnswerEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.CallRejectEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.ICallRejectEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_CallRejectEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_CallRejectEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_CallRejectEventArgs[] = L"Windows.Phone.Networking.Voip.CallRejectEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.CallStateChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.ICallStateChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_CallStateChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_CallStateChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_CallStateChangeEventArgs[] = L"Windows.Phone.Networking.Voip.CallStateChangeEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.MuteChangeEventArgs
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IMuteChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_MuteChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_MuteChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_MuteChangeEventArgs[] = L"Windows.Phone.Networking.Voip.MuteChangeEventArgs";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.QuerySeamlessUpgradeSupportOperation
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IVoipOperation
 *    Windows.Phone.Networking.Voip.IQuerySeamlessUpgradeSupportOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_QuerySeamlessUpgradeSupportOperation_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_QuerySeamlessUpgradeSupportOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_QuerySeamlessUpgradeSupportOperation[] = L"Windows.Phone.Networking.Voip.QuerySeamlessUpgradeSupportOperation";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.VoipCallCoordinator
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Phone.Networking.Voip.IVoipCallCoordinatorStatics interface starting with version 1.0 of the Windows.Phone.PhoneInternalContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinator ** Default Interface **
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithUpgrade
 *    Windows.Phone.Networking.Voip.IVoipOperationsManager
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinatorWithAppDeterminedUpgrade
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinator2
 *    Windows.Phone.Networking.Voip.IVoipCallCoordinator3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipCallCoordinator_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipCallCoordinator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_VoipCallCoordinator[] = L"Windows.Phone.Networking.Voip.VoipCallCoordinator";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.Networking.Voip.VoipPhoneCall
 *
 * Introduced to Windows.Phone.PhoneInternalContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall ** Default Interface **
 *    Windows.Phone.Networking.Voip.IVoipPhoneCallReady
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall2
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall3
 *    Windows.Phone.Networking.Voip.IVoipPhoneCall4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipPhoneCall_DEFINED
#define RUNTIMECLASS_Windows_Phone_Networking_Voip_VoipPhoneCall_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_Networking_Voip_VoipPhoneCall[] = L"Windows.Phone.Networking.Voip.VoipPhoneCall";
#endif
#endif // WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Ephone2Enetworking2Evoip_p_h__

#endif // __windows2Ephone2Enetworking2Evoip_h__
