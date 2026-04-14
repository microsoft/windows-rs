
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
#ifndef __windows2Esecurity2Ecryptography2Ecertificates_h__
#define __windows2Esecurity2Ecryptography2Ecertificates_h__
#ifndef __windows2Esecurity2Ecryptography2Ecertificates_p_h__
#define __windows2Esecurity2Ecryptography2Ecertificates_p_h__


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
#include "Windows.Networking.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate ABI::Windows::Security::Cryptography::Certificates::ICertificate

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2 ABI::Windows::Security::Cryptography::Certificates::ICertificate2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate3;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3 ABI::Windows::Security::Cryptography::Certificates::ICertificate3

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateChain;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain ABI::Windows::Security::Cryptography::Certificates::ICertificateChain

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateEnrollmentManagerStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics ABI::Windows::Security::Cryptography::Certificates::ICertificateEnrollmentManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateEnrollmentManagerStatics2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2 ABI::Windows::Security::Cryptography::Certificates::ICertificateEnrollmentManagerStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateEnrollmentManagerStatics3;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3 ABI::Windows::Security::Cryptography::Certificates::ICertificateEnrollmentManagerStatics3

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateExtension;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension ABI::Windows::Security::Cryptography::Certificates::ICertificateExtension

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateFactory;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory ABI::Windows::Security::Cryptography::Certificates::ICertificateFactory

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateKeyUsages;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages ABI::Windows::Security::Cryptography::Certificates::ICertificateKeyUsages

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateQuery;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery ABI::Windows::Security::Cryptography::Certificates::ICertificateQuery

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateQuery2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2 ABI::Windows::Security::Cryptography::Certificates::ICertificateQuery2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateRequestProperties;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties ABI::Windows::Security::Cryptography::Certificates::ICertificateRequestProperties

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateRequestProperties2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2 ABI::Windows::Security::Cryptography::Certificates::ICertificateRequestProperties2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateRequestProperties3;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3 ABI::Windows::Security::Cryptography::Certificates::ICertificateRequestProperties3

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateRequestProperties4;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4 ABI::Windows::Security::Cryptography::Certificates::ICertificateRequestProperties4

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateStore;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore ABI::Windows::Security::Cryptography::Certificates::ICertificateStore

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateStore2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2 ABI::Windows::Security::Cryptography::Certificates::ICertificateStore2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateStoresStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics ABI::Windows::Security::Cryptography::Certificates::ICertificateStoresStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificateStoresStatics2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2 ABI::Windows::Security::Cryptography::Certificates::ICertificateStoresStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IChainBuildingParameters;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters ABI::Windows::Security::Cryptography::Certificates::IChainBuildingParameters

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IChainValidationParameters;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters ABI::Windows::Security::Cryptography::Certificates::IChainValidationParameters

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsAttachedSignature;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature ABI::Windows::Security::Cryptography::Certificates::ICmsAttachedSignature

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsAttachedSignatureFactory;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory ABI::Windows::Security::Cryptography::Certificates::ICmsAttachedSignatureFactory

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsAttachedSignatureStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics ABI::Windows::Security::Cryptography::Certificates::ICmsAttachedSignatureStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsDetachedSignature;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature ABI::Windows::Security::Cryptography::Certificates::ICmsDetachedSignature

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsDetachedSignatureFactory;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory ABI::Windows::Security::Cryptography::Certificates::ICmsDetachedSignatureFactory

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsDetachedSignatureStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics ABI::Windows::Security::Cryptography::Certificates::ICmsDetachedSignatureStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsSignerInfo;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo ABI::Windows::Security::Cryptography::Certificates::ICmsSignerInfo

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICmsTimestampInfo;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo ABI::Windows::Security::Cryptography::Certificates::ICmsTimestampInfo

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IKeyAlgorithmNamesStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics ABI::Windows::Security::Cryptography::Certificates::IKeyAlgorithmNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IKeyAlgorithmNamesStatics2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2 ABI::Windows::Security::Cryptography::Certificates::IKeyAlgorithmNamesStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IKeyAttestationHelperStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics ABI::Windows::Security::Cryptography::Certificates::IKeyAttestationHelperStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IKeyAttestationHelperStatics2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2 ABI::Windows::Security::Cryptography::Certificates::IKeyAttestationHelperStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IKeyStorageProviderNamesStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics ABI::Windows::Security::Cryptography::Certificates::IKeyStorageProviderNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IKeyStorageProviderNamesStatics2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2 ABI::Windows::Security::Cryptography::Certificates::IKeyStorageProviderNamesStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IPfxImportParameters;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters ABI::Windows::Security::Cryptography::Certificates::IPfxImportParameters

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IStandardCertificateStoreNamesStatics;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics ABI::Windows::Security::Cryptography::Certificates::IStandardCertificateStoreNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ISubjectAlternativeNameInfo;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo ABI::Windows::Security::Cryptography::Certificates::ISubjectAlternativeNameInfo

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ISubjectAlternativeNameInfo2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2 ABI::Windows::Security::Cryptography::Certificates::ISubjectAlternativeNameInfo2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IUserCertificateEnrollmentManager;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager ABI::Windows::Security::Cryptography::Certificates::IUserCertificateEnrollmentManager

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IUserCertificateEnrollmentManager2;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2 ABI::Windows::Security::Cryptography::Certificates::IUserCertificateEnrollmentManager2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface IUserCertificateStore;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore ABI::Windows::Security::Cryptography::Certificates::IUserCertificateStore

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */



