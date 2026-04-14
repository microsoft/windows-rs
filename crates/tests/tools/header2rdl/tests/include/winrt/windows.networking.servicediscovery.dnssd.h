
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
#ifndef __windows2Enetworking2Eservicediscovery2Ednssd_h__
#define __windows2Enetworking2Eservicediscovery2Ednssd_h__
#ifndef __windows2Enetworking2Eservicediscovery2Ednssd_p_h__
#define __windows2Enetworking2Eservicediscovery2Ednssd_p_h__


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

#if !defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)
#define WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Networking.h"
#include "Windows.Networking.Connectivity.h"
#include "Windows.Networking.Sockets.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    interface IDnssdRegistrationResult;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdRegistrationResult

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    interface IDnssdServiceInstance;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstance

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    interface IDnssdServiceInstanceFactory;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstanceFactory

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    interface IDnssdServiceWatcher;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceWatcher

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    class DnssdRegistrationResult;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0c251e73-52c9-5026-a875-f685a50cbffd"))
IAsyncOperation<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationResult*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdRegistrationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationResult*> __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f3632f52-894a-5345-9be6-8389751c5189"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationResult*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdRegistrationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    class DnssdServiceInstance;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#define DEF___FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bc0cca83-44e5-5544-ab5b-e09d66f5fd5f"))
IIterator<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t;
#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#define DEF___FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1de3a3e5-387e-5328-b864-3f0e3475d343"))
IIterable<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t;
#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f75ff82a-7e10-5cf6-8064-6ae585e0bd8d"))
IVectorView<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t;
#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    class DnssdServiceWatcher;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6a6cc5d5-9f43-545a-91d7-3a40055475da"))
ITypedEventHandler<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcher*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e617711e-cdd5-5975-8fb3-8eaaaed24e7b"))
ITypedEventHandler<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcher*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcher*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher, Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcher*, ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceInstance*> __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IStringable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIStringable ABI::Windows::Foundation::IStringable

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                class NetworkAdapter;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface INetworkAdapter;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter ABI::Windows::Networking::Connectivity::INetworkAdapter

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            class HostName;
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IHostName;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIHostName ABI::Windows::Networking::IHostName

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class DatagramSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IDatagramSocket;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket ABI::Windows::Networking::Sockets::IDatagramSocket

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                class StreamSocketListener;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                interface IStreamSocketListener;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener ABI::Windows::Networking::Sockets::IStreamSocketListener

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    typedef enum DnssdRegistrationStatus : int DnssdRegistrationStatus;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    typedef enum DnssdServiceWatcherStatus : int DnssdServiceWatcherStatus;
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    enum DnssdRegistrationStatus : int
                    {
                        DnssdRegistrationStatus_Success = 0,
                        DnssdRegistrationStatus_InvalidServiceName = 1,
                        DnssdRegistrationStatus_ServerError = 2,
                        DnssdRegistrationStatus_SecurityError = 3,
                    };
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    enum DnssdServiceWatcherStatus : int
                    {
                        DnssdServiceWatcherStatus_Created = 0,
                        DnssdServiceWatcherStatus_Started = 1,
                        DnssdServiceWatcherStatus_EnumerationCompleted = 2,
                        DnssdServiceWatcherStatus_Stopping = 3,
                        DnssdServiceWatcherStatus_Stopped = 4,
                        DnssdServiceWatcherStatus_Aborted = 5,
                    };
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdRegistrationResult[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    MIDL_INTERFACE("3d786ad2-e606-5350-73ea-7e97f066162f")
                    IDnssdRegistrationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdRegistrationStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IPAddress(
                            ABI::Windows::Networking::IHostName** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HasInstanceNameChanged(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDnssdRegistrationResult = __uuidof(IDnssdRegistrationResult);
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdServiceInstance[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    MIDL_INTERFACE("e246db7e-98a5-4ca1-b9e4-c253d33c35ff")
                    IDnssdServiceInstance : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DnssdServiceInstanceName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DnssdServiceInstanceName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HostName(
                            ABI::Windows::Networking::IHostName** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HostName(
                            ABI::Windows::Networking::IHostName* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Port(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Port(
                            UINT16 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Priority(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Priority(
                            UINT16 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Weight(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Weight(
                            UINT16 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TextAttributes(
                            __FIMap_2_HSTRING_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterStreamSocketListenerAsync1(
                            ABI::Windows::Networking::Sockets::IStreamSocketListener* socket,
                            __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterStreamSocketListenerAsync2(
                            ABI::Windows::Networking::Sockets::IStreamSocketListener* socket,
                            ABI::Windows::Networking::Connectivity::INetworkAdapter* adapter,
                            __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterDatagramSocketAsync1(
                            ABI::Windows::Networking::Sockets::IDatagramSocket* socket,
                            __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterDatagramSocketAsync2(
                            ABI::Windows::Networking::Sockets::IDatagramSocket* socket,
                            ABI::Windows::Networking::Connectivity::INetworkAdapter* adapter,
                            __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDnssdServiceInstance = __uuidof(IDnssdServiceInstance);
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdServiceInstanceFactory[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    MIDL_INTERFACE("6cb061a1-c478-4331-9684-4af2186c0a2b")
                    IDnssdServiceInstanceFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            HSTRING dnssdServiceInstanceName,
                            ABI::Windows::Networking::IHostName* hostName,
                            UINT16 port,
                            ABI::Windows::Networking::ServiceDiscovery::Dnssd::IDnssdServiceInstance** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDnssdServiceInstanceFactory = __uuidof(IDnssdServiceInstanceFactory);
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdServiceWatcher[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace ServiceDiscovery {
                namespace Dnssd {
                    MIDL_INTERFACE("cc34d9c1-db7d-4b69-983d-c6f83f205682")
                    IDnssdServiceWatcher : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_Added(
                            __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Added(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                            __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                            __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Networking::ServiceDiscovery::Dnssd::DnssdServiceWatcherStatus* status
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDnssdServiceWatcher = __uuidof(IDnssdServiceWatcher);
                } /* Dnssd */
            } /* ServiceDiscovery */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdRegistrationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdRegistrationResult[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstance_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstance[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstanceCollection_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstanceCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstanceCollection[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceWatcher_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceWatcher[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult;

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance;

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory;

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher;

#endif // ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResultVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* This,
        __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

typedef struct __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl;

interface __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

typedef struct __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        __FIIterator_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl;

interface __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

typedef struct __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl;

interface __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* This,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* sender,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIStringable __x_ABI_CWindows_CFoundation_CIStringable;

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener;

#endif // ____x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdRegistrationStatus __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdRegistrationStatus;

typedef enum __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdServiceWatcherStatus __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdServiceWatcherStatus;

/*
 *
 * Struct Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdRegistrationStatus
{
    DnssdRegistrationStatus_Success = 0,
    DnssdRegistrationStatus_InvalidServiceName = 1,
    DnssdRegistrationStatus_ServerError = 2,
    DnssdRegistrationStatus_SecurityError = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcherStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdServiceWatcherStatus
{
    DnssdServiceWatcherStatus_Created = 0,
    DnssdServiceWatcherStatus_Started = 1,
    DnssdServiceWatcherStatus_EnumerationCompleted = 2,
    DnssdServiceWatcherStatus_Stopping = 3,
    DnssdServiceWatcherStatus_Stopped = 4,
    DnssdServiceWatcherStatus_Aborted = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdRegistrationResult[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult";
typedef struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        enum __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdRegistrationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_IPAddress)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_HasInstanceNameChanged)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResultVtbl;

interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_get_IPAddress(This, value) \
    ((This)->lpVtbl->get_IPAddress(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_get_HasInstanceNameChanged(This, value) \
    ((This)->lpVtbl->get_HasInstanceNameChanged(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdServiceInstance[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance";
typedef struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DnssdServiceInstanceName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DnssdServiceInstanceName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_HostName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_HostName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_Port)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_Port)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* get_Priority)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_Priority)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* get_Weight)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_Weight)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* get_TextAttributes)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* RegisterStreamSocketListenerAsync1)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* socket,
        __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result);
    HRESULT (STDMETHODCALLTYPE* RegisterStreamSocketListenerAsync2)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIStreamSocketListener* socket,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* adapter,
        __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result);
    HRESULT (STDMETHODCALLTYPE* RegisterDatagramSocketAsync1)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* socket,
        __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result);
    HRESULT (STDMETHODCALLTYPE* RegisterDatagramSocketAsync2)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance* This,
        __x_ABI_CWindows_CNetworking_CSockets_CIDatagramSocket* socket,
        __x_ABI_CWindows_CNetworking_CConnectivity_CINetworkAdapter* adapter,
        __FIAsyncOperation_1_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdRegistrationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceVtbl;

interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_get_DnssdServiceInstanceName(This, value) \
    ((This)->lpVtbl->get_DnssdServiceInstanceName(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_put_DnssdServiceInstanceName(This, value) \
    ((This)->lpVtbl->put_DnssdServiceInstanceName(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_get_HostName(This, value) \
    ((This)->lpVtbl->get_HostName(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_put_HostName(This, value) \
    ((This)->lpVtbl->put_HostName(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_get_Port(This, value) \
    ((This)->lpVtbl->get_Port(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_put_Port(This, value) \
    ((This)->lpVtbl->put_Port(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_get_Priority(This, value) \
    ((This)->lpVtbl->get_Priority(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_put_Priority(This, value) \
    ((This)->lpVtbl->put_Priority(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_get_Weight(This, value) \
    ((This)->lpVtbl->get_Weight(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_put_Weight(This, value) \
    ((This)->lpVtbl->put_Weight(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_get_TextAttributes(This, value) \
    ((This)->lpVtbl->get_TextAttributes(This, value))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_RegisterStreamSocketListenerAsync1(This, socket, result) \
    ((This)->lpVtbl->RegisterStreamSocketListenerAsync1(This, socket, result))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_RegisterStreamSocketListenerAsync2(This, socket, adapter, result) \
    ((This)->lpVtbl->RegisterStreamSocketListenerAsync2(This, socket, adapter, result))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_RegisterDatagramSocketAsync1(This, socket, result) \
    ((This)->lpVtbl->RegisterDatagramSocketAsync1(This, socket, result))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_RegisterDatagramSocketAsync2(This, socket, adapter, result) \
    ((This)->lpVtbl->RegisterDatagramSocketAsync2(This, socket, adapter, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdServiceInstanceFactory[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory";
typedef struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory* This,
        HSTRING dnssdServiceInstanceName,
        __x_ABI_CWindows_CNetworking_CIHostName* hostName,
        UINT16 port,
        __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstance** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_Create(This, dnssdServiceInstanceName, hostName, port, result) \
    ((This)->lpVtbl->Create(This, dnssdServiceInstanceName, hostName, port, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceInstanceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_ServiceDiscovery_Dnssd_IDnssdServiceWatcher[] = L"Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher";
typedef struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceInstance* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        __FITypedEventHandler_2_Windows__CNetworking__CServiceDiscovery__CDnssd__CDnssdServiceWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This,
        enum __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CDnssdServiceWatcherStatus* status);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcherVtbl;

interface __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_get_Status(This, status) \
    ((This)->lpVtbl->get_Status(This, status))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CServiceDiscovery_CDnssd_CIDnssdServiceWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdRegistrationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdRegistrationResult[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstance_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstance[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstanceCollection_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstanceCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceInstanceCollection[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceWatcher_DEFINED
#define RUNTIMECLASS_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_ServiceDiscovery_Dnssd_DnssdServiceWatcher[] = L"Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher";
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
#endif // __windows2Enetworking2Eservicediscovery2Ednssd_p_h__

#endif // __windows2Enetworking2Eservicediscovery2Ednssd_h__
