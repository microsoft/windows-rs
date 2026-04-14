
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
#ifndef __windows2Esystem2Epower2Ethermal_h__
#define __windows2Esystem2Epower2Ethermal_h__
#ifndef __windows2Esystem2Epower2Ethermal_p_h__
#define __windows2Esystem2Epower2Ethermal_p_h__


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

#if !defined(WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION)
#define WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION)

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
#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelConfiguration;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration ABI::Windows::System::Power::Thermal::IPowerThermalChannelConfiguration

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDataConsumer;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataConsumer

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDataConsumerFactory;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataConsumerFactory

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDataProducer;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataProducer

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDataProducerFactory;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataProducerFactory

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDataReceivedEventArgs;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataReceivedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDiagnostics;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics ABI::Windows::System::Power::Thermal::IPowerThermalChannelDiagnostics

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelDiagnosticsStatics;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics ABI::Windows::System::Power::Thermal::IPowerThermalChannelDiagnosticsStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    interface IPowerThermalChannelFinderStatics;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics ABI::Windows::System::Power::Thermal::IPowerThermalChannelFinderStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    typedef struct PowerThermalChannelId PowerThermalChannelId;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    class PowerThermalChannelConfiguration;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#define DEF___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("81b90641-0b65-56ef-b040-457f2e7be35a"))
IKeyValuePair<struct ABI::Windows::System::Power::Thermal::PowerThermalChannelId, ABI::Windows::System::Power::Thermal::PowerThermalChannelConfiguration*> : IKeyValuePair_impl<struct ABI::Windows::System::Power::Thermal::PowerThermalChannelId, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Power::Thermal::PowerThermalChannelConfiguration*, ABI::Windows::System::Power::Thermal::IPowerThermalChannelConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<Windows.System.Power.Thermal.PowerThermalChannelId, Windows.System.Power.Thermal.PowerThermalChannelConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<struct ABI::Windows::System::Power::Thermal::PowerThermalChannelId, ABI::Windows::System::Power::Thermal::PowerThermalChannelConfiguration*> __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t;
#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9c56b5da-3bcf-5ace-9b5d-2ee52b939664"))
IIterator<__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration*> : IIterator_impl<__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<Windows.System.Power.Thermal.PowerThermalChannelId, Windows.System.Power.Thermal.PowerThermalChannelConfiguration>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration*> __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t;
#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("36bfae51-120d-5970-b925-dfe1b226bf9a"))
IIterable<__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration*> : IIterable_impl<__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<Windows.System.Power.Thermal.PowerThermalChannelId, Windows.System.Power.Thermal.PowerThermalChannelConfiguration>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration*> __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t;
#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#define DEF___FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("610ce963-2546-5e6f-8cf1-51dd64a32781"))
IMapView<struct ABI::Windows::System::Power::Thermal::PowerThermalChannelId, ABI::Windows::System::Power::Thermal::PowerThermalChannelConfiguration*> : IMapView_impl<struct ABI::Windows::System::Power::Thermal::PowerThermalChannelId, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Power::Thermal::PowerThermalChannelConfiguration*, ABI::Windows::System::Power::Thermal::IPowerThermalChannelConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<Windows.System.Power.Thermal.PowerThermalChannelId, Windows.System.Power.Thermal.PowerThermalChannelConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<struct ABI::Windows::System::Power::Thermal::PowerThermalChannelId, ABI::Windows::System::Power::Thermal::PowerThermalChannelConfiguration*> __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t;
#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration ABI::Windows::Foundation::Collections::__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    class PowerThermalChannelDataConsumer;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fb816e05-b649-5baf-bb41-c1a7051773f0"))
ITypedEventHandler<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataConsumer*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataConsumer*, ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataConsumer*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Power.Thermal.PowerThermalChannelDataConsumer, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataConsumer*, IInspectable*> __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    class PowerThermalChannelDataReceivedEventArgs;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b59910ad-a826-5040-885b-c89fc0ad2cf3"))
ITypedEventHandler<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataConsumer*, ABI::Windows::System::Power::Thermal::PowerThermalChannelDataReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataConsumer*, ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataConsumer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataReceivedEventArgs*, ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Power.Thermal.PowerThermalChannelDataConsumer, Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataConsumer*, ABI::Windows::System::Power::Thermal::PowerThermalChannelDataReceivedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    class PowerThermalChannelDataProducer;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("790afc3f-625e-52ac-8769-fbb5b0ba7386"))
ITypedEventHandler<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataProducer*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataProducer*, ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataProducer*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Power.Thermal.PowerThermalChannelDataProducer, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Power::Thermal::PowerThermalChannelDataProducer*, IInspectable*> __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_USE */

