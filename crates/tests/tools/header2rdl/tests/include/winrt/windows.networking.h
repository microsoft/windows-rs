
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
#ifndef __windows2Enetworking_h__
#define __windows2Enetworking_h__
#ifndef __windows2Enetworking_p_h__
#define __windows2Enetworking_p_h__


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

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Networking.Connectivity.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IEndpointPair;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIEndpointPair ABI::Windows::Networking::IEndpointPair

#endif // ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IEndpointPairFactory;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory ABI::Windows::Networking::IEndpointPairFactory

#endif // ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CNetworking_CIHostNameFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostNameFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IHostNameFactory;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIHostNameFactory ABI::Windows::Networking::IHostNameFactory

#endif // ____x_ABI_CWindows_CNetworking_CIHostNameFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostNameStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostNameStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IHostNameStatics;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIHostNameStatics ABI::Windows::Networking::IHostNameStatics

#endif // ____x_ABI_CWindows_CNetworking_CIHostNameStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
                class IPInformation;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Connectivity {
                interface IIPInformation;
            } /* Connectivity */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation ABI::Windows::Networking::Connectivity::IIPInformation

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            typedef enum HostNameType : int HostNameType;
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            class EndpointPair;
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            class HostName;
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.DomainNameType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            enum DomainNameType : int
            {
                DomainNameType_Suffix = 0,
                DomainNameType_FullyQualified = 1,
            };
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.HostNameSortOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            enum HostNameSortOptions : unsigned int
            {
                HostNameSortOptions_None = 0,
                HostNameSortOptions_OptimizeForLongConnections = 0x2,
            };

            DEFINE_ENUM_FLAG_OPERATORS(HostNameSortOptions)
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.HostNameType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            enum HostNameType : int
            {
                HostNameType_DomainName = 0,
                HostNameType_Ipv4 = 1,
                HostNameType_Ipv6 = 2,
                HostNameType_Bluetooth = 3,
            };
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IEndpointPair
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.EndpointPair
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIEndpointPair_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIEndpointPair_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IEndpointPair[] = L"Windows.Networking.IEndpointPair";
namespace ABI {
    namespace Windows {
        namespace Networking {
            MIDL_INTERFACE("33a0aa36-f8fa-4b30-b856-76517c3bd06d")
            IEndpointPair : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_LocalHostName(
                    ABI::Windows::Networking::IHostName** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_LocalHostName(
                    ABI::Windows::Networking::IHostName* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LocalServiceName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_LocalServiceName(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RemoteHostName(
                    ABI::Windows::Networking::IHostName** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_RemoteHostName(
                    ABI::Windows::Networking::IHostName* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RemoteServiceName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_RemoteServiceName(
                    HSTRING value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IEndpointPair = __uuidof(IEndpointPair);
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIEndpointPair;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIEndpointPair_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IEndpointPairFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.EndpointPair
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IEndpointPairFactory[] = L"Windows.Networking.IEndpointPairFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            MIDL_INTERFACE("b609d971-64e0-442b-aa6f-cc8c8f181f78")
            IEndpointPairFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateEndpointPair(
                    ABI::Windows::Networking::IHostName* localHostName,
                    HSTRING localServiceName,
                    ABI::Windows::Networking::IHostName* remoteHostName,
                    HSTRING remoteServiceName,
                    ABI::Windows::Networking::IEndpointPair** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IEndpointPairFactory = __uuidof(IEndpointPairFactory);
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIEndpointPairFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IHostName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.HostName
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIHostName_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIHostName_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IHostName[] = L"Windows.Networking.IHostName";
namespace ABI {
    namespace Windows {
        namespace Networking {
            MIDL_INTERFACE("bf8ecaad-ed96-49a7-9084-d416cae88dcb")
            IHostName : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_IPInformation(
                    ABI::Windows::Networking::Connectivity::IIPInformation** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RawName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CanonicalName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Type(
                    ABI::Windows::Networking::HostNameType* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE IsEqual(
                    ABI::Windows::Networking::IHostName* hostName,
                    boolean* isEqual
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IHostName = __uuidof(IHostName);
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIHostName;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIHostName_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IHostNameFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.HostName
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIHostNameFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIHostNameFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IHostNameFactory[] = L"Windows.Networking.IHostNameFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            MIDL_INTERFACE("458c23ed-712f-4576-adf1-c20b2c643558")
            IHostNameFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateHostName(
                    HSTRING hostName,
                    ABI::Windows::Networking::IHostName** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IHostNameFactory = __uuidof(IHostNameFactory);
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIHostNameFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIHostNameFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IHostNameStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.HostName
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIHostNameStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIHostNameStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IHostNameStatics[] = L"Windows.Networking.IHostNameStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            MIDL_INTERFACE("f68cd4bf-a388-4e8b-91ea-54dd6dd901c0")
            IHostNameStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Compare(
                    HSTRING value1,
                    HSTRING value2,
                    INT32* result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IHostNameStatics = __uuidof(IHostNameStatics);
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIHostNameStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIHostNameStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.EndpointPair
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.IEndpointPairFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.IEndpointPair ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_EndpointPair_DEFINED
#define RUNTIMECLASS_Windows_Networking_EndpointPair_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_EndpointPair[] = L"Windows.Networking.EndpointPair";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.HostName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.IHostNameFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.IHostNameStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.IHostName ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_HostName_DEFINED
#define RUNTIMECLASS_Windows_Networking_HostName_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_HostName[] = L"Windows.Networking.HostName";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIEndpointPair __x_ABI_CWindows_CNetworking_CIEndpointPair;

#endif // ____x_ABI_CWindows_CNetworking_CIEndpointPair_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIEndpointPairFactory __x_ABI_CWindows_CNetworking_CIEndpointPairFactory;

#endif // ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostNameFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostNameFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostNameFactory __x_ABI_CWindows_CNetworking_CIHostNameFactory;

#endif // ____x_ABI_CWindows_CNetworking_CIHostNameFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostNameStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostNameStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostNameStatics __x_ABI_CWindows_CNetworking_CIHostNameStatics;

#endif // ____x_ABI_CWindows_CNetworking_CIHostNameStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIStringable __x_ABI_CWindows_CFoundation_CIStringable;

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation;

#endif // ____x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CHostNameType __x_ABI_CWindows_CNetworking_CHostNameType;

/*
 *
 * Struct Windows.Networking.DomainNameType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CDomainNameType
{
    DomainNameType_Suffix = 0,
    DomainNameType_FullyQualified = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.HostNameSortOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CHostNameSortOptions
{
    HostNameSortOptions_None = 0,
    HostNameSortOptions_OptimizeForLongConnections = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.HostNameType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CHostNameType
{
    HostNameType_DomainName = 0,
    HostNameType_Ipv4 = 1,
    HostNameType_Ipv6 = 2,
    HostNameType_Bluetooth = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IEndpointPair
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.EndpointPair
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIEndpointPair_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIEndpointPair_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IEndpointPair[] = L"Windows.Networking.IEndpointPair";
typedef struct __x_ABI_CWindows_CNetworking_CIEndpointPairVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalHostName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_LocalHostName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalServiceName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_LocalServiceName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteHostName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteHostName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteServiceName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteServiceName)(__x_ABI_CWindows_CNetworking_CIEndpointPair* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CIEndpointPairVtbl;

interface __x_ABI_CWindows_CNetworking_CIEndpointPair
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CIEndpointPairVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_get_LocalHostName(This, value) \
    ((This)->lpVtbl->get_LocalHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_put_LocalHostName(This, value) \
    ((This)->lpVtbl->put_LocalHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_get_LocalServiceName(This, value) \
    ((This)->lpVtbl->get_LocalServiceName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_put_LocalServiceName(This, value) \
    ((This)->lpVtbl->put_LocalServiceName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_get_RemoteHostName(This, value) \
    ((This)->lpVtbl->get_RemoteHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_put_RemoteHostName(This, value) \
    ((This)->lpVtbl->put_RemoteHostName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_get_RemoteServiceName(This, value) \
    ((This)->lpVtbl->get_RemoteServiceName(This, value))

#define __x_ABI_CWindows_CNetworking_CIEndpointPair_put_RemoteServiceName(This, value) \
    ((This)->lpVtbl->put_RemoteServiceName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIEndpointPair;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIEndpointPair_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IEndpointPairFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.EndpointPair
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IEndpointPairFactory[] = L"Windows.Networking.IEndpointPairFactory";
typedef struct __x_ABI_CWindows_CNetworking_CIEndpointPairFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateEndpointPair)(__x_ABI_CWindows_CNetworking_CIEndpointPairFactory* This,
        __x_ABI_CWindows_CNetworking_CIHostName* localHostName,
        HSTRING localServiceName,
        __x_ABI_CWindows_CNetworking_CIHostName* remoteHostName,
        HSTRING remoteServiceName,
        __x_ABI_CWindows_CNetworking_CIEndpointPair** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CIEndpointPairFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CIEndpointPairFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CIEndpointPairFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CIEndpointPairFactory_CreateEndpointPair(This, localHostName, localServiceName, remoteHostName, remoteServiceName, value) \
    ((This)->lpVtbl->CreateEndpointPair(This, localHostName, localServiceName, remoteHostName, remoteServiceName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIEndpointPairFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIEndpointPairFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IHostName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.HostName
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIHostName_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIHostName_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IHostName[] = L"Windows.Networking.IHostName";
typedef struct __x_ABI_CWindows_CNetworking_CIHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CIHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CIHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IPInformation)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        __x_ABI_CWindows_CNetworking_CConnectivity_CIIPInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_RawName)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CanonicalName)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        enum __x_ABI_CWindows_CNetworking_CHostNameType* value);
    HRESULT (STDMETHODCALLTYPE* IsEqual)(__x_ABI_CWindows_CNetworking_CIHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName* hostName,
        boolean* isEqual);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CIHostNameVtbl;

interface __x_ABI_CWindows_CNetworking_CIHostName
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CIHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CIHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CIHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CIHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CIHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CIHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CIHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CIHostName_get_IPInformation(This, value) \
    ((This)->lpVtbl->get_IPInformation(This, value))

#define __x_ABI_CWindows_CNetworking_CIHostName_get_RawName(This, value) \
    ((This)->lpVtbl->get_RawName(This, value))

#define __x_ABI_CWindows_CNetworking_CIHostName_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CNetworking_CIHostName_get_CanonicalName(This, value) \
    ((This)->lpVtbl->get_CanonicalName(This, value))

#define __x_ABI_CWindows_CNetworking_CIHostName_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CNetworking_CIHostName_IsEqual(This, hostName, isEqual) \
    ((This)->lpVtbl->IsEqual(This, hostName, isEqual))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIHostName;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIHostName_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IHostNameFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.HostName
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIHostNameFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIHostNameFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IHostNameFactory[] = L"Windows.Networking.IHostNameFactory";
typedef struct __x_ABI_CWindows_CNetworking_CIHostNameFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateHostName)(__x_ABI_CWindows_CNetworking_CIHostNameFactory* This,
        HSTRING hostName,
        __x_ABI_CWindows_CNetworking_CIHostName** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CIHostNameFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CIHostNameFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CIHostNameFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CIHostNameFactory_CreateHostName(This, hostName, value) \
    ((This)->lpVtbl->CreateHostName(This, hostName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIHostNameFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIHostNameFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.IHostNameStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.HostName
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CIHostNameStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CIHostNameStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_IHostNameStatics[] = L"Windows.Networking.IHostNameStatics";
typedef struct __x_ABI_CWindows_CNetworking_CIHostNameStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Compare)(__x_ABI_CWindows_CNetworking_CIHostNameStatics* This,
        HSTRING value1,
        HSTRING value2,
        INT32* result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CIHostNameStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CIHostNameStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CIHostNameStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CIHostNameStatics_Compare(This, value1, value2, result) \
    ((This)->lpVtbl->Compare(This, value1, value2, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CIHostNameStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CIHostNameStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.EndpointPair
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.IEndpointPairFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.IEndpointPair ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_EndpointPair_DEFINED
#define RUNTIMECLASS_Windows_Networking_EndpointPair_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_EndpointPair[] = L"Windows.Networking.EndpointPair";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.HostName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.IHostNameFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.IHostNameStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.IHostName ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_HostName_DEFINED
#define RUNTIMECLASS_Windows_Networking_HostName_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_HostName[] = L"Windows.Networking.HostName";
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
#endif // __windows2Enetworking_p_h__

#endif // __windows2Enetworking_h__