#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class Certificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("676fc159-f15c-58bd-91a7-28f7e795c756"))
IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0c7d1423-e8fd-5a91-b55c-8bfbe7ac2d40"))
IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("963f7013-77c2-51c5-8038-b5bcef633edb"))
IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9b26648e-b32f-5909-a635-78e6d3bb4067"))
IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.Certificate>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate*> __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1896faee-23e2-59ca-9802-0f48eed98ef4"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.Certificate>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CertificateChain;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f618c7d4-aee1-58ae-afe8-fc336daf0395"))
IAsyncOperation<ABI::Windows::Security::Cryptography::Certificates::CertificateChain*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CertificateChain*, ABI::Windows::Security::Cryptography::Certificates::ICertificateChain*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Cryptography.Certificates.CertificateChain>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Cryptography::Certificates::CertificateChain*> __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4c3f50e9-90e3-5a30-9015-4aa0376904f3"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Cryptography::Certificates::CertificateChain*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CertificateChain*, ABI::Windows::Security::Cryptography::Certificates::ICertificateChain*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Cryptography.Certificates.CertificateChain>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Cryptography::Certificates::CertificateChain*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum SignatureValidationResult : int SignatureValidationResult;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f09c0bcf-ce3b-5dff-971f-2c3ffe6fd04d"))
IAsyncOperation<enum ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult> : IAsyncOperation_impl<enum ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Cryptography.Certificates.SignatureValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult> __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dff50005-78ad-5f4f-a085-cb614a674a25"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Cryptography.Certificates.SignatureValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bee8834-b9a7-5a80-a746-5ef097227878"))
IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51c3d2fd-b8a1-5620-b746-7ee6d533aca3"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CertificateExtension;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5e5af982-332d-54ec-8e54-e62c1a1eace9"))
IIterator<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*, ABI::Windows::Security::Cryptography::Certificates::ICertificateExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.CertificateExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1bdd7127-73b3-5192-8bde-20c136281260"))
IIterable<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*, ABI::Windows::Security::Cryptography::Certificates::ICertificateExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.CertificateExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CmsSignerInfo;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ba691628-d419-5e0a-b924-03ebc236b11e"))
IIterator<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*, ABI::Windows::Security::Cryptography::Certificates::ICmsSignerInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.CmsSignerInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6af24174-2dda-5a54-a0b9-4d6690059427"))
IIterable<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*, ABI::Windows::Security::Cryptography::Certificates::ICmsSignerInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.CmsSignerInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE */

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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43857453-e7b1-5cba-9730-5ea4ddebdd95"))
IVectorView<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*, ABI::Windows::Security::Cryptography::Certificates::ICertificateExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.CertificateExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f46bcaa8-747c-5a93-82fe-85d63549fe81"))
IVectorView<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*, ABI::Windows::Security::Cryptography::Certificates::ICmsSignerInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.CmsSignerInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Cryptography::Certificates::CmsSignerInfo*> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("36282c0f-2f1f-57f4-b2b1-867af90c3d13"))
IVector<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#define DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4c2523e8-9773-50fe-b870-483fd8b906dc"))
IVector<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*, ABI::Windows::Security::Cryptography::Certificates::ICertificateExtension*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Security.Cryptography.Certificates.CertificateExtension>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Security::Cryptography::Certificates::CertificateExtension*> __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t;
#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum CertificateChainPolicy : int CertificateChainPolicy;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum ChainValidationResult : int ChainValidationResult;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum EnrollKeyUsages : unsigned int EnrollKeyUsages;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum ExportOption : int ExportOption;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum InstallOptions : unsigned int InstallOptions;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum KeyProtectionLevel : int KeyProtectionLevel;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CertificateKeyUsages;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CertificateQuery;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CertificateRequestProperties;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CertificateStore;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class ChainBuildingParameters;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class ChainValidationParameters;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CmsAttachedSignature;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CmsDetachedSignature;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class CmsTimestampInfo;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class PfxImportParameters;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class SubjectAlternativeNameInfo;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class UserCertificateEnrollmentManager;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class UserCertificateStore;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.CertificateChainPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum CertificateChainPolicy : int
                    {
                        CertificateChainPolicy_Base = 0,
                        CertificateChainPolicy_Ssl = 1,
                        CertificateChainPolicy_NTAuthentication = 2,
                        CertificateChainPolicy_MicrosoftRoot = 3,
                    };
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.ChainValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum ChainValidationResult : int
                    {
                        ChainValidationResult_Success = 0,
                        ChainValidationResult_Untrusted = 1,
                        ChainValidationResult_Revoked = 2,
                        ChainValidationResult_Expired = 3,
                        ChainValidationResult_IncompleteChain = 4,
                        ChainValidationResult_InvalidSignature = 5,
                        ChainValidationResult_WrongUsage = 6,
                        ChainValidationResult_InvalidName = 7,
                        ChainValidationResult_InvalidCertificateAuthorityPolicy = 8,
                        ChainValidationResult_BasicConstraintsError = 9,
                        ChainValidationResult_UnknownCriticalExtension = 10,
                        ChainValidationResult_RevocationInformationMissing = 11,
                        ChainValidationResult_RevocationFailure = 12,
                        ChainValidationResult_OtherErrors = 13,
                    };
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.EnrollKeyUsages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum EnrollKeyUsages : unsigned int
                    {
                        EnrollKeyUsages_None = 0,
                        EnrollKeyUsages_Decryption = 0x1,
                        EnrollKeyUsages_Signing = 0x2,
                        EnrollKeyUsages_KeyAgreement = 0x4,
                        EnrollKeyUsages_All = 0xffffff,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(EnrollKeyUsages)
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.ExportOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum ExportOption : int
                    {
                        ExportOption_NotExportable = 0,
                        ExportOption_Exportable = 1,
                    };
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.InstallOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum InstallOptions : unsigned int
                    {
                        InstallOptions_None = 0,
                        InstallOptions_DeleteExpired = 0x1,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(InstallOptions)
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.KeyProtectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum KeyProtectionLevel : int
                    {
                        KeyProtectionLevel_NoConsent = 0,
                        KeyProtectionLevel_ConsentOnly = 1,
                        KeyProtectionLevel_ConsentWithPassword = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        KeyProtectionLevel_ConsentWithFingerprint = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    };
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.KeySize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum KeySize : int
                    {
                        KeySize_Invalid = 0,
                        KeySize_Rsa2048 = 2048,
                        KeySize_Rsa4096 = 4096,
                    };
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.SignatureValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    enum SignatureValidationResult : int
                    {
                        SignatureValidationResult_Success = 0,
                        SignatureValidationResult_InvalidParameter = 1,
                        SignatureValidationResult_BadMessage = 2,
                        SignatureValidationResult_InvalidSignature = 3,
                        SignatureValidationResult_OtherErrors = 4,
                    };
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificate[] = L"Windows.Security.Cryptography.Certificates.ICertificate";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("333f740c-04d8-43b3-b278-8c5fcc9be5a0")
                    ICertificate : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE BuildChainAsync(
                            __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
                            __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE BuildChainWithParametersAsync(
                            __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
                            ABI::Windows::Security::Cryptography::Certificates::IChainBuildingParameters* parameters,
                            __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SerialNumber(
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHashValue(
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHashValueWithAlgorithm(
                            HSTRING hashAlgorithmName,
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCertificateBlob(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Issuer(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HasPrivateKey(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsStronglyProtected(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValidFrom(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValidTo(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EnhancedKeyUsages(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificate = __uuidof(ICertificate);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificate2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificate2[] = L"Windows.Security.Cryptography.Certificates.ICertificate2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("17b8374c-8a25-4d96-a492-8fc29ac4fda6")
                    ICertificate2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsSecurityDeviceBound(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyUsages(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateKeyUsages** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyAlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SignatureAlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SignatureHashAlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SubjectAlternativeName(
                            ABI::Windows::Security::Cryptography::Certificates::ISubjectAlternativeNameInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificate2 = __uuidof(ICertificate2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificate3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificate3[] = L"Windows.Security.Cryptography.Certificates.ICertificate3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("be51a966-ae5f-4652-ace7-c6d7e7724cf3")
                    ICertificate3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsPerUser(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StoreName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyStorageProviderName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificate3 = __uuidof(ICertificate3);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateChain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateChain
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateChain[] = L"Windows.Security.Cryptography.Certificates.ICertificateChain";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("20bf5385-3691-4501-a62c-fd97278b31ee")
                    ICertificateChain : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Validate(
                            ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult* status
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ValidateWithParameters(
                            ABI::Windows::Security::Cryptography::Certificates::IChainValidationParameters* parameter,
                            ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult* status
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCertificates(
                            boolean includeRoot,
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** certificates
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateChain = __uuidof(ICertificateChain);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateEnrollmentManagerStatics[] = L"Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("8846ef3f-a986-48fb-9fd7-9aec06935bf1")
                    ICertificateEnrollmentManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateRequestAsync(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateRequestProperties* request,
                            __FIAsyncOperation_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE InstallCertificateAsync(
                            HSTRING certificate,
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions installOption,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportPfxDataAsync(
                            HSTRING pfxData,
                            HSTRING password,
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption exportable,
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel keyProtectionLevel,
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions installOption,
                            HSTRING friendlyName,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateEnrollmentManagerStatics = __uuidof(ICertificateEnrollmentManagerStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateEnrollmentManagerStatics2[] = L"Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("dc5b1c33-6429-4014-999c-5d9735802d1d")
                    ICertificateEnrollmentManagerStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UserCertificateEnrollmentManager(
                            ABI::Windows::Security::Cryptography::Certificates::IUserCertificateEnrollmentManager** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportPfxDataToKspAsync(
                            HSTRING pfxData,
                            HSTRING password,
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption exportable,
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel keyProtectionLevel,
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions installOption,
                            HSTRING friendlyName,
                            HSTRING keyStorageProvider,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateEnrollmentManagerStatics2 = __uuidof(ICertificateEnrollmentManagerStatics2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateEnrollmentManagerStatics3[] = L"Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("fdec82be-617c-425a-b72d-398b26ac7264")
                    ICertificateEnrollmentManagerStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ImportPfxDataToKspWithParametersAsync(
                            HSTRING pfxData,
                            HSTRING password,
                            ABI::Windows::Security::Cryptography::Certificates::IPfxImportParameters* pfxImportParameters,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateEnrollmentManagerStatics3 = __uuidof(ICertificateEnrollmentManagerStatics3);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateExtension[] = L"Windows.Security.Cryptography.Certificates.ICertificateExtension";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("84cf0656-a9e6-454d-8e45-2ea7c4bcd53b")
                    ICertificateExtension : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ObjectId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ObjectId(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCritical(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsCritical(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE EncodeValue(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            UINT32 valueLength,
                            BYTE* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateExtension = __uuidof(ICertificateExtension);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateFactory[] = L"Windows.Security.Cryptography.Certificates.ICertificateFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("17b4221c-4baf-44a2-9608-04fb62b16942")
                    ICertificateFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateCertificate(
                            ABI::Windows::Storage::Streams::IBuffer* certBlob,
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** certificate
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateFactory = __uuidof(ICertificateFactory);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateKeyUsages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateKeyUsages
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateKeyUsages[] = L"Windows.Security.Cryptography.Certificates.ICertificateKeyUsages";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("6ac6206f-e1cf-486a-b485-a69c83e46fd1")
                    ICertificateKeyUsages : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EncipherOnly(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EncipherOnly(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CrlSign(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CrlSign(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyCertificateSign(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyCertificateSign(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyAgreement(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyAgreement(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DataEncipherment(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DataEncipherment(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyEncipherment(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyEncipherment(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NonRepudiation(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NonRepudiation(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DigitalSignature(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DigitalSignature(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateKeyUsages = __uuidof(ICertificateKeyUsages);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateQuery[] = L"Windows.Security.Cryptography.Certificates.ICertificateQuery";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("5b082a31-a728-4916-b5ee-ffcb8acf2417")
                    ICertificateQuery : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EnhancedKeyUsages(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IssuerName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IssuerName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Thumbprint(
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Thumbprint(
                            UINT32 valueLength,
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareOnly(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HardwareOnly(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateQuery = __uuidof(ICertificateQuery);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateQuery2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateQuery2[] = L"Windows.Security.Cryptography.Certificates.ICertificateQuery2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("935a0af7-0bd9-4f75-b8c2-e27a7f74eecd")
                    ICertificateQuery2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IncludeDuplicates(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncludeDuplicates(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncludeExpiredCertificates(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncludeExpiredCertificates(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StoreName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StoreName(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateQuery2 = __uuidof(ICertificateQuery2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("487e84f6-94e2-4dce-8833-1a700a37a29a")
                    ICertificateRequestProperties : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Subject(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyAlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyAlgorithmName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeySize(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeySize(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HashAlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HashAlgorithmName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Exportable(
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Exportable(
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyUsages(
                            ABI::Windows::Security::Cryptography::Certificates::EnrollKeyUsages* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyUsages(
                            ABI::Windows::Security::Cryptography::Certificates::EnrollKeyUsages value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyProtectionLevel(
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyProtectionLevel(
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyStorageProviderName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyStorageProviderName(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateRequestProperties = __uuidof(ICertificateRequestProperties);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties2[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("3da0c954-d73f-4ff3-a0a6-0677c0ada05b")
                    ICertificateRequestProperties2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SmartcardReaderName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SmartcardReaderName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SigningCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SigningCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AttestationCredentialCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AttestationCredentialCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateRequestProperties2 = __uuidof(ICertificateRequestProperties2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties3[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("e687f616-734d-46b1-9d4c-6edfdbfc845b")
                    ICertificateRequestProperties3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CurveName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CurveName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurveParameters(
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CurveParameters(
                            UINT32 valueLength,
                            BYTE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContainerNamePrefix(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContainerNamePrefix(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContainerName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContainerName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UseExistingKey(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseExistingKey(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateRequestProperties3 = __uuidof(ICertificateRequestProperties3);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties4[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("4e429ad2-1c61-4fea-b8fe-135fb19cdce4")
                    ICertificateRequestProperties4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SuppressedDefaults(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SubjectAlternativeName(
                            ABI::Windows::Security::Cryptography::Certificates::ISubjectAlternativeNameInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Extensions(
                            __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateRequestProperties4 = __uuidof(ICertificateRequestProperties4);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStore[] = L"Windows.Security.Cryptography.Certificates.ICertificateStore";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("b0bff720-344e-4331-af14-a7f7a7ebc93a")
                    ICertificateStore : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Add(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Delete(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateStore = __uuidof(ICertificateStore);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStore2[] = L"Windows.Security.Cryptography.Certificates.ICertificateStore2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("c7e68e4a-417d-4d1a-babd-15687e549974")
                    ICertificateStore2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateStore2 = __uuidof(ICertificateStore2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStoresStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStores
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStoresStatics[] = L"Windows.Security.Cryptography.Certificates.ICertificateStoresStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("fbecc739-c6fe-4de7-99cf-74c3e596e032")
                    ICertificateStoresStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindAllWithQueryAsync(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateQuery* query,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TrustedRootCertificationAuthorities(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateStore** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IntermediateCertificationAuthorities(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateStore** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStoreByName(
                            HSTRING storeName,
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateStore** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateStoresStatics = __uuidof(ICertificateStoresStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStores
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStoresStatics2[] = L"Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("fa900b79-a0d4-4b8c-bc55-c0a37eb141ed")
                    ICertificateStoresStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetUserStoreByName(
                            HSTRING storeName,
                            ABI::Windows::Security::Cryptography::Certificates::IUserCertificateStore** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICertificateStoresStatics2 = __uuidof(ICertificateStoresStatics2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IChainBuildingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.ChainBuildingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IChainBuildingParameters[] = L"Windows.Security.Cryptography.Certificates.IChainBuildingParameters";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("422ba922-7c8d-47b7-b59b-b12703733ac3")
                    IChainBuildingParameters : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EnhancedKeyUsages(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValidationTimestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ValidationTimestamp(
                            ABI::Windows::Foundation::DateTime value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RevocationCheckEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RevocationCheckEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NetworkRetrievalEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NetworkRetrievalEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthorityInformationAccessEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AuthorityInformationAccessEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentTimeValidationEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CurrentTimeValidationEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExclusiveTrustRoots(
                            __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** certificates
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IChainBuildingParameters = __uuidof(IChainBuildingParameters);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IChainValidationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.ChainValidationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IChainValidationParameters[] = L"Windows.Security.Cryptography.Certificates.IChainValidationParameters";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("c4743b4a-7eb0-4b56-a040-b9c8e655ddf3")
                    IChainValidationParameters : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CertificateChainPolicy(
                            ABI::Windows::Security::Cryptography::Certificates::CertificateChainPolicy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CertificateChainPolicy(
                            ABI::Windows::Security::Cryptography::Certificates::CertificateChainPolicy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerDnsName(
                            ABI::Windows::Networking::IHostName** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ServerDnsName(
                            ABI::Windows::Networking::IHostName* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IChainValidationParameters = __uuidof(IChainValidationParameters);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsAttachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsAttachedSignature[] = L"Windows.Security.Cryptography.Certificates.ICmsAttachedSignature";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("61899d9d-3757-4ecb-bddc-0ca357d7a936")
                    ICmsAttachedSignature : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Certificates(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Content(
                            UINT32* valueLength,
                            BYTE** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Signers(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE VerifySignature(
                            ABI::Windows::Security::Cryptography::Certificates::SignatureValidationResult* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsAttachedSignature = __uuidof(ICmsAttachedSignature);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsAttachedSignatureFactory[] = L"Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("d0c8fc15-f757-4c64-a362-52cc1c77cffb")
                    ICmsAttachedSignatureFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateCmsAttachedSignature(
                            ABI::Windows::Storage::Streams::IBuffer* inputBlob,
                            ABI::Windows::Security::Cryptography::Certificates::ICmsAttachedSignature** cmsSignedData
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsAttachedSignatureFactory = __uuidof(ICmsAttachedSignatureFactory);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsAttachedSignatureStatics[] = L"Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("87989c8e-b0ad-498d-a7f5-78b59bce4b36")
                    ICmsAttachedSignatureStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GenerateSignatureAsync(
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* signers,
                            __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** outputBlob
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsAttachedSignatureStatics = __uuidof(ICmsAttachedSignatureStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsDetachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsDetachedSignature[] = L"Windows.Security.Cryptography.Certificates.ICmsDetachedSignature";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("0f1ef154-f65e-4536-8339-5944081db2ca")
                    ICmsDetachedSignature : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Certificates(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Signers(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE VerifySignatureAsync(
                            ABI::Windows::Storage::Streams::IInputStream* data,
                            __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsDetachedSignature = __uuidof(ICmsDetachedSignature);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsDetachedSignatureFactory[] = L"Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("c4ab3503-ae7f-4387-ad19-00f150e48ebb")
                    ICmsDetachedSignatureFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateCmsDetachedSignature(
                            ABI::Windows::Storage::Streams::IBuffer* inputBlob,
                            ABI::Windows::Security::Cryptography::Certificates::ICmsDetachedSignature** cmsSignedData
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsDetachedSignatureFactory = __uuidof(ICmsDetachedSignatureFactory);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsDetachedSignatureStatics[] = L"Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("3d114cfd-bf9b-4682-9be6-91f57c053808")
                    ICmsDetachedSignatureStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GenerateSignatureAsync(
                            ABI::Windows::Storage::Streams::IInputStream* data,
                            __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* signers,
                            __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** outputBlob
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsDetachedSignatureStatics = __uuidof(ICmsDetachedSignatureStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsSignerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsSignerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsSignerInfo[] = L"Windows.Security.Cryptography.Certificates.ICmsSignerInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("50d020db-1d2f-4c1a-b5c5-d0188ff91f47")
                    ICmsSignerInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Certificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Certificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HashAlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HashAlgorithmName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TimestampInfo(
                            ABI::Windows::Security::Cryptography::Certificates::ICmsTimestampInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsSignerInfo = __uuidof(ICmsSignerInfo);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsTimestampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsTimestampInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsTimestampInfo[] = L"Windows.Security.Cryptography.Certificates.ICmsTimestampInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("2f5f00f2-2c18-4f88-8435-c534086076f5")
                    ICmsTimestampInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SigningCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Certificates(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICmsTimestampInfo = __uuidof(ICmsTimestampInfo);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("479065d7-7ac7-4581-8c3b-d07027140448")
                    IKeyAlgorithmNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Rsa(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Dsa(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdh256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdh384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdh521(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdsa256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdsa384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdsa521(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyAlgorithmNamesStatics = __uuidof(IKeyAlgorithmNamesStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAlgorithmNamesStatics2[] = L"Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("c99b5686-e1fd-4a4a-893d-a26f33dd8bb4")
                    IKeyAlgorithmNamesStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdsa(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ecdh(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyAlgorithmNamesStatics2 = __uuidof(IKeyAlgorithmNamesStatics2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAttestationHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAttestationHelperStatics[] = L"Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("1648e246-f644-4326-88be-3af102d30e0c")
                    IKeyAttestationHelperStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE DecryptTpmAttestationCredentialAsync(
                            HSTRING credential,
                            __FIAsyncOperation_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetTpmAttestationCredentialId(
                            HSTRING credential,
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyAttestationHelperStatics = __uuidof(IKeyAttestationHelperStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAttestationHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAttestationHelperStatics2[] = L"Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("9c590b2c-a6c6-4a5e-9e64-e85d5279df97")
                    IKeyAttestationHelperStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE DecryptTpmAttestationCredentialWithContainerNameAsync(
                            HSTRING credential,
                            HSTRING containerName,
                            __FIAsyncOperation_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyAttestationHelperStatics2 = __uuidof(IKeyAttestationHelperStatics2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyStorageProviderNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyStorageProviderNamesStatics[] = L"Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("af186ae0-5529-4602-bd94-0aab91957b5c")
                    IKeyStorageProviderNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SoftwareKeyStorageProvider(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SmartcardKeyStorageProvider(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PlatformKeyStorageProvider(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyStorageProviderNamesStatics = __uuidof(IKeyStorageProviderNamesStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyStorageProviderNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyStorageProviderNamesStatics2[] = L"Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("262d743d-9c2e-41cc-8812-c4d971dd7c60")
                    IKeyStorageProviderNamesStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PassportKeyStorageProvider(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyStorageProviderNamesStatics2 = __uuidof(IKeyStorageProviderNamesStatics2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IPfxImportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.PfxImportParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IPfxImportParameters[] = L"Windows.Security.Cryptography.Certificates.IPfxImportParameters";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("680d3511-9a08-47c8-864a-2edd4d8eb46c")
                    IPfxImportParameters : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Exportable(
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Exportable(
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyProtectionLevel(
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyProtectionLevel(
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InstallOptions(
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InstallOptions(
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyStorageProviderName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyStorageProviderName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContainerNamePrefix(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContainerNamePrefix(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ReaderName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ReaderName(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPfxImportParameters = __uuidof(IPfxImportParameters);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IStandardCertificateStoreNamesStatics[] = L"Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("0c154adb-a496-41f8-8fe5-9e96f36efbf8")
                    IStandardCertificateStoreNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Personal(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TrustedRootCertificationAuthorities(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IntermediateCertificationAuthorities(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStandardCertificateStoreNamesStatics = __uuidof(IStandardCertificateStoreNamesStatics);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ISubjectAlternativeNameInfo[] = L"Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("582859f1-569d-4c20-be7b-4e1c9a0bc52b")
                    ISubjectAlternativeNameInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailName(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IPAddress(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Url(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DnsName(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DistinguishedName(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PrincipalName(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISubjectAlternativeNameInfo = __uuidof(ISubjectAlternativeNameInfo);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ISubjectAlternativeNameInfo2[] = L"Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("437a78c6-1c51-41ea-b34a-3d654398a370")
                    ISubjectAlternativeNameInfo2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailNames(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IPAddresses(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Urls(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DnsNames(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DistinguishedNames(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PrincipalNames(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Extension(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateExtension** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISubjectAlternativeNameInfo2 = __uuidof(ISubjectAlternativeNameInfo2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IUserCertificateEnrollmentManager[] = L"Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("96313718-22e1-4819-b20b-ab46a6eca06e")
                    IUserCertificateEnrollmentManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateRequestAsync(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificateRequestProperties* request,
                            __FIAsyncOperation_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE InstallCertificateAsync(
                            HSTRING certificate,
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions installOption,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportPfxDataAsync(
                            HSTRING pfxData,
                            HSTRING password,
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption exportable,
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel keyProtectionLevel,
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions installOption,
                            HSTRING friendlyName,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportPfxDataToKspAsync(
                            HSTRING pfxData,
                            HSTRING password,
                            ABI::Windows::Security::Cryptography::Certificates::ExportOption exportable,
                            ABI::Windows::Security::Cryptography::Certificates::KeyProtectionLevel keyProtectionLevel,
                            ABI::Windows::Security::Cryptography::Certificates::InstallOptions installOption,
                            HSTRING friendlyName,
                            HSTRING keyStorageProvider,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserCertificateEnrollmentManager = __uuidof(IUserCertificateEnrollmentManager);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IUserCertificateEnrollmentManager2[] = L"Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("0dad9cb1-65de-492a-b86d-fc5c482c3747")
                    IUserCertificateEnrollmentManager2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ImportPfxDataToKspWithParametersAsync(
                            HSTRING pfxData,
                            HSTRING password,
                            ABI::Windows::Security::Cryptography::Certificates::IPfxImportParameters* pfxImportParameters,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserCertificateEnrollmentManager2 = __uuidof(IUserCertificateEnrollmentManager2);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IUserCertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.UserCertificateStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IUserCertificateStore[] = L"Windows.Security.Cryptography.Certificates.IUserCertificateStore";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    MIDL_INTERFACE("c9fb1d83-789f-4b4e-9180-045a757aac6d")
                    IUserCertificateStore : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestAddAsync(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate,
                            __FIAsyncOperation_1_boolean** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestDeleteAsync(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate,
                            __FIAsyncOperation_1_boolean** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserCertificateStore = __uuidof(IUserCertificateStore);
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.Certificate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Cryptography.Certificates.ICertificateFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificate ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificate2
 *    Windows.Security.Cryptography.Certificates.ICertificate3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_Certificate_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_Certificate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_Certificate[] = L"Windows.Security.Cryptography.Certificates.Certificate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateChain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateChain ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateChain_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateChain_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateChain[] = L"Windows.Security.Cryptography.Certificates.CertificateChain";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateEnrollmentManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateEnrollmentManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateEnrollmentManager[] = L"Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateExtension ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateExtension_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateExtension[] = L"Windows.Security.Cryptography.Certificates.CertificateExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateKeyUsages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateKeyUsages ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateKeyUsages_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateKeyUsages_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateKeyUsages[] = L"Windows.Security.Cryptography.Certificates.CertificateKeyUsages";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateQuery ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificateQuery2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateQuery_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateQuery[] = L"Windows.Security.Cryptography.Certificates.CertificateQuery";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateRequestProperties_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateRequestProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateRequestProperties[] = L"Windows.Security.Cryptography.Certificates.CertificateRequestProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateStore ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificateStore2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStore_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateStore[] = L"Windows.Security.Cryptography.Certificates.CertificateStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateStores
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateStoresStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStores_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStores_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateStores[] = L"Windows.Security.Cryptography.Certificates.CertificateStores";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.ChainBuildingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IChainBuildingParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainBuildingParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainBuildingParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_ChainBuildingParameters[] = L"Windows.Security.Cryptography.Certificates.ChainBuildingParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.ChainValidationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IChainValidationParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainValidationParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainValidationParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_ChainValidationParameters[] = L"Windows.Security.Cryptography.Certificates.ChainValidationParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsAttachedSignature ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsAttachedSignature_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsAttachedSignature_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsAttachedSignature[] = L"Windows.Security.Cryptography.Certificates.CmsAttachedSignature";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsDetachedSignature ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsDetachedSignature_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsDetachedSignature_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsDetachedSignature[] = L"Windows.Security.Cryptography.Certificates.CmsDetachedSignature";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsSignerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsSignerInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsSignerInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsSignerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsSignerInfo[] = L"Windows.Security.Cryptography.Certificates.CmsSignerInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsTimestampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsTimestampInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsTimestampInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsTimestampInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsTimestampInfo[] = L"Windows.Security.Cryptography.Certificates.CmsTimestampInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.KeyAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_KeyAlgorithmNames[] = L"Windows.Security.Cryptography.Certificates.KeyAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.KeyAttestationHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAttestationHelper_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAttestationHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_KeyAttestationHelper[] = L"Windows.Security.Cryptography.Certificates.KeyAttestationHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.KeyStorageProviderNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyStorageProviderNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyStorageProviderNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_KeyStorageProviderNames[] = L"Windows.Security.Cryptography.Certificates.KeyStorageProviderNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.PfxImportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IPfxImportParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_PfxImportParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_PfxImportParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_PfxImportParameters[] = L"Windows.Security.Cryptography.Certificates.PfxImportParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_StandardCertificateStoreNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_StandardCertificateStoreNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_StandardCertificateStoreNames[] = L"Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_SubjectAlternativeNameInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_SubjectAlternativeNameInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_SubjectAlternativeNameInfo[] = L"Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateEnrollmentManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateEnrollmentManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_UserCertificateEnrollmentManager[] = L"Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.UserCertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IUserCertificateStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateStore_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_UserCertificateStore[] = L"Windows.Security.Cryptography.Certificates.UserCertificateStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2 __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChainVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChainVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChainVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChainVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChainVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChainVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CSignatureValidationResult __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CSignatureValidationResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CSignatureValidationResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items);

    END_INTERFACE
} __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension;

typedef struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** items);

    END_INTERFACE
} __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl;

interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension
{
    CONST_VTBL struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CCertificateChainPolicy __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CCertificateChainPolicy;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CEnrollKeyUsages __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CEnrollKeyUsages;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel;

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.CertificateChainPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CCertificateChainPolicy
{
    CertificateChainPolicy_Base = 0,
    CertificateChainPolicy_Ssl = 1,
    CertificateChainPolicy_NTAuthentication = 2,
    CertificateChainPolicy_MicrosoftRoot = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.ChainValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult
{
    ChainValidationResult_Success = 0,
    ChainValidationResult_Untrusted = 1,
    ChainValidationResult_Revoked = 2,
    ChainValidationResult_Expired = 3,
    ChainValidationResult_IncompleteChain = 4,
    ChainValidationResult_InvalidSignature = 5,
    ChainValidationResult_WrongUsage = 6,
    ChainValidationResult_InvalidName = 7,
    ChainValidationResult_InvalidCertificateAuthorityPolicy = 8,
    ChainValidationResult_BasicConstraintsError = 9,
    ChainValidationResult_UnknownCriticalExtension = 10,
    ChainValidationResult_RevocationInformationMissing = 11,
    ChainValidationResult_RevocationFailure = 12,
    ChainValidationResult_OtherErrors = 13,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.EnrollKeyUsages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CEnrollKeyUsages
{
    EnrollKeyUsages_None = 0,
    EnrollKeyUsages_Decryption = 0x1,
    EnrollKeyUsages_Signing = 0x2,
    EnrollKeyUsages_KeyAgreement = 0x4,
    EnrollKeyUsages_All = 0xffffff,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.ExportOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption
{
    ExportOption_NotExportable = 0,
    ExportOption_Exportable = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.InstallOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions
{
    InstallOptions_None = 0,
    InstallOptions_DeleteExpired = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.KeyProtectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel
{
    KeyProtectionLevel_NoConsent = 0,
    KeyProtectionLevel_ConsentOnly = 1,
    KeyProtectionLevel_ConsentWithPassword = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    KeyProtectionLevel_ConsentWithFingerprint = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.KeySize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeySize
{
    KeySize_Invalid = 0,
    KeySize_Rsa2048 = 2048,
    KeySize_Rsa4096 = 4096,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Certificates.SignatureValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CSignatureValidationResult
{
    SignatureValidationResult_Success = 0,
    SignatureValidationResult_InvalidParameter = 1,
    SignatureValidationResult_BadMessage = 2,
    SignatureValidationResult_InvalidSignature = 3,
    SignatureValidationResult_OtherErrors = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificate[] = L"Windows.Security.Cryptography.Certificates.ICertificate";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BuildChainAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain** value);
    HRESULT (STDMETHODCALLTYPE* BuildChainWithParametersAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* parameters,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateChain** value);
    HRESULT (STDMETHODCALLTYPE* get_SerialNumber)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* GetHashValue)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* GetHashValueWithAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        HSTRING hashAlgorithmName,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* GetCertificateBlob)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Issuer)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HasPrivateKey)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStronglyProtected)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ValidFrom)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ValidTo)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_EnhancedKeyUsages)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* put_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_BuildChainAsync(This, certificates, value) \
    ((This)->lpVtbl->BuildChainAsync(This, certificates, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_BuildChainWithParametersAsync(This, certificates, parameters, value) \
    ((This)->lpVtbl->BuildChainWithParametersAsync(This, certificates, parameters, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_SerialNumber(This, valueLength, value) \
    ((This)->lpVtbl->get_SerialNumber(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_GetHashValue(This, valueLength, value) \
    ((This)->lpVtbl->GetHashValue(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_GetHashValueWithAlgorithm(This, hashAlgorithmName, valueLength, value) \
    ((This)->lpVtbl->GetHashValueWithAlgorithm(This, hashAlgorithmName, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_GetCertificateBlob(This, value) \
    ((This)->lpVtbl->GetCertificateBlob(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_Issuer(This, value) \
    ((This)->lpVtbl->get_Issuer(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_HasPrivateKey(This, value) \
    ((This)->lpVtbl->get_HasPrivateKey(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_IsStronglyProtected(This, value) \
    ((This)->lpVtbl->get_IsStronglyProtected(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_ValidFrom(This, value) \
    ((This)->lpVtbl->get_ValidFrom(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_ValidTo(This, value) \
    ((This)->lpVtbl->get_ValidTo(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_EnhancedKeyUsages(This, value) \
    ((This)->lpVtbl->get_EnhancedKeyUsages(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_put_FriendlyName(This, value) \
    ((This)->lpVtbl->put_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificate2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificate2[] = L"Windows.Security.Cryptography.Certificates.ICertificate2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSecurityDeviceBound)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyUsages)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages** value);
    HRESULT (STDMETHODCALLTYPE* get_KeyAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SignatureAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SignatureHashAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubjectAlternativeName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_get_IsSecurityDeviceBound(This, value) \
    ((This)->lpVtbl->get_IsSecurityDeviceBound(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_get_KeyUsages(This, value) \
    ((This)->lpVtbl->get_KeyUsages(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_get_KeyAlgorithmName(This, value) \
    ((This)->lpVtbl->get_KeyAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_get_SignatureAlgorithmName(This, value) \
    ((This)->lpVtbl->get_SignatureAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_get_SignatureHashAlgorithmName(This, value) \
    ((This)->lpVtbl->get_SignatureHashAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_get_SubjectAlternativeName(This, value) \
    ((This)->lpVtbl->get_SubjectAlternativeName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificate3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificate3[] = L"Windows.Security.Cryptography.Certificates.ICertificate3";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPerUser)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_StoreName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStorageProviderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_get_IsPerUser(This, value) \
    ((This)->lpVtbl->get_IsPerUser(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_get_StoreName(This, value) \
    ((This)->lpVtbl->get_StoreName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_get_KeyStorageProviderName(This, value) \
    ((This)->lpVtbl->get_KeyStorageProviderName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateChain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateChain
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateChain[] = L"Windows.Security.Cryptography.Certificates.ICertificateChain";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChainVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Validate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* status);
    HRESULT (STDMETHODCALLTYPE* ValidateWithParameters)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* parameter,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* status);
    HRESULT (STDMETHODCALLTYPE* GetCertificates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain* This,
        boolean includeRoot,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** certificates);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChainVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChainVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_Validate(This, status) \
    ((This)->lpVtbl->Validate(This, status))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_ValidateWithParameters(This, parameter, status) \
    ((This)->lpVtbl->ValidateWithParameters(This, parameter, status))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_GetCertificates(This, includeRoot, certificates) \
    ((This)->lpVtbl->GetCertificates(This, includeRoot, certificates))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateChain_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateEnrollmentManagerStatics[] = L"Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateRequestAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* request,
        __FIAsyncOperation_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* InstallCertificateAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        HSTRING certificate,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions installOption,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* ImportPfxDataAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics* This,
        HSTRING pfxData,
        HSTRING password,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption exportable,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel keyProtectionLevel,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions installOption,
        HSTRING friendlyName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_CreateRequestAsync(This, request, value) \
    ((This)->lpVtbl->CreateRequestAsync(This, request, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_InstallCertificateAsync(This, certificate, installOption, value) \
    ((This)->lpVtbl->InstallCertificateAsync(This, certificate, installOption, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_ImportPfxDataAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, value) \
    ((This)->lpVtbl->ImportPfxDataAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateEnrollmentManagerStatics2[] = L"Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserCertificateEnrollmentManager)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager** value);
    HRESULT (STDMETHODCALLTYPE* ImportPfxDataToKspAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2* This,
        HSTRING pfxData,
        HSTRING password,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption exportable,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel keyProtectionLevel,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions installOption,
        HSTRING friendlyName,
        HSTRING keyStorageProvider,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_get_UserCertificateEnrollmentManager(This, value) \
    ((This)->lpVtbl->get_UserCertificateEnrollmentManager(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_ImportPfxDataToKspAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, keyStorageProvider, value) \
    ((This)->lpVtbl->ImportPfxDataToKspAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, keyStorageProvider, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateEnrollmentManagerStatics3[] = L"Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ImportPfxDataToKspWithParametersAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3* This,
        HSTRING pfxData,
        HSTRING password,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* pfxImportParameters,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_ImportPfxDataToKspWithParametersAsync(This, pfxData, password, pfxImportParameters, value) \
    ((This)->lpVtbl->ImportPfxDataToKspWithParametersAsync(This, pfxData, password, pfxImportParameters, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateEnrollmentManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateExtension[] = L"Windows.Security.Cryptography.Certificates.ICertificateExtension";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ObjectId)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ObjectId)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsCritical)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCritical)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* EncodeValue)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension* This,
        UINT32 valueLength,
        BYTE* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtensionVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_get_ObjectId(This, value) \
    ((This)->lpVtbl->get_ObjectId(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_put_ObjectId(This, value) \
    ((This)->lpVtbl->put_ObjectId(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_get_IsCritical(This, value) \
    ((This)->lpVtbl->get_IsCritical(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_put_IsCritical(This, value) \
    ((This)->lpVtbl->put_IsCritical(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_EncodeValue(This, value) \
    ((This)->lpVtbl->EncodeValue(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_get_Value(This, valueLength, value) \
    ((This)->lpVtbl->get_Value(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_put_Value(This, valueLength, value) \
    ((This)->lpVtbl->put_Value(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.Certificate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateFactory[] = L"Windows.Security.Cryptography.Certificates.ICertificateFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* certBlob,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** certificate);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_CreateCertificate(This, certBlob, certificate) \
    ((This)->lpVtbl->CreateCertificate(This, certBlob, certificate))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateKeyUsages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateKeyUsages
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateKeyUsages[] = L"Windows.Security.Cryptography.Certificates.ICertificateKeyUsages";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsagesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EncipherOnly)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_EncipherOnly)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CrlSign)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CrlSign)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KeyCertificateSign)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyCertificateSign)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KeyAgreement)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyAgreement)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DataEncipherment)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DataEncipherment)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KeyEncipherment)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyEncipherment)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_NonRepudiation)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_NonRepudiation)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DigitalSignature)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DigitalSignature)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsagesVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsagesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_EncipherOnly(This, value) \
    ((This)->lpVtbl->get_EncipherOnly(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_EncipherOnly(This, value) \
    ((This)->lpVtbl->put_EncipherOnly(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_CrlSign(This, value) \
    ((This)->lpVtbl->get_CrlSign(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_CrlSign(This, value) \
    ((This)->lpVtbl->put_CrlSign(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_KeyCertificateSign(This, value) \
    ((This)->lpVtbl->get_KeyCertificateSign(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_KeyCertificateSign(This, value) \
    ((This)->lpVtbl->put_KeyCertificateSign(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_KeyAgreement(This, value) \
    ((This)->lpVtbl->get_KeyAgreement(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_KeyAgreement(This, value) \
    ((This)->lpVtbl->put_KeyAgreement(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_DataEncipherment(This, value) \
    ((This)->lpVtbl->get_DataEncipherment(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_DataEncipherment(This, value) \
    ((This)->lpVtbl->put_DataEncipherment(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_KeyEncipherment(This, value) \
    ((This)->lpVtbl->get_KeyEncipherment(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_KeyEncipherment(This, value) \
    ((This)->lpVtbl->put_KeyEncipherment(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_NonRepudiation(This, value) \
    ((This)->lpVtbl->get_NonRepudiation(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_NonRepudiation(This, value) \
    ((This)->lpVtbl->put_NonRepudiation(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_get_DigitalSignature(This, value) \
    ((This)->lpVtbl->get_DigitalSignature(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_put_DigitalSignature(This, value) \
    ((This)->lpVtbl->put_DigitalSignature(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateKeyUsages_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateQuery[] = L"Windows.Security.Cryptography.Certificates.ICertificateQuery";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQueryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnhancedKeyUsages)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_IssuerName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_IssuerName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbprint)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* put_Thumbprint)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareOnly)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_HardwareOnly)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQueryVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQueryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_get_EnhancedKeyUsages(This, value) \
    ((This)->lpVtbl->get_EnhancedKeyUsages(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_get_IssuerName(This, value) \
    ((This)->lpVtbl->get_IssuerName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_put_IssuerName(This, value) \
    ((This)->lpVtbl->put_IssuerName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_put_FriendlyName(This, value) \
    ((This)->lpVtbl->put_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_get_Thumbprint(This, valueLength, value) \
    ((This)->lpVtbl->get_Thumbprint(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_put_Thumbprint(This, valueLength, value) \
    ((This)->lpVtbl->put_Thumbprint(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_get_HardwareOnly(This, value) \
    ((This)->lpVtbl->get_HardwareOnly(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_put_HardwareOnly(This, value) \
    ((This)->lpVtbl->put_HardwareOnly(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateQuery2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateQuery2[] = L"Windows.Security.Cryptography.Certificates.ICertificateQuery2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IncludeDuplicates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeDuplicates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeExpiredCertificates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeExpiredCertificates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StoreName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_StoreName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_get_IncludeDuplicates(This, value) \
    ((This)->lpVtbl->get_IncludeDuplicates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_put_IncludeDuplicates(This, value) \
    ((This)->lpVtbl->put_IncludeDuplicates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_get_IncludeExpiredCertificates(This, value) \
    ((This)->lpVtbl->get_IncludeExpiredCertificates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_put_IncludeExpiredCertificates(This, value) \
    ((This)->lpVtbl->put_IncludeExpiredCertificates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_get_StoreName(This, value) \
    ((This)->lpVtbl->get_StoreName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_put_StoreName(This, value) \
    ((This)->lpVtbl->put_StoreName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subject)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_KeyAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_KeySize)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_KeySize)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_HashAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_HashAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Exportable)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption* value);
    HRESULT (STDMETHODCALLTYPE* put_Exportable)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption value);
    HRESULT (STDMETHODCALLTYPE* get_KeyUsages)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CEnrollKeyUsages* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyUsages)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CEnrollKeyUsages value);
    HRESULT (STDMETHODCALLTYPE* get_KeyProtectionLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyProtectionLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStorageProviderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyStorageProviderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestPropertiesVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_Subject(This, value) \
    ((This)->lpVtbl->put_Subject(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_KeyAlgorithmName(This, value) \
    ((This)->lpVtbl->get_KeyAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_KeyAlgorithmName(This, value) \
    ((This)->lpVtbl->put_KeyAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_KeySize(This, value) \
    ((This)->lpVtbl->get_KeySize(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_KeySize(This, value) \
    ((This)->lpVtbl->put_KeySize(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_FriendlyName(This, value) \
    ((This)->lpVtbl->put_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_HashAlgorithmName(This, value) \
    ((This)->lpVtbl->get_HashAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_HashAlgorithmName(This, value) \
    ((This)->lpVtbl->put_HashAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_Exportable(This, value) \
    ((This)->lpVtbl->get_Exportable(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_Exportable(This, value) \
    ((This)->lpVtbl->put_Exportable(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_KeyUsages(This, value) \
    ((This)->lpVtbl->get_KeyUsages(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_KeyUsages(This, value) \
    ((This)->lpVtbl->put_KeyUsages(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_KeyProtectionLevel(This, value) \
    ((This)->lpVtbl->get_KeyProtectionLevel(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_KeyProtectionLevel(This, value) \
    ((This)->lpVtbl->put_KeyProtectionLevel(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_get_KeyStorageProviderName(This, value) \
    ((This)->lpVtbl->get_KeyStorageProviderName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_put_KeyStorageProviderName(This, value) \
    ((This)->lpVtbl->put_KeyStorageProviderName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties2[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SmartcardReaderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SmartcardReaderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SigningCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_SigningCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);
    HRESULT (STDMETHODCALLTYPE* get_AttestationCredentialCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_AttestationCredentialCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_get_SmartcardReaderName(This, value) \
    ((This)->lpVtbl->get_SmartcardReaderName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_put_SmartcardReaderName(This, value) \
    ((This)->lpVtbl->put_SmartcardReaderName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_get_SigningCertificate(This, value) \
    ((This)->lpVtbl->get_SigningCertificate(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_put_SigningCertificate(This, value) \
    ((This)->lpVtbl->put_SigningCertificate(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_get_AttestationCredentialCertificate(This, value) \
    ((This)->lpVtbl->get_AttestationCredentialCertificate(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_put_AttestationCredentialCertificate(This, value) \
    ((This)->lpVtbl->put_AttestationCredentialCertificate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties3[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurveName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CurveName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CurveParameters)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* put_CurveParameters)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* get_ContainerNamePrefix)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContainerNamePrefix)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContainerName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContainerName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_UseExistingKey)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseExistingKey)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_get_CurveName(This, value) \
    ((This)->lpVtbl->get_CurveName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_put_CurveName(This, value) \
    ((This)->lpVtbl->put_CurveName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_get_CurveParameters(This, valueLength, value) \
    ((This)->lpVtbl->get_CurveParameters(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_put_CurveParameters(This, valueLength, value) \
    ((This)->lpVtbl->put_CurveParameters(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_get_ContainerNamePrefix(This, value) \
    ((This)->lpVtbl->get_ContainerNamePrefix(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_put_ContainerNamePrefix(This, value) \
    ((This)->lpVtbl->put_ContainerNamePrefix(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_get_ContainerName(This, value) \
    ((This)->lpVtbl->get_ContainerName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_put_ContainerName(This, value) \
    ((This)->lpVtbl->put_ContainerName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_get_UseExistingKey(This, value) \
    ((This)->lpVtbl->get_UseExistingKey(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_put_UseExistingKey(This, value) \
    ((This)->lpVtbl->put_UseExistingKey(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateRequestProperties4[] = L"Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SuppressedDefaults)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SubjectAlternativeName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_Extensions)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4* This,
        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateExtension** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_get_SuppressedDefaults(This, value) \
    ((This)->lpVtbl->get_SuppressedDefaults(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_get_SubjectAlternativeName(This, value) \
    ((This)->lpVtbl->get_SubjectAlternativeName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_get_Extensions(This, value) \
    ((This)->lpVtbl->get_Extensions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStore[] = L"Windows.Security.Cryptography.Certificates.ICertificateStore";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Add)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate);
    HRESULT (STDMETHODCALLTYPE* Delete)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoreVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_Add(This, certificate) \
    ((This)->lpVtbl->Add(This, certificate))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_Delete(This, certificate) \
    ((This)->lpVtbl->Delete(This, certificate))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStore2[] = L"Windows.Security.Cryptography.Certificates.ICertificateStore2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStoresStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStores
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStoresStatics[] = L"Windows.Security.Cryptography.Certificates.ICertificateStoresStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* FindAllWithQueryAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateQuery* query,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_TrustedRootCertificationAuthorities)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore** value);
    HRESULT (STDMETHODCALLTYPE* get_IntermediateCertificationAuthorities)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore** value);
    HRESULT (STDMETHODCALLTYPE* GetStoreByName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics* This,
        HSTRING storeName,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStore** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FindAllAsync(This, value) \
    ((This)->lpVtbl->FindAllAsync(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_FindAllWithQueryAsync(This, query, value) \
    ((This)->lpVtbl->FindAllWithQueryAsync(This, query, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_get_TrustedRootCertificationAuthorities(This, value) \
    ((This)->lpVtbl->get_TrustedRootCertificationAuthorities(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_get_IntermediateCertificationAuthorities(This, value) \
    ((This)->lpVtbl->get_IntermediateCertificationAuthorities(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_GetStoreByName(This, storeName, value) \
    ((This)->lpVtbl->GetStoreByName(This, storeName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CertificateStores
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICertificateStoresStatics2[] = L"Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetUserStoreByName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2* This,
        HSTRING storeName,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_GetUserStoreByName(This, storeName, result) \
    ((This)->lpVtbl->GetUserStoreByName(This, storeName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateStoresStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IChainBuildingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.ChainBuildingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IChainBuildingParameters[] = L"Windows.Security.Cryptography.Certificates.IChainBuildingParameters";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnhancedKeyUsages)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ValidationTimestamp)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_ValidationTimestamp)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_RevocationCheckEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RevocationCheckEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkRetrievalEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_NetworkRetrievalEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AuthorityInformationAccessEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthorityInformationAccessEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentTimeValidationEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CurrentTimeValidationEnabled)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ExclusiveTrustRoots)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters* This,
        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** certificates);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParametersVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_EnhancedKeyUsages(This, value) \
    ((This)->lpVtbl->get_EnhancedKeyUsages(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_ValidationTimestamp(This, value) \
    ((This)->lpVtbl->get_ValidationTimestamp(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_put_ValidationTimestamp(This, value) \
    ((This)->lpVtbl->put_ValidationTimestamp(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_RevocationCheckEnabled(This, value) \
    ((This)->lpVtbl->get_RevocationCheckEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_put_RevocationCheckEnabled(This, value) \
    ((This)->lpVtbl->put_RevocationCheckEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_NetworkRetrievalEnabled(This, value) \
    ((This)->lpVtbl->get_NetworkRetrievalEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_put_NetworkRetrievalEnabled(This, value) \
    ((This)->lpVtbl->put_NetworkRetrievalEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_AuthorityInformationAccessEnabled(This, value) \
    ((This)->lpVtbl->get_AuthorityInformationAccessEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_put_AuthorityInformationAccessEnabled(This, value) \
    ((This)->lpVtbl->put_AuthorityInformationAccessEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_CurrentTimeValidationEnabled(This, value) \
    ((This)->lpVtbl->get_CurrentTimeValidationEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_put_CurrentTimeValidationEnabled(This, value) \
    ((This)->lpVtbl->put_CurrentTimeValidationEnabled(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_get_ExclusiveTrustRoots(This, certificates) \
    ((This)->lpVtbl->get_ExclusiveTrustRoots(This, certificates))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainBuildingParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IChainValidationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.ChainValidationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IChainValidationParameters[] = L"Windows.Security.Cryptography.Certificates.IChainValidationParameters";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CertificateChainPolicy)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CCertificateChainPolicy* value);
    HRESULT (STDMETHODCALLTYPE* put_CertificateChainPolicy)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CCertificateChainPolicy value);
    HRESULT (STDMETHODCALLTYPE* get_ServerDnsName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_ServerDnsName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParametersVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_get_CertificateChainPolicy(This, value) \
    ((This)->lpVtbl->get_CertificateChainPolicy(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_put_CertificateChainPolicy(This, value) \
    ((This)->lpVtbl->put_CertificateChainPolicy(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_get_ServerDnsName(This, value) \
    ((This)->lpVtbl->get_ServerDnsName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_put_ServerDnsName(This, value) \
    ((This)->lpVtbl->put_ServerDnsName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIChainValidationParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsAttachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsAttachedSignature[] = L"Windows.Security.Cryptography.Certificates.ICmsAttachedSignature";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Certificates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* get_Signers)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo** value);
    HRESULT (STDMETHODCALLTYPE* VerifySignature)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CSignatureValidationResult* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_get_Certificates(This, value) \
    ((This)->lpVtbl->get_Certificates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_get_Content(This, valueLength, value) \
    ((This)->lpVtbl->get_Content(This, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_get_Signers(This, value) \
    ((This)->lpVtbl->get_Signers(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_VerifySignature(This, value) \
    ((This)->lpVtbl->VerifySignature(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsAttachedSignatureFactory[] = L"Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCmsAttachedSignature)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* inputBlob,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignature** cmsSignedData);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_CreateCmsAttachedSignature(This, inputBlob, cmsSignedData) \
    ((This)->lpVtbl->CreateCmsAttachedSignature(This, inputBlob, cmsSignedData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsAttachedSignatureStatics[] = L"Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GenerateSignatureAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* signers,
        __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** outputBlob);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_GenerateSignatureAsync(This, data, signers, certificates, outputBlob) \
    ((This)->lpVtbl->GenerateSignatureAsync(This, data, signers, certificates, outputBlob))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsAttachedSignatureStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsDetachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsDetachedSignature[] = L"Windows.Security.Cryptography.Certificates.ICmsDetachedSignature";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Certificates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_Signers)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo** value);
    HRESULT (STDMETHODCALLTYPE* VerifySignatureAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* data,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCertificates__CSignatureValidationResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_get_Certificates(This, value) \
    ((This)->lpVtbl->get_Certificates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_get_Signers(This, value) \
    ((This)->lpVtbl->get_Signers(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_VerifySignatureAsync(This, data, value) \
    ((This)->lpVtbl->VerifySignatureAsync(This, data, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsDetachedSignatureFactory[] = L"Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCmsDetachedSignature)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* inputBlob,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignature** cmsSignedData);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_CreateCmsDetachedSignature(This, inputBlob, cmsSignedData) \
    ((This)->lpVtbl->CreateCmsDetachedSignature(This, inputBlob, cmsSignedData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsDetachedSignatureStatics[] = L"Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GenerateSignatureAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* data,
        __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCmsSignerInfo* signers,
        __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* certificates,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** outputBlob);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_GenerateSignatureAsync(This, data, signers, certificates, outputBlob) \
    ((This)->lpVtbl->GenerateSignatureAsync(This, data, signers, certificates, outputBlob))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsDetachedSignatureStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsSignerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsSignerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsSignerInfo[] = L"Windows.Security.Cryptography.Certificates.ICmsSignerInfo";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Certificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_Certificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);
    HRESULT (STDMETHODCALLTYPE* get_HashAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_HashAlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_TimestampInfo)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_get_Certificate(This, value) \
    ((This)->lpVtbl->get_Certificate(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_put_Certificate(This, value) \
    ((This)->lpVtbl->put_Certificate(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_get_HashAlgorithmName(This, value) \
    ((This)->lpVtbl->get_HashAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_put_HashAlgorithmName(This, value) \
    ((This)->lpVtbl->put_HashAlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_get_TimestampInfo(This, value) \
    ((This)->lpVtbl->get_TimestampInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsSignerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ICmsTimestampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.CmsTimestampInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ICmsTimestampInfo[] = L"Windows.Security.Cryptography.Certificates.ICmsTimestampInfo";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SigningCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_Certificates)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_get_SigningCertificate(This, value) \
    ((This)->lpVtbl->get_SigningCertificate(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_get_Certificates(This, value) \
    ((This)->lpVtbl->get_Certificates(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICmsTimestampInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Rsa)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Dsa)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdh256)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdh384)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdh521)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdsa256)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdsa384)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdsa521)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Rsa(This, value) \
    ((This)->lpVtbl->get_Rsa(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Dsa(This, value) \
    ((This)->lpVtbl->get_Dsa(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Ecdh256(This, value) \
    ((This)->lpVtbl->get_Ecdh256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Ecdh384(This, value) \
    ((This)->lpVtbl->get_Ecdh384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Ecdh521(This, value) \
    ((This)->lpVtbl->get_Ecdh521(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Ecdsa256(This, value) \
    ((This)->lpVtbl->get_Ecdsa256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Ecdsa384(This, value) \
    ((This)->lpVtbl->get_Ecdsa384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_get_Ecdsa521(This, value) \
    ((This)->lpVtbl->get_Ecdsa521(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAlgorithmNamesStatics2[] = L"Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Ecdsa)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ecdh)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_get_Ecdsa(This, value) \
    ((This)->lpVtbl->get_Ecdsa(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_get_Ecdh(This, value) \
    ((This)->lpVtbl->get_Ecdh(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAlgorithmNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAttestationHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAttestationHelperStatics[] = L"Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DecryptTpmAttestationCredentialAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This,
        HSTRING credential,
        __FIAsyncOperation_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetTpmAttestationCredentialId)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics* This,
        HSTRING credential,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_DecryptTpmAttestationCredentialAsync(This, credential, value) \
    ((This)->lpVtbl->DecryptTpmAttestationCredentialAsync(This, credential, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_GetTpmAttestationCredentialId(This, credential, value) \
    ((This)->lpVtbl->GetTpmAttestationCredentialId(This, credential, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyAttestationHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyAttestationHelperStatics2[] = L"Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DecryptTpmAttestationCredentialWithContainerNameAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2* This,
        HSTRING credential,
        HSTRING containerName,
        __FIAsyncOperation_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_DecryptTpmAttestationCredentialWithContainerNameAsync(This, credential, containerName, value) \
    ((This)->lpVtbl->DecryptTpmAttestationCredentialWithContainerNameAsync(This, credential, containerName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyAttestationHelperStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyStorageProviderNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyStorageProviderNamesStatics[] = L"Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SoftwareKeyStorageProvider)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SmartcardKeyStorageProvider)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PlatformKeyStorageProvider)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_get_SoftwareKeyStorageProvider(This, value) \
    ((This)->lpVtbl->get_SoftwareKeyStorageProvider(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_get_SmartcardKeyStorageProvider(This, value) \
    ((This)->lpVtbl->get_SmartcardKeyStorageProvider(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_get_PlatformKeyStorageProvider(This, value) \
    ((This)->lpVtbl->get_PlatformKeyStorageProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.KeyStorageProviderNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IKeyStorageProviderNamesStatics2[] = L"Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PassportKeyStorageProvider)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_get_PassportKeyStorageProvider(This, value) \
    ((This)->lpVtbl->get_PassportKeyStorageProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIKeyStorageProviderNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IPfxImportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.PfxImportParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IPfxImportParameters[] = L"Windows.Security.Cryptography.Certificates.IPfxImportParameters";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Exportable)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption* value);
    HRESULT (STDMETHODCALLTYPE* put_Exportable)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption value);
    HRESULT (STDMETHODCALLTYPE* get_KeyProtectionLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyProtectionLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel value);
    HRESULT (STDMETHODCALLTYPE* get_InstallOptions)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_InstallOptions)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_FriendlyName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStorageProviderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyStorageProviderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContainerNamePrefix)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContainerNamePrefix)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ReaderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ReaderName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParametersVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_Exportable(This, value) \
    ((This)->lpVtbl->get_Exportable(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_Exportable(This, value) \
    ((This)->lpVtbl->put_Exportable(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_KeyProtectionLevel(This, value) \
    ((This)->lpVtbl->get_KeyProtectionLevel(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_KeyProtectionLevel(This, value) \
    ((This)->lpVtbl->put_KeyProtectionLevel(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_InstallOptions(This, value) \
    ((This)->lpVtbl->get_InstallOptions(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_InstallOptions(This, value) \
    ((This)->lpVtbl->put_InstallOptions(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_FriendlyName(This, value) \
    ((This)->lpVtbl->put_FriendlyName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_KeyStorageProviderName(This, value) \
    ((This)->lpVtbl->get_KeyStorageProviderName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_KeyStorageProviderName(This, value) \
    ((This)->lpVtbl->put_KeyStorageProviderName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_ContainerNamePrefix(This, value) \
    ((This)->lpVtbl->get_ContainerNamePrefix(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_ContainerNamePrefix(This, value) \
    ((This)->lpVtbl->put_ContainerNamePrefix(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_get_ReaderName(This, value) \
    ((This)->lpVtbl->get_ReaderName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_put_ReaderName(This, value) \
    ((This)->lpVtbl->put_ReaderName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IStandardCertificateStoreNamesStatics[] = L"Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Personal)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TrustedRootCertificationAuthorities)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IntermediateCertificationAuthorities)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_get_Personal(This, value) \
    ((This)->lpVtbl->get_Personal(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_get_TrustedRootCertificationAuthorities(This, value) \
    ((This)->lpVtbl->get_TrustedRootCertificationAuthorities(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_get_IntermediateCertificationAuthorities(This, value) \
    ((This)->lpVtbl->get_IntermediateCertificationAuthorities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIStandardCertificateStoreNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ISubjectAlternativeNameInfo[] = L"Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_IPAddress)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Url)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DnsName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DistinguishedName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_PrincipalName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_get_EmailName(This, value) \
    ((This)->lpVtbl->get_EmailName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_get_IPAddress(This, value) \
    ((This)->lpVtbl->get_IPAddress(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_get_Url(This, value) \
    ((This)->lpVtbl->get_Url(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_get_DnsName(This, value) \
    ((This)->lpVtbl->get_DnsName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_get_DistinguishedName(This, value) \
    ((This)->lpVtbl->get_DistinguishedName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_get_PrincipalName(This, value) \
    ((This)->lpVtbl->get_PrincipalName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_ISubjectAlternativeNameInfo2[] = L"Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailNames)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_IPAddresses)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Urls)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DnsNames)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DistinguishedNames)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_PrincipalNames)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Extension)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateExtension** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_EmailNames(This, value) \
    ((This)->lpVtbl->get_EmailNames(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_IPAddresses(This, value) \
    ((This)->lpVtbl->get_IPAddresses(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_Urls(This, value) \
    ((This)->lpVtbl->get_Urls(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_DnsNames(This, value) \
    ((This)->lpVtbl->get_DnsNames(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_DistinguishedNames(This, value) \
    ((This)->lpVtbl->get_DistinguishedNames(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_PrincipalNames(This, value) \
    ((This)->lpVtbl->get_PrincipalNames(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_get_Extension(This, value) \
    ((This)->lpVtbl->get_Extension(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CISubjectAlternativeNameInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IUserCertificateEnrollmentManager[] = L"Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateRequestAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificateRequestProperties* request,
        __FIAsyncOperation_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* InstallCertificateAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        HSTRING certificate,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions installOption,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* ImportPfxDataAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        HSTRING pfxData,
        HSTRING password,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption exportable,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel keyProtectionLevel,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions installOption,
        HSTRING friendlyName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* ImportPfxDataToKspAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager* This,
        HSTRING pfxData,
        HSTRING password,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CExportOption exportable,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CKeyProtectionLevel keyProtectionLevel,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CInstallOptions installOption,
        HSTRING friendlyName,
        HSTRING keyStorageProvider,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManagerVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_CreateRequestAsync(This, request, value) \
    ((This)->lpVtbl->CreateRequestAsync(This, request, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_InstallCertificateAsync(This, certificate, installOption, value) \
    ((This)->lpVtbl->InstallCertificateAsync(This, certificate, installOption, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_ImportPfxDataAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, value) \
    ((This)->lpVtbl->ImportPfxDataAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_ImportPfxDataToKspAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, keyStorageProvider, value) \
    ((This)->lpVtbl->ImportPfxDataToKspAsync(This, pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, keyStorageProvider, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IUserCertificateEnrollmentManager2[] = L"Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ImportPfxDataToKspWithParametersAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2* This,
        HSTRING pfxData,
        HSTRING password,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIPfxImportParameters* pfxImportParameters,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_ImportPfxDataToKspWithParametersAsync(This, pfxData, password, pfxImportParameters, value) \
    ((This)->lpVtbl->ImportPfxDataToKspWithParametersAsync(This, pfxData, password, pfxImportParameters, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateEnrollmentManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Certificates.IUserCertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Certificates.UserCertificateStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Certificates_IUserCertificateStore[] = L"Windows.Security.Cryptography.Certificates.IUserCertificateStore";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAddAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* RequestDeleteAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStoreVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_RequestAddAsync(This, certificate, result) \
    ((This)->lpVtbl->RequestAddAsync(This, certificate, result))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_RequestDeleteAsync(This, certificate, result) \
    ((This)->lpVtbl->RequestDeleteAsync(This, certificate, result))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CIUserCertificateStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.Certificate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Cryptography.Certificates.ICertificateFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificate ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificate2
 *    Windows.Security.Cryptography.Certificates.ICertificate3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_Certificate_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_Certificate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_Certificate[] = L"Windows.Security.Cryptography.Certificates.Certificate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateChain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateChain ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateChain_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateChain_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateChain[] = L"Windows.Security.Cryptography.Certificates.CertificateChain";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateEnrollmentManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateEnrollmentManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateEnrollmentManager[] = L"Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateExtension ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateExtension_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateExtension[] = L"Windows.Security.Cryptography.Certificates.CertificateExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateKeyUsages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateKeyUsages ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateKeyUsages_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateKeyUsages_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateKeyUsages[] = L"Windows.Security.Cryptography.Certificates.CertificateKeyUsages";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateQuery ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificateQuery2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateQuery_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateQuery[] = L"Windows.Security.Cryptography.Certificates.CertificateQuery";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateRequestProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3
 *    Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateRequestProperties_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateRequestProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateRequestProperties[] = L"Windows.Security.Cryptography.Certificates.CertificateRequestProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICertificateStore ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ICertificateStore2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStore_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateStore[] = L"Windows.Security.Cryptography.Certificates.CertificateStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CertificateStores
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICertificateStoresStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStores_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CertificateStores_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CertificateStores[] = L"Windows.Security.Cryptography.Certificates.CertificateStores";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.ChainBuildingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IChainBuildingParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainBuildingParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainBuildingParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_ChainBuildingParameters[] = L"Windows.Security.Cryptography.Certificates.ChainBuildingParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.ChainValidationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IChainValidationParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainValidationParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_ChainValidationParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_ChainValidationParameters[] = L"Windows.Security.Cryptography.Certificates.ChainValidationParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsAttachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsAttachedSignature ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsAttachedSignature_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsAttachedSignature_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsAttachedSignature[] = L"Windows.Security.Cryptography.Certificates.CmsAttachedSignature";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsDetachedSignature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsDetachedSignature ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsDetachedSignature_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsDetachedSignature_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsDetachedSignature[] = L"Windows.Security.Cryptography.Certificates.CmsDetachedSignature";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsSignerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsSignerInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsSignerInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsSignerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsSignerInfo[] = L"Windows.Security.Cryptography.Certificates.CmsSignerInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.CmsTimestampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ICmsTimestampInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsTimestampInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_CmsTimestampInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_CmsTimestampInfo[] = L"Windows.Security.Cryptography.Certificates.CmsTimestampInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.KeyAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_KeyAlgorithmNames[] = L"Windows.Security.Cryptography.Certificates.KeyAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.KeyAttestationHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAttestationHelper_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyAttestationHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_KeyAttestationHelper[] = L"Windows.Security.Cryptography.Certificates.KeyAttestationHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.KeyStorageProviderNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyStorageProviderNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_KeyStorageProviderNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_KeyStorageProviderNames[] = L"Windows.Security.Cryptography.Certificates.KeyStorageProviderNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.PfxImportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IPfxImportParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_PfxImportParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_PfxImportParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_PfxImportParameters[] = L"Windows.Security.Cryptography.Certificates.PfxImportParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_StandardCertificateStoreNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_StandardCertificateStoreNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_StandardCertificateStoreNames[] = L"Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_SubjectAlternativeNameInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_SubjectAlternativeNameInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_SubjectAlternativeNameInfo[] = L"Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager ** Default Interface **
 *    Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateEnrollmentManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateEnrollmentManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_UserCertificateEnrollmentManager[] = L"Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Certificates.UserCertificateStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Certificates.IUserCertificateStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateStore_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Certificates_UserCertificateStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Certificates_UserCertificateStore[] = L"Windows.Security.Cryptography.Certificates.UserCertificateStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esecurity2Ecryptography2Ecertificates_p_h__

#endif // __windows2Esecurity2Ecryptography2Ecertificates_h__