#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

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
        namespace System {
            namespace Power {
                namespace Thermal {
                    typedef enum PowerThermalBackEndStatus : int PowerThermalBackEndStatus;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    typedef struct PowerThermalChannelData PowerThermalChannelData;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    class PowerThermalChannelDiagnostics;
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Power.Thermal.PowerThermalBackEndStatus
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    enum PowerThermalBackEndStatus : int
                    {
                        PowerThermalBackEndStatus_Stopped = 0,
                        PowerThermalBackEndStatus_Started = 1,
                    };
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.Thermal.PowerThermalChannelId
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    struct PowerThermalChannelId
                    {
                        GUID InterfaceType;
                        UINT16 InstanceId;
                    };
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.Thermal.PowerThermalChannelData
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    struct PowerThermalChannelData
                    {
                        ABI::Windows::System::Power::Thermal::PowerThermalChannelId Id;
                        INT32 Value;
                    };
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelConfiguration
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelConfiguration
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelConfiguration[] = L"Windows.System.Power.Thermal.IPowerThermalChannelConfiguration";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("ad8383fa-172d-5d82-b29d-81d93710fb45")
                    IPowerThermalChannelConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ConfigurationString(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetConfigurationNumericParameters(
                            UINT32* resultLength,
                            INT32** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelConfiguration = __uuidof(IPowerThermalChannelConfiguration);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataConsumer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataConsumer[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("47cca211-7348-5026-898c-b1873123760d")
                    IPowerThermalChannelDataConsumer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetChannelIds(
                            UINT32* resultLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetChannelConfigurations(
                            __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ChannelDataReceived(
                            __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ChannelDataReceived(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BackEndStatus(
                            ABI::Windows::System::Power::Thermal::PowerThermalBackEndStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_BackEndStatusChanged(
                            __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_BackEndStatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDataConsumer = __uuidof(IPowerThermalChannelDataConsumer);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataConsumer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataConsumerFactory[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("b42d9ab1-32f0-54bb-899a-9ae0529da381")
                    IPowerThermalChannelDataConsumerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            UINT32 channelIdsLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId* channelIds,
                            ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataConsumer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDataConsumerFactory = __uuidof(IPowerThermalChannelDataConsumerFactory);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataProducer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataProducer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataProducer[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataProducer";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("a935f244-1a7d-55d5-9c69-8adc1cd1d993")
                    IPowerThermalChannelDataProducer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetChannelIds(
                            UINT32* resultLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetChannelConfigurations(
                            __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DisableChannel(
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId channelId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PublishInputChannelData(
                            UINT32 dataLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelData* data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BackEndStatus(
                            ABI::Windows::System::Power::Thermal::PowerThermalBackEndStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_BackEndStatusChanged(
                            __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_BackEndStatusChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDataProducer = __uuidof(IPowerThermalChannelDataProducer);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataProducer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataProducerFactory[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("d2d380cd-e09d-5472-ad62-70061e630067")
                    IPowerThermalChannelDataProducerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            UINT32 channelIdsLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId* channelIds,
                            ABI::Windows::System::Power::Thermal::IPowerThermalChannelDataProducer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDataProducerFactory = __uuidof(IPowerThermalChannelDataProducerFactory);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataReceivedEventArgs[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("d6b643e0-6ab6-5683-a8fc-5ed65ee20dc5")
                    IPowerThermalChannelDataReceivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetData(
                            UINT32* resultLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelData** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDataReceivedEventArgs = __uuidof(IPowerThermalChannelDataReceivedEventArgs);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDiagnostics
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDiagnostics[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("628fd5c4-49b7-508f-ad9f-2309b1168aad")
                    IPowerThermalChannelDiagnostics : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDiagnostics = __uuidof(IPowerThermalChannelDiagnostics);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDiagnostics
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDiagnosticsStatics[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("a86ec393-b684-5633-a6ca-dfa1c7eecf87")
                    IPowerThermalChannelDiagnosticsStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::System::Power::Thermal::IPowerThermalChannelDiagnostics** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDataForChannels(
                            UINT32 channelIdsLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId* channelIds,
                            UINT32* resultLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelData** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelDiagnosticsStatics = __uuidof(IPowerThermalChannelDiagnosticsStatics);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelFinder
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelFinderStatics[] = L"Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Thermal {
                    MIDL_INTERFACE("df8d288b-f056-55ce-b370-f3e1c4e32063")
                    IPowerThermalChannelFinderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindChannels(
                            GUID channelInterfaceType,
                            UINT32* resultLength,
                            ABI::Windows::System::Power::Thermal::PowerThermalChannelId** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPowerThermalChannelFinderStatics = __uuidof(IPowerThermalChannelFinderStatics);
                } /* Thermal */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelConfiguration
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelConfiguration_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelConfiguration[] = L"Windows.System.Power.Thermal.PowerThermalChannelConfiguration";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDataConsumer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataConsumer_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataConsumer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDataConsumer[] = L"Windows.System.Power.Thermal.PowerThermalChannelDataConsumer";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDataProducer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDataProducer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataProducer_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataProducer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDataProducer[] = L"Windows.System.Power.Thermal.PowerThermalChannelDataProducer";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDataReceivedEventArgs[] = L"Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDiagnostics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDiagnostics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDiagnostics[] = L"Windows.System.Power.Thermal.PowerThermalChannelDiagnostics";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelFinder
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelFinder_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelFinder[] = L"Windows.System.Power.Thermal.PowerThermalChannelFinder";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId;

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

typedef struct __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration** result);

    END_INTERFACE
} __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl;

interface __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration
{
    CONST_VTBL struct __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

typedef struct __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl;

interface __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

typedef struct __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        __FIIterator_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl;

interface __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__)
#define ____FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__

typedef interface __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration;

typedef struct __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId key,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration* This,
        __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** first,
        __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** second);

    END_INTERFACE
} __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl;

interface __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration
{
    CONST_VTBL struct __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable* This,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* sender,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable* This,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalBackEndStatus __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalBackEndStatus;

typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelData __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelData;

/*
 *
 * Struct Windows.System.Power.Thermal.PowerThermalBackEndStatus
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalBackEndStatus
{
    PowerThermalBackEndStatus_Stopped = 0,
    PowerThermalBackEndStatus_Started = 1,
};
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.Thermal.PowerThermalChannelId
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId
{
    GUID InterfaceType;
    UINT16 InstanceId;
};
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Power.Thermal.PowerThermalChannelData
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelData
{
    struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId Id;
    INT32 Value;
};
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelConfiguration
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelConfiguration
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelConfiguration[] = L"Windows.System.Power.Thermal.IPowerThermalChannelConfiguration";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId* value);
    HRESULT (STDMETHODCALLTYPE* get_ConfigurationString)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetConfigurationNumericParameters)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration* This,
        UINT32* resultLength,
        INT32** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfigurationVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_get_ConfigurationString(This, value) \
    ((This)->lpVtbl->get_ConfigurationString(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_GetConfigurationNumericParameters(This, resultLength, result) \
    ((This)->lpVtbl->GetConfigurationNumericParameters(This, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataConsumer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataConsumer[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetChannelIds)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId** result);
    HRESULT (STDMETHODCALLTYPE* GetChannelConfigurations)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This);
    HRESULT (STDMETHODCALLTYPE* add_ChannelDataReceived)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ChannelDataReceived)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_BackEndStatus)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        enum __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalBackEndStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_BackEndStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataConsumer_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BackEndStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_GetChannelIds(This, resultLength, result) \
    ((This)->lpVtbl->GetChannelIds(This, resultLength, result))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_GetChannelConfigurations(This, result) \
    ((This)->lpVtbl->GetChannelConfigurations(This, result))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_add_ChannelDataReceived(This, handler, token) \
    ((This)->lpVtbl->add_ChannelDataReceived(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_remove_ChannelDataReceived(This, token) \
    ((This)->lpVtbl->remove_ChannelDataReceived(This, token))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_get_BackEndStatus(This, value) \
    ((This)->lpVtbl->get_BackEndStatus(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_add_BackEndStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_BackEndStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_remove_BackEndStatusChanged(This, token) \
    ((This)->lpVtbl->remove_BackEndStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataConsumer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataConsumerFactory[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory* This,
        UINT32 channelIdsLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId* channelIds,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_CreateInstance(This, channelIdsLength, channelIds, value) \
    ((This)->lpVtbl->CreateInstance(This, channelIdsLength, channelIds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataConsumerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataProducer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataProducer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataProducer[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataProducer";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetChannelIds)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId** result);
    HRESULT (STDMETHODCALLTYPE* GetChannelConfigurations)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        __FIMapView_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelId_Windows__CSystem__CPower__CThermal__CPowerThermalChannelConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* DisableChannel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId channelId);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This);
    HRESULT (STDMETHODCALLTYPE* PublishInputChannelData)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        UINT32 dataLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelData* data);
    HRESULT (STDMETHODCALLTYPE* get_BackEndStatus)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        enum __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalBackEndStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_BackEndStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        __FITypedEventHandler_2_Windows__CSystem__CPower__CThermal__CPowerThermalChannelDataProducer_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BackEndStatusChanged)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_GetChannelIds(This, resultLength, result) \
    ((This)->lpVtbl->GetChannelIds(This, resultLength, result))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_GetChannelConfigurations(This, result) \
    ((This)->lpVtbl->GetChannelConfigurations(This, result))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_DisableChannel(This, channelId) \
    ((This)->lpVtbl->DisableChannel(This, channelId))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_PublishInputChannelData(This, dataLength, data) \
    ((This)->lpVtbl->PublishInputChannelData(This, dataLength, data))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_get_BackEndStatus(This, value) \
    ((This)->lpVtbl->get_BackEndStatus(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_add_BackEndStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_BackEndStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_remove_BackEndStatusChanged(This, token) \
    ((This)->lpVtbl->remove_BackEndStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataProducer
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataProducerFactory[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory* This,
        UINT32 channelIdsLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId* channelIds,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_CreateInstance(This, channelIdsLength, channelIds, value) \
    ((This)->lpVtbl->CreateInstance(This, channelIdsLength, channelIds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataProducerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDataReceivedEventArgs[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetData)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs* This,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelData** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_GetData(This, resultLength, result) \
    ((This)->lpVtbl->GetData(This, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDataReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDiagnostics
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDiagnostics[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelDiagnostics
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelDiagnosticsStatics[] = L"Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This,
        __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnostics** value);
    HRESULT (STDMETHODCALLTYPE* GetDataForChannels)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics* This,
        UINT32 channelIdsLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId* channelIds,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelData** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_GetDataForChannels(This, channelIdsLength, channelIds, resultLength, result) \
    ((This)->lpVtbl->GetDataForChannels(This, channelIdsLength, channelIds, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelDiagnosticsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Thermal.PowerThermalChannelFinder
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Thermal_IPowerThermalChannelFinderStatics[] = L"Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics";
typedef struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindChannels)(__x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics* This,
        GUID channelInterfaceType,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CSystem_CPower_CThermal_CPowerThermalChannelId** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_FindChannels(This, channelInterfaceType, resultLength, result) \
    ((This)->lpVtbl->FindChannels(This, channelInterfaceType, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CThermal_CIPowerThermalChannelFinderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelConfiguration
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelConfiguration_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelConfiguration[] = L"Windows.System.Power.Thermal.PowerThermalChannelConfiguration";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDataConsumer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataConsumer_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataConsumer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDataConsumer[] = L"Windows.System.Power.Thermal.PowerThermalChannelDataConsumer";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDataProducer
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDataProducer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataProducer_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataProducer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDataProducer[] = L"Windows.System.Power.Thermal.PowerThermalChannelDataProducer";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDataReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDataReceivedEventArgs[] = L"Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelDiagnostics
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelDiagnostics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelDiagnostics[] = L"Windows.System.Power.Thermal.PowerThermalChannelDiagnostics";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Thermal.PowerThermalChannelFinder
 *
 * Introduced to Windows.System.Power.Thermal.PowerThermalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics interface starting with version 1.0 of the Windows.System.Power.Thermal.PowerThermalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelFinder_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Thermal_PowerThermalChannelFinder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Thermal_PowerThermalChannelFinder[] = L"Windows.System.Power.Thermal.PowerThermalChannelFinder";
#endif
#endif // WINDOWS_SYSTEM_POWER_THERMAL_POWERTHERMALAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Epower2Ethermal_p_h__

#endif // __windows2Esystem2Epower2Ethermal_h__
