
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
#ifndef __windows2Esecurity2Ecryptography2Ecore_h__
#define __windows2Esecurity2Ecryptography2Ecore_h__
#ifndef __windows2Esecurity2Ecryptography2Ecore_p_h__
#define __windows2Esecurity2Ecryptography2Ecore_p_h__


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
#include "Windows.Security.Cryptography.Certificates.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IAsymmetricAlgorithmNamesStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics ABI::Windows::Security::Cryptography::Core::IAsymmetricAlgorithmNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IAsymmetricAlgorithmNamesStatics2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2 ABI::Windows::Security::Cryptography::Core::IAsymmetricAlgorithmNamesStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IAsymmetricKeyAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider ABI::Windows::Security::Cryptography::Core::IAsymmetricKeyAlgorithmProvider

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IAsymmetricKeyAlgorithmProvider2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2 ABI::Windows::Security::Cryptography::Core::IAsymmetricKeyAlgorithmProvider2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IAsymmetricKeyAlgorithmProviderStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics ABI::Windows::Security::Cryptography::Core::IAsymmetricKeyAlgorithmProviderStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ICryptographicEngineStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics ABI::Windows::Security::Cryptography::Core::ICryptographicEngineStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ICryptographicEngineStatics2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2 ABI::Windows::Security::Cryptography::Core::ICryptographicEngineStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ICryptographicKey;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey ABI::Windows::Security::Cryptography::Core::ICryptographicKey

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IEccCurveNamesStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics ABI::Windows::Security::Cryptography::Core::IEccCurveNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IEncryptedAndAuthenticatedData;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData ABI::Windows::Security::Cryptography::Core::IEncryptedAndAuthenticatedData

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IHashAlgorithmNamesStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics ABI::Windows::Security::Cryptography::Core::IHashAlgorithmNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IHashAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider ABI::Windows::Security::Cryptography::Core::IHashAlgorithmProvider

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IHashAlgorithmProviderStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics ABI::Windows::Security::Cryptography::Core::IHashAlgorithmProviderStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IHashComputation;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation ABI::Windows::Security::Cryptography::Core::IHashComputation

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationAlgorithmNamesStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics ABI::Windows::Security::Cryptography::Core::IKeyDerivationAlgorithmNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationAlgorithmNamesStatics2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2 ABI::Windows::Security::Cryptography::Core::IKeyDerivationAlgorithmNamesStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider ABI::Windows::Security::Cryptography::Core::IKeyDerivationAlgorithmProvider

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationAlgorithmProviderStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics ABI::Windows::Security::Cryptography::Core::IKeyDerivationAlgorithmProviderStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationParameters;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationParameters2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2 ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationParametersStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics ABI::Windows::Security::Cryptography::Core::IKeyDerivationParametersStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IKeyDerivationParametersStatics2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2 ABI::Windows::Security::Cryptography::Core::IKeyDerivationParametersStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IMacAlgorithmNamesStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics ABI::Windows::Security::Cryptography::Core::IMacAlgorithmNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IMacAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider ABI::Windows::Security::Cryptography::Core::IMacAlgorithmProvider

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IMacAlgorithmProvider2;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2 ABI::Windows::Security::Cryptography::Core::IMacAlgorithmProvider2

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IMacAlgorithmProviderStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics ABI::Windows::Security::Cryptography::Core::IMacAlgorithmProviderStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface IPersistedKeyProviderStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics ABI::Windows::Security::Cryptography::Core::IPersistedKeyProviderStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ISymmetricAlgorithmNamesStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics ABI::Windows::Security::Cryptography::Core::ISymmetricAlgorithmNamesStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ISymmetricKeyAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider ABI::Windows::Security::Cryptography::Core::ISymmetricKeyAlgorithmProvider

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ISymmetricKeyAlgorithmProviderStatics;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics ABI::Windows::Security::Cryptography::Core::ISymmetricKeyAlgorithmProviderStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class CryptographicKey;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("81ca789b-98df-5c6a-9531-966238e3e7ae"))
IAsyncOperation<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Core::CryptographicKey*, ABI::Windows::Security::Cryptography::Core::ICryptographicKey*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Cryptography.Core.CryptographicKey>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("04ca4378-f594-5de6-a555-304f62cb4faf"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Core::CryptographicKey*, ABI::Windows::Security::Cryptography::Core::ICryptographicKey*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Cryptography.Core.CryptographicKey>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE */

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

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    typedef enum Capi1KdfTargetAlgorithm : int Capi1KdfTargetAlgorithm;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    typedef enum CryptographicPadding : int CryptographicPadding;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    typedef enum CryptographicPrivateKeyBlobType : int CryptographicPrivateKeyBlobType;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    typedef enum CryptographicPublicKeyBlobType : int CryptographicPublicKeyBlobType;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class AsymmetricKeyAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class CryptographicHash;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class EncryptedAndAuthenticatedData;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class HashAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class KeyDerivationAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class KeyDerivationParameters;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class MacAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    class SymmetricKeyAlgorithmProvider;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Cryptography.Core.Capi1KdfTargetAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    enum Capi1KdfTargetAlgorithm : int
                    {
                        Capi1KdfTargetAlgorithm_NotAes = 0,
                        Capi1KdfTargetAlgorithm_Aes = 1,
                    };
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Core.CryptographicPadding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    enum CryptographicPadding : int
                    {
                        CryptographicPadding_None = 0,
                        CryptographicPadding_RsaOaep = 1,
                        CryptographicPadding_RsaPkcs1V15 = 2,
                        CryptographicPadding_RsaPss = 3,
                    };
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Core.CryptographicPrivateKeyBlobType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    enum CryptographicPrivateKeyBlobType : int
                    {
                        CryptographicPrivateKeyBlobType_Pkcs8RawPrivateKeyInfo = 0,
                        CryptographicPrivateKeyBlobType_Pkcs1RsaPrivateKey = 1,
                        CryptographicPrivateKeyBlobType_BCryptPrivateKey = 2,
                        CryptographicPrivateKeyBlobType_Capi1PrivateKey = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        CryptographicPrivateKeyBlobType_BCryptEccFullPrivateKey = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    };
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Core.CryptographicPublicKeyBlobType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    enum CryptographicPublicKeyBlobType : int
                    {
                        CryptographicPublicKeyBlobType_X509SubjectPublicKeyInfo = 0,
                        CryptographicPublicKeyBlobType_Pkcs1RsaPublicKey = 1,
                        CryptographicPublicKeyBlobType_BCryptPublicKey = 2,
                        CryptographicPublicKeyBlobType_Capi1PublicKey = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        CryptographicPublicKeyBlobType_BCryptEccFullPublicKey = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    };
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("caf6fce4-67c0-46aa-84f9-752e77449f9b")
                    IAsymmetricAlgorithmNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RsaPkcs1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaOaepSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaOaepSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaOaepSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaOaepSha512(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EcdsaP256Sha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EcdsaP384Sha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EcdsaP521Sha512(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DsaSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DsaSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPkcs1Sha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPkcs1Sha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPkcs1Sha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPkcs1Sha512(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPssSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPssSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPssSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RsaSignPssSha512(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAsymmetricAlgorithmNamesStatics = __uuidof(IAsymmetricAlgorithmNamesStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricAlgorithmNamesStatics2[] = L"Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("f141c0d6-4bff-4f23-ba66-6045b137d5df")
                    IAsymmetricAlgorithmNamesStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EcdsaSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EcdsaSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EcdsaSha512(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAsymmetricAlgorithmNamesStatics2 = __uuidof(IAsymmetricAlgorithmNamesStatics2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("e8d2ff37-6259-4e88-b7e0-94191fde699e")
                    IAsymmetricKeyAlgorithmProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateKeyPair(
                            UINT32 keySize,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportDefaultPrivateKeyBlob(
                            ABI::Windows::Storage::Streams::IBuffer* keyBlob,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportKeyPairWithBlobType(
                            ABI::Windows::Storage::Streams::IBuffer* keyBlob,
                            ABI::Windows::Security::Cryptography::Core::CryptographicPrivateKeyBlobType BlobType,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportDefaultPublicKeyBlob(
                            ABI::Windows::Storage::Streams::IBuffer* keyBlob,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportPublicKeyWithBlobType(
                            ABI::Windows::Storage::Streams::IBuffer* keyBlob,
                            ABI::Windows::Security::Cryptography::Core::CryptographicPublicKeyBlobType BlobType,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAsymmetricKeyAlgorithmProvider = __uuidof(IAsymmetricKeyAlgorithmProvider);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricKeyAlgorithmProvider2[] = L"Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("4e322a7e-7c4d-4997-ac4f-1b848b36306e")
                    IAsymmetricKeyAlgorithmProvider2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateKeyPairWithCurveName(
                            HSTRING curveName,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateKeyPairWithCurveParameters(
                            UINT32 parametersLength,
                            BYTE* parameters,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAsymmetricKeyAlgorithmProvider2 = __uuidof(IAsymmetricKeyAlgorithmProvider2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricKeyAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("425bde18-a7f3-47a6-a8d2-c48d6033a65c")
                    IAsymmetricKeyAlgorithmProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OpenAlgorithm(
                            HSTRING algorithm,
                            ABI::Windows::Security::Cryptography::Core::IAsymmetricKeyAlgorithmProvider** provider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAsymmetricKeyAlgorithmProviderStatics = __uuidof(IAsymmetricKeyAlgorithmProviderStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ICryptographicEngineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ICryptographicEngineStatics[] = L"Windows.Security.Cryptography.Core.ICryptographicEngineStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("9fea0639-6ff7-4c85-a095-95eb31715eb9")
                    ICryptographicEngineStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Encrypt(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* iv,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Decrypt(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* iv,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE EncryptAndAuthenticate(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* nonce,
                            ABI::Windows::Storage::Streams::IBuffer* authenticatedData,
                            ABI::Windows::Security::Cryptography::Core::IEncryptedAndAuthenticatedData** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DecryptAndAuthenticate(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* nonce,
                            ABI::Windows::Storage::Streams::IBuffer* authenticationTag,
                            ABI::Windows::Storage::Streams::IBuffer* authenticatedData,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Sign(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE VerifySignature(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* signature,
                            boolean* isAuthenticated
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DeriveKeyMaterial(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters* parameters,
                            UINT32 desiredKeySize,
                            ABI::Windows::Storage::Streams::IBuffer** keyMaterial
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICryptographicEngineStatics = __uuidof(ICryptographicEngineStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ICryptographicEngineStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ICryptographicEngineStatics2[] = L"Windows.Security.Cryptography.Core.ICryptographicEngineStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("675948fe-df9f-4191-92c7-6ce6f58420e0")
                    ICryptographicEngineStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SignHashedData(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE VerifySignatureWithHashInput(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* signature,
                            boolean* isAuthenticated
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DecryptAsync(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer* iv,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SignAsync(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SignHashedDataAsync(
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey* key,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICryptographicEngineStatics2 = __uuidof(ICryptographicEngineStatics2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ICryptographicKey
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicKey
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ICryptographicKey[] = L"Windows.Security.Cryptography.Core.ICryptographicKey";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("ed2a3b70-8e7b-4009-8401-ffd1a62eeb27")
                    ICryptographicKey : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_KeySize(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ExportDefaultPrivateKeyBlobType(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ExportPrivateKeyWithBlobType(
                            ABI::Windows::Security::Cryptography::Core::CryptographicPrivateKeyBlobType BlobType,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ExportDefaultPublicKeyBlobType(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ExportPublicKeyWithBlobType(
                            ABI::Windows::Security::Cryptography::Core::CryptographicPublicKeyBlobType BlobType,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICryptographicKey = __uuidof(ICryptographicKey);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IEccCurveNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.EccCurveNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IEccCurveNamesStatics[] = L"Windows.Security.Cryptography.Core.IEccCurveNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("b3ff930c-aeeb-409e-b7d4-9b95295aaecf")
                    IEccCurveNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP160r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP160t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP192r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP192t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP224r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP224t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP256r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP256t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP320r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP320t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP384r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP384t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP512r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BrainpoolP512t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Curve25519(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Ec192wapi(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NistP192(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NistP224(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NistP256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NistP384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NistP521(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NumsP256t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NumsP384t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NumsP512t1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP160k1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP160r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP160r2(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP192k1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP192r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP224k1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP224r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP256k1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP256r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP384r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecP521r1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Wtls7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Wtls9(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Wtls12(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P192v1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P192v2(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P192v3(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P239v1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P239v2(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P239v3(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X962P256v1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AllEccCurveNames(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEccCurveNamesStatics = __uuidof(IEccCurveNamesStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IEncryptedAndAuthenticatedData[] = L"Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("6fa42fe7-1ecb-4b00-bea5-60b83f862f17")
                    IEncryptedAndAuthenticatedData : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EncryptedData(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthenticationTag(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEncryptedAndAuthenticatedData = __uuidof(IEncryptedAndAuthenticatedData);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.HashAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("6b5e0516-de96-4f0a-8d57-dcc9dae36c76")
                    IHashAlgorithmNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Md5(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sha512(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHashAlgorithmNamesStatics = __uuidof(IHashAlgorithmNamesStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.HashAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IHashAlgorithmProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("be9b3080-b2c3-422b-bce1-ec90efb5d7b5")
                    IHashAlgorithmProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HashLength(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE HashData(
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateHash(
                            ABI::Windows::Security::Cryptography::Core::IHashComputation** Value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHashAlgorithmProvider = __uuidof(IHashAlgorithmProvider);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.HashAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("9fac9741-5cc4-4336-ae38-6212b75a915a")
                    IHashAlgorithmProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OpenAlgorithm(
                            HSTRING algorithm,
                            ABI::Windows::Security::Cryptography::Core::IHashAlgorithmProvider** provider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHashAlgorithmProviderStatics = __uuidof(IHashAlgorithmProviderStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashComputation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicHash
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashComputation[] = L"Windows.Security.Cryptography.Core.IHashComputation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("5904d1b6-ad31-4603-a3a4-b1bda98e2562")
                    IHashComputation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Append(
                            ABI::Windows::Storage::Streams::IBuffer* data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValueAndReset(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHashComputation = __uuidof(IHashComputation);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("7b6e363e-94d2-4739-a57b-022e0c3a402a")
                    IKeyDerivationAlgorithmNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Pbkdf2Md5(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Pbkdf2Sha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Pbkdf2Sha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Pbkdf2Sha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Pbkdf2Sha512(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp800108CtrHmacMd5(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp800108CtrHmacSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp800108CtrHmacSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp800108CtrHmacSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp800108CtrHmacSha512(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp80056aConcatMd5(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp80056aConcatSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp80056aConcatSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp80056aConcatSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Sp80056aConcatSha512(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationAlgorithmNamesStatics = __uuidof(IKeyDerivationAlgorithmNamesStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmNamesStatics2[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("57953fab-6044-466f-97f4-337b7808384d")
                    IKeyDerivationAlgorithmNamesStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CapiKdfMd5(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapiKdfSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapiKdfSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapiKdfSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapiKdfSha512(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationAlgorithmNamesStatics2 = __uuidof(IKeyDerivationAlgorithmNamesStatics2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("e1fba83b-4671-43b7-9158-763aaa98b6bf")
                    IKeyDerivationAlgorithmProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateKey(
                            ABI::Windows::Storage::Streams::IBuffer* keyMaterial,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationAlgorithmProvider = __uuidof(IKeyDerivationAlgorithmProvider);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("0a22097a-0a1c-443b-9418-b9498aeb1603")
                    IKeyDerivationAlgorithmProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OpenAlgorithm(
                            HSTRING algorithm,
                            ABI::Windows::Security::Cryptography::Core::IKeyDerivationAlgorithmProvider** provider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationAlgorithmProviderStatics = __uuidof(IKeyDerivationAlgorithmProviderStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParameters[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParameters";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("7bf05967-047b-4a8c-964a-469ffd5522e2")
                    IKeyDerivationParameters : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_KdfGenericBinary(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KdfGenericBinary(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IterationCount(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationParameters = __uuidof(IKeyDerivationParameters);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParameters2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParameters2[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParameters2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("cd4166d1-417e-4f4c-b666-c0d879f3f8e0")
                    IKeyDerivationParameters2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Capi1KdfTargetAlgorithm(
                            ABI::Windows::Security::Cryptography::Core::Capi1KdfTargetAlgorithm* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Capi1KdfTargetAlgorithm(
                            ABI::Windows::Security::Cryptography::Core::Capi1KdfTargetAlgorithm value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationParameters2 = __uuidof(IKeyDerivationParameters2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParametersStatics[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("ea961fbe-f37f-4146-9dfe-a456f1735f4b")
                    IKeyDerivationParametersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE BuildForPbkdf2(
                            ABI::Windows::Storage::Streams::IBuffer* pbkdf2Salt,
                            UINT32 iterationCount,
                            ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE BuildForSP800108(
                            ABI::Windows::Storage::Streams::IBuffer* label,
                            ABI::Windows::Storage::Streams::IBuffer* context,
                            ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE BuildForSP80056a(
                            ABI::Windows::Storage::Streams::IBuffer* algorithmId,
                            ABI::Windows::Storage::Streams::IBuffer* partyUInfo,
                            ABI::Windows::Storage::Streams::IBuffer* partyVInfo,
                            ABI::Windows::Storage::Streams::IBuffer* suppPubInfo,
                            ABI::Windows::Storage::Streams::IBuffer* suppPrivInfo,
                            ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationParametersStatics = __uuidof(IKeyDerivationParametersStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParametersStatics2[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("a5783dd5-58e3-4efb-b283-a1653126e1be")
                    IKeyDerivationParametersStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE BuildForCapi1Kdf(
                            ABI::Windows::Security::Cryptography::Core::Capi1KdfTargetAlgorithm capi1KdfTargetAlgorithm,
                            ABI::Windows::Security::Cryptography::Core::IKeyDerivationParameters** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IKeyDerivationParametersStatics2 = __uuidof(IKeyDerivationParametersStatics2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("41412678-fb1e-43a4-895e-a9026e4390a3")
                    IMacAlgorithmNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HmacMd5(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HmacSha1(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HmacSha256(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HmacSha384(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HmacSha512(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesCmac(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMacAlgorithmNamesStatics = __uuidof(IMacAlgorithmNamesStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("4a3fc5c3-1cbd-41ce-a092-aa0bc5d2d2f5")
                    IMacAlgorithmProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MacLength(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateKey(
                            ABI::Windows::Storage::Streams::IBuffer* keyMaterial,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** macKey
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMacAlgorithmProvider = __uuidof(IMacAlgorithmProvider);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmProvider2[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmProvider2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("6da32a15-d931-42ed-8e7e-c301caee119c")
                    IMacAlgorithmProvider2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateHash(
                            ABI::Windows::Storage::Streams::IBuffer* keyMaterial,
                            ABI::Windows::Security::Cryptography::Core::IHashComputation** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMacAlgorithmProvider2 = __uuidof(IMacAlgorithmProvider2);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("c9bdc147-cc77-4df0-9e4e-b921e080644c")
                    IMacAlgorithmProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OpenAlgorithm(
                            HSTRING algorithm,
                            ABI::Windows::Security::Cryptography::Core::IMacAlgorithmProvider** provider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMacAlgorithmProviderStatics = __uuidof(IMacAlgorithmProviderStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.PersistedKeyProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IPersistedKeyProviderStatics[] = L"Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("77274814-d9d4-4cf5-b668-e0457df30894")
                    IPersistedKeyProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OpenKeyPairFromCertificateAsync(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate,
                            HSTRING hashAlgorithmName,
                            ABI::Windows::Security::Cryptography::Core::CryptographicPadding padding,
                            __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OpenPublicKeyFromCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate,
                            HSTRING hashAlgorithmName,
                            ABI::Windows::Security::Cryptography::Core::CryptographicPadding padding,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPersistedKeyProviderStatics = __uuidof(IPersistedKeyProviderStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.SymmetricAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ISymmetricAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("6870727b-c996-4eae-84d7-79b2aeb73b9c")
                    ISymmetricAlgorithmNamesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DesCbc(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesEcb(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TripleDesCbc(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TripleDesEcb(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Rc2Cbc(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Rc2Ecb(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesCbc(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesEcb(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesGcm(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesCcm(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesCbcPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AesEcbPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesCbcPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesEcbPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TripleDesCbcPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TripleDesEcbPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Rc2CbcPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Rc2EcbPkcs7(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Rc4(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISymmetricAlgorithmNamesStatics = __uuidof(ISymmetricAlgorithmNamesStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ISymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("3d7e4a33-3bd0-4902-8ac8-470d50d21376")
                    ISymmetricKeyAlgorithmProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AlgorithmName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BlockLength(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateSymmetricKey(
                            ABI::Windows::Storage::Streams::IBuffer* keyMaterial,
                            ABI::Windows::Security::Cryptography::Core::ICryptographicKey** key
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISymmetricKeyAlgorithmProvider = __uuidof(ISymmetricKeyAlgorithmProvider);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ISymmetricKeyAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    MIDL_INTERFACE("8d3b2326-1f37-491f-b60e-f5431b26b483")
                    ISymmetricKeyAlgorithmProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OpenAlgorithm(
                            HSTRING algorithm,
                            ABI::Windows::Security::Cryptography::Core::ISymmetricKeyAlgorithmProvider** provider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISymmetricKeyAlgorithmProviderStatics = __uuidof(ISymmetricKeyAlgorithmProviderStatics);
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_AsymmetricAlgorithmNames[] = L"Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider ** Default Interface **
 *    Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricKeyAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricKeyAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_AsymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.CryptographicEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ICryptographicEngineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ICryptographicEngineStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicEngine_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicEngine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_CryptographicEngine[] = L"Windows.Security.Cryptography.Core.CryptographicEngine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.CryptographicHash
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IHashComputation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicHash_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicHash_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_CryptographicHash[] = L"Windows.Security.Cryptography.Core.CryptographicHash";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.CryptographicKey
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.ICryptographicKey ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicKey_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicKey_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_CryptographicKey[] = L"Windows.Security.Cryptography.Core.CryptographicKey";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.EccCurveNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IEccCurveNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_EccCurveNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_EccCurveNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_EccCurveNames[] = L"Windows.Security.Cryptography.Core.EccCurveNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_EncryptedAndAuthenticatedData_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_EncryptedAndAuthenticatedData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_EncryptedAndAuthenticatedData[] = L"Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.HashAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_HashAlgorithmNames[] = L"Windows.Security.Cryptography.Core.HashAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.HashAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IHashAlgorithmProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_HashAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.HashAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmNames[] = L"Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IKeyDerivationParameters ** Default Interface **
 *    Windows.Security.Cryptography.Core.IKeyDerivationParameters2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_KeyDerivationParameters[] = L"Windows.Security.Cryptography.Core.KeyDerivationParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.MacAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_MacAlgorithmNames[] = L"Windows.Security.Cryptography.Core.MacAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IMacAlgorithmProvider ** Default Interface **
 *    Windows.Security.Cryptography.Core.IMacAlgorithmProvider2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_MacAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.MacAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.PersistedKeyProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_PersistedKeyProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_PersistedKeyProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_PersistedKeyProvider[] = L"Windows.Security.Cryptography.Core.PersistedKeyProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.SymmetricAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_SymmetricAlgorithmNames[] = L"Windows.Security.Cryptography.Core.SymmetricAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricKeyAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricKeyAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_SymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2 __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__
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

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCapi1KdfTargetAlgorithm __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCapi1KdfTargetAlgorithm;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPadding __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPadding;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPrivateKeyBlobType __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPrivateKeyBlobType;

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType;

/*
 *
 * Struct Windows.Security.Cryptography.Core.Capi1KdfTargetAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCapi1KdfTargetAlgorithm
{
    Capi1KdfTargetAlgorithm_NotAes = 0,
    Capi1KdfTargetAlgorithm_Aes = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Core.CryptographicPadding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPadding
{
    CryptographicPadding_None = 0,
    CryptographicPadding_RsaOaep = 1,
    CryptographicPadding_RsaPkcs1V15 = 2,
    CryptographicPadding_RsaPss = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Core.CryptographicPrivateKeyBlobType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPrivateKeyBlobType
{
    CryptographicPrivateKeyBlobType_Pkcs8RawPrivateKeyInfo = 0,
    CryptographicPrivateKeyBlobType_Pkcs1RsaPrivateKey = 1,
    CryptographicPrivateKeyBlobType_BCryptPrivateKey = 2,
    CryptographicPrivateKeyBlobType_Capi1PrivateKey = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    CryptographicPrivateKeyBlobType_BCryptEccFullPrivateKey = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Cryptography.Core.CryptographicPublicKeyBlobType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType
{
    CryptographicPublicKeyBlobType_X509SubjectPublicKeyInfo = 0,
    CryptographicPublicKeyBlobType_Pkcs1RsaPublicKey = 1,
    CryptographicPublicKeyBlobType_BCryptPublicKey = 2,
    CryptographicPublicKeyBlobType_Capi1PublicKey = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    CryptographicPublicKeyBlobType_BCryptEccFullPublicKey = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RsaPkcs1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaOaepSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaOaepSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaOaepSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaOaepSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EcdsaP256Sha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EcdsaP384Sha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EcdsaP521Sha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DsaSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DsaSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPkcs1Sha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPkcs1Sha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPkcs1Sha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPkcs1Sha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPssSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPssSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPssSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RsaSignPssSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaPkcs1(This, value) \
    ((This)->lpVtbl->get_RsaPkcs1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaOaepSha1(This, value) \
    ((This)->lpVtbl->get_RsaOaepSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaOaepSha256(This, value) \
    ((This)->lpVtbl->get_RsaOaepSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaOaepSha384(This, value) \
    ((This)->lpVtbl->get_RsaOaepSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaOaepSha512(This, value) \
    ((This)->lpVtbl->get_RsaOaepSha512(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_EcdsaP256Sha256(This, value) \
    ((This)->lpVtbl->get_EcdsaP256Sha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_EcdsaP384Sha384(This, value) \
    ((This)->lpVtbl->get_EcdsaP384Sha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_EcdsaP521Sha512(This, value) \
    ((This)->lpVtbl->get_EcdsaP521Sha512(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_DsaSha1(This, value) \
    ((This)->lpVtbl->get_DsaSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_DsaSha256(This, value) \
    ((This)->lpVtbl->get_DsaSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPkcs1Sha1(This, value) \
    ((This)->lpVtbl->get_RsaSignPkcs1Sha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPkcs1Sha256(This, value) \
    ((This)->lpVtbl->get_RsaSignPkcs1Sha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPkcs1Sha384(This, value) \
    ((This)->lpVtbl->get_RsaSignPkcs1Sha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPkcs1Sha512(This, value) \
    ((This)->lpVtbl->get_RsaSignPkcs1Sha512(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPssSha1(This, value) \
    ((This)->lpVtbl->get_RsaSignPssSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPssSha256(This, value) \
    ((This)->lpVtbl->get_RsaSignPssSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPssSha384(This, value) \
    ((This)->lpVtbl->get_RsaSignPssSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_get_RsaSignPssSha512(This, value) \
    ((This)->lpVtbl->get_RsaSignPssSha512(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricAlgorithmNamesStatics2[] = L"Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EcdsaSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EcdsaSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EcdsaSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_get_EcdsaSha256(This, value) \
    ((This)->lpVtbl->get_EcdsaSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_get_EcdsaSha384(This, value) \
    ((This)->lpVtbl->get_EcdsaSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_get_EcdsaSha512(This, value) \
    ((This)->lpVtbl->get_EcdsaSha512(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricAlgorithmNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* CreateKeyPair)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        UINT32 keySize,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);
    HRESULT (STDMETHODCALLTYPE* ImportDefaultPrivateKeyBlob)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyBlob,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);
    HRESULT (STDMETHODCALLTYPE* ImportKeyPairWithBlobType)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyBlob,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPrivateKeyBlobType BlobType,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);
    HRESULT (STDMETHODCALLTYPE* ImportDefaultPublicKeyBlob)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyBlob,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);
    HRESULT (STDMETHODCALLTYPE* ImportPublicKeyWithBlobType)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyBlob,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType BlobType,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_get_AlgorithmName(This, value) \
    ((This)->lpVtbl->get_AlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_CreateKeyPair(This, keySize, key) \
    ((This)->lpVtbl->CreateKeyPair(This, keySize, key))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_ImportDefaultPrivateKeyBlob(This, keyBlob, key) \
    ((This)->lpVtbl->ImportDefaultPrivateKeyBlob(This, keyBlob, key))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_ImportKeyPairWithBlobType(This, keyBlob, BlobType, key) \
    ((This)->lpVtbl->ImportKeyPairWithBlobType(This, keyBlob, BlobType, key))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_ImportDefaultPublicKeyBlob(This, keyBlob, key) \
    ((This)->lpVtbl->ImportDefaultPublicKeyBlob(This, keyBlob, key))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_ImportPublicKeyWithBlobType(This, keyBlob, BlobType, key) \
    ((This)->lpVtbl->ImportPublicKeyWithBlobType(This, keyBlob, BlobType, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricKeyAlgorithmProvider2[] = L"Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateKeyPairWithCurveName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This,
        HSTRING curveName,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);
    HRESULT (STDMETHODCALLTYPE* CreateKeyPairWithCurveParameters)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2* This,
        UINT32 parametersLength,
        BYTE* parameters,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_CreateKeyPairWithCurveName(This, curveName, key) \
    ((This)->lpVtbl->CreateKeyPairWithCurveName(This, curveName, key))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_CreateKeyPairWithCurveParameters(This, parametersLength, parameters, key) \
    ((This)->lpVtbl->CreateKeyPairWithCurveParameters(This, parametersLength, parameters, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IAsymmetricKeyAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics* This,
        HSTRING algorithm,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProvider** provider);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_OpenAlgorithm(This, algorithm, provider) \
    ((This)->lpVtbl->OpenAlgorithm(This, algorithm, provider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIAsymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ICryptographicEngineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ICryptographicEngineStatics[] = L"Windows.Security.Cryptography.Core.ICryptographicEngineStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Encrypt)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* iv,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* Decrypt)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* iv,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* EncryptAndAuthenticate)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* nonce,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* authenticatedData,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData** value);
    HRESULT (STDMETHODCALLTYPE* DecryptAndAuthenticate)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* nonce,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* authenticationTag,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* authenticatedData,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* Sign)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* VerifySignature)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* signature,
        boolean* isAuthenticated);
    HRESULT (STDMETHODCALLTYPE* DeriveKeyMaterial)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* parameters,
        UINT32 desiredKeySize,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** keyMaterial);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_Encrypt(This, key, data, iv, value) \
    ((This)->lpVtbl->Encrypt(This, key, data, iv, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_Decrypt(This, key, data, iv, value) \
    ((This)->lpVtbl->Decrypt(This, key, data, iv, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_EncryptAndAuthenticate(This, key, data, nonce, authenticatedData, value) \
    ((This)->lpVtbl->EncryptAndAuthenticate(This, key, data, nonce, authenticatedData, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_DecryptAndAuthenticate(This, key, data, nonce, authenticationTag, authenticatedData, value) \
    ((This)->lpVtbl->DecryptAndAuthenticate(This, key, data, nonce, authenticationTag, authenticatedData, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_Sign(This, key, data, value) \
    ((This)->lpVtbl->Sign(This, key, data, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_VerifySignature(This, key, data, signature, isAuthenticated) \
    ((This)->lpVtbl->VerifySignature(This, key, data, signature, isAuthenticated))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_DeriveKeyMaterial(This, key, parameters, desiredKeySize, keyMaterial) \
    ((This)->lpVtbl->DeriveKeyMaterial(This, key, parameters, desiredKeySize, keyMaterial))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ICryptographicEngineStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ICryptographicEngineStatics2[] = L"Windows.Security.Cryptography.Core.ICryptographicEngineStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SignHashedData)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* VerifySignatureWithHashInput)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* signature,
        boolean* isAuthenticated);
    HRESULT (STDMETHODCALLTYPE* DecryptAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* iv,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* SignAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* SignHashedDataAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_SignHashedData(This, key, data, value) \
    ((This)->lpVtbl->SignHashedData(This, key, data, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_VerifySignatureWithHashInput(This, key, data, signature, isAuthenticated) \
    ((This)->lpVtbl->VerifySignatureWithHashInput(This, key, data, signature, isAuthenticated))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_DecryptAsync(This, key, data, iv, value) \
    ((This)->lpVtbl->DecryptAsync(This, key, data, iv, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_SignAsync(This, key, data, value) \
    ((This)->lpVtbl->SignAsync(This, key, data, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_SignHashedDataAsync(This, key, data, value) \
    ((This)->lpVtbl->SignHashedDataAsync(This, key, data, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicEngineStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ICryptographicKey
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicKey
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ICryptographicKey[] = L"Windows.Security.Cryptography.Core.ICryptographicKey";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKeyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeySize)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* ExportDefaultPrivateKeyBlobType)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* ExportPrivateKeyWithBlobType)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPrivateKeyBlobType BlobType,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* ExportDefaultPublicKeyBlobType)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* ExportPublicKeyWithBlobType)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType BlobType,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKeyVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKeyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_get_KeySize(This, value) \
    ((This)->lpVtbl->get_KeySize(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_ExportDefaultPrivateKeyBlobType(This, value) \
    ((This)->lpVtbl->ExportDefaultPrivateKeyBlobType(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_ExportPrivateKeyWithBlobType(This, BlobType, value) \
    ((This)->lpVtbl->ExportPrivateKeyWithBlobType(This, BlobType, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_ExportDefaultPublicKeyBlobType(This, value) \
    ((This)->lpVtbl->ExportDefaultPublicKeyBlobType(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_ExportPublicKeyWithBlobType(This, BlobType, value) \
    ((This)->lpVtbl->ExportPublicKeyWithBlobType(This, BlobType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IEccCurveNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.EccCurveNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IEccCurveNamesStatics[] = L"Windows.Security.Cryptography.Core.IEccCurveNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP160r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP160t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP192r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP192t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP224r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP224t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP256r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP256t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP320r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP320t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP384r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP384t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP512r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BrainpoolP512t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Curve25519)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ec192wapi)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NistP192)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NistP224)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NistP256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NistP384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NistP521)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NumsP256t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NumsP384t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NumsP512t1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP160k1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP160r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP160r2)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP192k1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP192r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP224k1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP224r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP256k1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP256r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP384r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SecP521r1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wtls7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wtls9)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wtls12)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P192v1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P192v2)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P192v3)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P239v1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P239v2)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P239v3)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_X962P256v1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AllEccCurveNames)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP160r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP160r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP160t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP160t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP192r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP192r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP192t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP192t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP224r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP224r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP224t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP224t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP256r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP256r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP256t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP256t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP320r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP320r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP320t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP320t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP384r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP384r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP384t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP384t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP512r1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP512r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_BrainpoolP512t1(This, value) \
    ((This)->lpVtbl->get_BrainpoolP512t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_Curve25519(This, value) \
    ((This)->lpVtbl->get_Curve25519(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_Ec192wapi(This, value) \
    ((This)->lpVtbl->get_Ec192wapi(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NistP192(This, value) \
    ((This)->lpVtbl->get_NistP192(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NistP224(This, value) \
    ((This)->lpVtbl->get_NistP224(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NistP256(This, value) \
    ((This)->lpVtbl->get_NistP256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NistP384(This, value) \
    ((This)->lpVtbl->get_NistP384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NistP521(This, value) \
    ((This)->lpVtbl->get_NistP521(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NumsP256t1(This, value) \
    ((This)->lpVtbl->get_NumsP256t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NumsP384t1(This, value) \
    ((This)->lpVtbl->get_NumsP384t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_NumsP512t1(This, value) \
    ((This)->lpVtbl->get_NumsP512t1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP160k1(This, value) \
    ((This)->lpVtbl->get_SecP160k1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP160r1(This, value) \
    ((This)->lpVtbl->get_SecP160r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP160r2(This, value) \
    ((This)->lpVtbl->get_SecP160r2(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP192k1(This, value) \
    ((This)->lpVtbl->get_SecP192k1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP192r1(This, value) \
    ((This)->lpVtbl->get_SecP192r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP224k1(This, value) \
    ((This)->lpVtbl->get_SecP224k1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP224r1(This, value) \
    ((This)->lpVtbl->get_SecP224r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP256k1(This, value) \
    ((This)->lpVtbl->get_SecP256k1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP256r1(This, value) \
    ((This)->lpVtbl->get_SecP256r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP384r1(This, value) \
    ((This)->lpVtbl->get_SecP384r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_SecP521r1(This, value) \
    ((This)->lpVtbl->get_SecP521r1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_Wtls7(This, value) \
    ((This)->lpVtbl->get_Wtls7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_Wtls9(This, value) \
    ((This)->lpVtbl->get_Wtls9(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_Wtls12(This, value) \
    ((This)->lpVtbl->get_Wtls12(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P192v1(This, value) \
    ((This)->lpVtbl->get_X962P192v1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P192v2(This, value) \
    ((This)->lpVtbl->get_X962P192v2(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P192v3(This, value) \
    ((This)->lpVtbl->get_X962P192v3(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P239v1(This, value) \
    ((This)->lpVtbl->get_X962P239v1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P239v2(This, value) \
    ((This)->lpVtbl->get_X962P239v2(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P239v3(This, value) \
    ((This)->lpVtbl->get_X962P239v3(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_X962P256v1(This, value) \
    ((This)->lpVtbl->get_X962P256v1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_get_AllEccCurveNames(This, value) \
    ((This)->lpVtbl->get_AllEccCurveNames(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEccCurveNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IEncryptedAndAuthenticatedData[] = L"Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EncryptedData)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_AuthenticationTag)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedDataVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_get_EncryptedData(This, value) \
    ((This)->lpVtbl->get_EncryptedData(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_get_AuthenticationTag(This, value) \
    ((This)->lpVtbl->get_AuthenticationTag(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIEncryptedAndAuthenticatedData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.HashAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Md5)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_get_Md5(This, value) \
    ((This)->lpVtbl->get_Md5(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_get_Sha1(This, value) \
    ((This)->lpVtbl->get_Sha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_get_Sha256(This, value) \
    ((This)->lpVtbl->get_Sha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_get_Sha384(This, value) \
    ((This)->lpVtbl->get_Sha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_get_Sha512(This, value) \
    ((This)->lpVtbl->get_Sha512(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.HashAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IHashAlgorithmProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HashLength)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* HashData)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* CreateHash)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation** Value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_get_AlgorithmName(This, value) \
    ((This)->lpVtbl->get_AlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_get_HashLength(This, value) \
    ((This)->lpVtbl->get_HashLength(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_HashData(This, data, value) \
    ((This)->lpVtbl->HashData(This, data, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_CreateHash(This, Value) \
    ((This)->lpVtbl->CreateHash(This, Value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.HashAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics* This,
        HSTRING algorithm,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProvider** provider);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_OpenAlgorithm(This, algorithm, provider) \
    ((This)->lpVtbl->OpenAlgorithm(This, algorithm, provider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IHashComputation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.CryptographicHash
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IHashComputation[] = L"Windows.Security.Cryptography.Core.IHashComputation";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Append)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data);
    HRESULT (STDMETHODCALLTYPE* GetValueAndReset)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputationVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_Append(This, data) \
    ((This)->lpVtbl->Append(This, data))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_GetValueAndReset(This, value) \
    ((This)->lpVtbl->GetValueAndReset(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Pbkdf2Md5)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pbkdf2Sha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pbkdf2Sha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pbkdf2Sha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pbkdf2Sha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp800108CtrHmacMd5)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp800108CtrHmacSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp800108CtrHmacSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp800108CtrHmacSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp800108CtrHmacSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp80056aConcatMd5)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp80056aConcatSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp80056aConcatSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp80056aConcatSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sp80056aConcatSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Pbkdf2Md5(This, value) \
    ((This)->lpVtbl->get_Pbkdf2Md5(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Pbkdf2Sha1(This, value) \
    ((This)->lpVtbl->get_Pbkdf2Sha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Pbkdf2Sha256(This, value) \
    ((This)->lpVtbl->get_Pbkdf2Sha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Pbkdf2Sha384(This, value) \
    ((This)->lpVtbl->get_Pbkdf2Sha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Pbkdf2Sha512(This, value) \
    ((This)->lpVtbl->get_Pbkdf2Sha512(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp800108CtrHmacMd5(This, value) \
    ((This)->lpVtbl->get_Sp800108CtrHmacMd5(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp800108CtrHmacSha1(This, value) \
    ((This)->lpVtbl->get_Sp800108CtrHmacSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp800108CtrHmacSha256(This, value) \
    ((This)->lpVtbl->get_Sp800108CtrHmacSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp800108CtrHmacSha384(This, value) \
    ((This)->lpVtbl->get_Sp800108CtrHmacSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp800108CtrHmacSha512(This, value) \
    ((This)->lpVtbl->get_Sp800108CtrHmacSha512(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp80056aConcatMd5(This, value) \
    ((This)->lpVtbl->get_Sp80056aConcatMd5(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp80056aConcatSha1(This, value) \
    ((This)->lpVtbl->get_Sp80056aConcatSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp80056aConcatSha256(This, value) \
    ((This)->lpVtbl->get_Sp80056aConcatSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp80056aConcatSha384(This, value) \
    ((This)->lpVtbl->get_Sp80056aConcatSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_get_Sp80056aConcatSha512(This, value) \
    ((This)->lpVtbl->get_Sp80056aConcatSha512(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmNamesStatics2[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CapiKdfMd5)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CapiKdfSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CapiKdfSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CapiKdfSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CapiKdfSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_get_CapiKdfMd5(This, value) \
    ((This)->lpVtbl->get_CapiKdfMd5(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_get_CapiKdfSha1(This, value) \
    ((This)->lpVtbl->get_CapiKdfSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_get_CapiKdfSha256(This, value) \
    ((This)->lpVtbl->get_CapiKdfSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_get_CapiKdfSha384(This, value) \
    ((This)->lpVtbl->get_CapiKdfSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_get_CapiKdfSha512(This, value) \
    ((This)->lpVtbl->get_CapiKdfSha512(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmNamesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* CreateKey)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyMaterial,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_get_AlgorithmName(This, value) \
    ((This)->lpVtbl->get_AlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_CreateKey(This, keyMaterial, key) \
    ((This)->lpVtbl->CreateKey(This, keyMaterial, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics* This,
        HSTRING algorithm,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProvider** provider);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_OpenAlgorithm(This, algorithm, provider) \
    ((This)->lpVtbl->OpenAlgorithm(This, algorithm, provider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParameters[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParameters";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KdfGenericBinary)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_KdfGenericBinary)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_IterationCount)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_get_KdfGenericBinary(This, value) \
    ((This)->lpVtbl->get_KdfGenericBinary(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_put_KdfGenericBinary(This, value) \
    ((This)->lpVtbl->put_KdfGenericBinary(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_get_IterationCount(This, value) \
    ((This)->lpVtbl->get_IterationCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParameters2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParameters2[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParameters2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capi1KdfTargetAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCapi1KdfTargetAlgorithm* value);
    HRESULT (STDMETHODCALLTYPE* put_Capi1KdfTargetAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCapi1KdfTargetAlgorithm value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_get_Capi1KdfTargetAlgorithm(This, value) \
    ((This)->lpVtbl->get_Capi1KdfTargetAlgorithm(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_put_Capi1KdfTargetAlgorithm(This, value) \
    ((This)->lpVtbl->put_Capi1KdfTargetAlgorithm(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParametersStatics[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BuildForPbkdf2)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* pbkdf2Salt,
        UINT32 iterationCount,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters** value);
    HRESULT (STDMETHODCALLTYPE* BuildForSP800108)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* label,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* context,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters** value);
    HRESULT (STDMETHODCALLTYPE* BuildForSP80056a)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* algorithmId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* partyUInfo,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* partyVInfo,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* suppPubInfo,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* suppPrivInfo,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_BuildForPbkdf2(This, pbkdf2Salt, iterationCount, value) \
    ((This)->lpVtbl->BuildForPbkdf2(This, pbkdf2Salt, iterationCount, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_BuildForSP800108(This, label, context, value) \
    ((This)->lpVtbl->BuildForSP800108(This, label, context, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_BuildForSP80056a(This, algorithmId, partyUInfo, partyVInfo, suppPubInfo, suppPrivInfo, value) \
    ((This)->lpVtbl->BuildForSP80056a(This, algorithmId, partyUInfo, partyVInfo, suppPubInfo, suppPrivInfo, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IKeyDerivationParametersStatics2[] = L"Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BuildForCapi1Kdf)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCapi1KdfTargetAlgorithm capi1KdfTargetAlgorithm,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParameters** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_BuildForCapi1Kdf(This, capi1KdfTargetAlgorithm, value) \
    ((This)->lpVtbl->BuildForCapi1Kdf(This, capi1KdfTargetAlgorithm, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIKeyDerivationParametersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HmacMd5)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HmacSha1)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HmacSha256)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HmacSha384)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HmacSha512)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesCmac)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_get_HmacMd5(This, value) \
    ((This)->lpVtbl->get_HmacMd5(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_get_HmacSha1(This, value) \
    ((This)->lpVtbl->get_HmacSha1(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_get_HmacSha256(This, value) \
    ((This)->lpVtbl->get_HmacSha256(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_get_HmacSha384(This, value) \
    ((This)->lpVtbl->get_HmacSha384(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_get_HmacSha512(This, value) \
    ((This)->lpVtbl->get_HmacSha512(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_get_AesCmac(This, value) \
    ((This)->lpVtbl->get_AesCmac(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MacLength)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* CreateKey)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyMaterial,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** macKey);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_get_AlgorithmName(This, value) \
    ((This)->lpVtbl->get_AlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_get_MacLength(This, value) \
    ((This)->lpVtbl->get_MacLength(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_CreateKey(This, keyMaterial, macKey) \
    ((This)->lpVtbl->CreateKey(This, keyMaterial, macKey))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmProvider2[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmProvider2";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateHash)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyMaterial,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIHashComputation** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_CreateHash(This, keyMaterial, value) \
    ((This)->lpVtbl->CreateHash(This, keyMaterial, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IMacAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics* This,
        HSTRING algorithm,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProvider** provider);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_OpenAlgorithm(This, algorithm, provider) \
    ((This)->lpVtbl->OpenAlgorithm(This, algorithm, provider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIMacAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.PersistedKeyProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_IPersistedKeyProviderStatics[] = L"Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenKeyPairFromCertificateAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate,
        HSTRING hashAlgorithmName,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPadding padding,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey** operation);
    HRESULT (STDMETHODCALLTYPE* OpenPublicKeyFromCertificate)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate,
        HSTRING hashAlgorithmName,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPadding padding,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_OpenKeyPairFromCertificateAsync(This, certificate, hashAlgorithmName, padding, operation) \
    ((This)->lpVtbl->OpenKeyPairFromCertificateAsync(This, certificate, hashAlgorithmName, padding, operation))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_OpenPublicKeyFromCertificate(This, certificate, hashAlgorithmName, padding, key) \
    ((This)->lpVtbl->OpenPublicKeyFromCertificate(This, certificate, hashAlgorithmName, padding, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CIPersistedKeyProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.SymmetricAlgorithmNames
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ISymmetricAlgorithmNamesStatics[] = L"Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesCbc)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DesEcb)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TripleDesCbc)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TripleDesEcb)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rc2Cbc)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rc2Ecb)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesCbc)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesEcb)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesGcm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesCcm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesCbcPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AesEcbPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DesCbcPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DesEcbPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TripleDesCbcPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TripleDesEcbPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rc2CbcPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rc2EcbPkcs7)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rc4)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_DesCbc(This, value) \
    ((This)->lpVtbl->get_DesCbc(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_DesEcb(This, value) \
    ((This)->lpVtbl->get_DesEcb(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_TripleDesCbc(This, value) \
    ((This)->lpVtbl->get_TripleDesCbc(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_TripleDesEcb(This, value) \
    ((This)->lpVtbl->get_TripleDesEcb(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_Rc2Cbc(This, value) \
    ((This)->lpVtbl->get_Rc2Cbc(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_Rc2Ecb(This, value) \
    ((This)->lpVtbl->get_Rc2Ecb(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_AesCbc(This, value) \
    ((This)->lpVtbl->get_AesCbc(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_AesEcb(This, value) \
    ((This)->lpVtbl->get_AesEcb(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_AesGcm(This, value) \
    ((This)->lpVtbl->get_AesGcm(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_AesCcm(This, value) \
    ((This)->lpVtbl->get_AesCcm(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_AesCbcPkcs7(This, value) \
    ((This)->lpVtbl->get_AesCbcPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_AesEcbPkcs7(This, value) \
    ((This)->lpVtbl->get_AesEcbPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_DesCbcPkcs7(This, value) \
    ((This)->lpVtbl->get_DesCbcPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_DesEcbPkcs7(This, value) \
    ((This)->lpVtbl->get_DesEcbPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_TripleDesCbcPkcs7(This, value) \
    ((This)->lpVtbl->get_TripleDesCbcPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_TripleDesEcbPkcs7(This, value) \
    ((This)->lpVtbl->get_TripleDesEcbPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_Rc2CbcPkcs7(This, value) \
    ((This)->lpVtbl->get_Rc2CbcPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_Rc2EcbPkcs7(This, value) \
    ((This)->lpVtbl->get_Rc2EcbPkcs7(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_get_Rc4(This, value) \
    ((This)->lpVtbl->get_Rc4(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricAlgorithmNamesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ISymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlgorithmName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BlockLength)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* CreateSymmetricKey)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* keyMaterial,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** key);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_get_AlgorithmName(This, value) \
    ((This)->lpVtbl->get_AlgorithmName(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_get_BlockLength(This, value) \
    ((This)->lpVtbl->get_BlockLength(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_CreateSymmetricKey(This, keyMaterial, key) \
    ((This)->lpVtbl->CreateSymmetricKey(This, keyMaterial, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_Core_ISymmetricKeyAlgorithmProviderStatics[] = L"Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenAlgorithm)(__x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics* This,
        HSTRING algorithm,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProvider** provider);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_OpenAlgorithm(This, algorithm, provider) \
    ((This)->lpVtbl->OpenAlgorithm(This, algorithm, provider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CCore_CISymmetricKeyAlgorithmProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_AsymmetricAlgorithmNames[] = L"Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider ** Default Interface **
 *    Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricKeyAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_AsymmetricKeyAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_AsymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.CryptographicEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ICryptographicEngineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ICryptographicEngineStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicEngine_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicEngine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_CryptographicEngine[] = L"Windows.Security.Cryptography.Core.CryptographicEngine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.CryptographicHash
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IHashComputation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicHash_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicHash_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_CryptographicHash[] = L"Windows.Security.Cryptography.Core.CryptographicHash";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.CryptographicKey
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.ICryptographicKey ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicKey_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_CryptographicKey_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_CryptographicKey[] = L"Windows.Security.Cryptography.Core.CryptographicKey";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.EccCurveNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IEccCurveNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_EccCurveNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_EccCurveNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_EccCurveNames[] = L"Windows.Security.Cryptography.Core.EccCurveNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_EncryptedAndAuthenticatedData_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_EncryptedAndAuthenticatedData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_EncryptedAndAuthenticatedData[] = L"Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.HashAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_HashAlgorithmNames[] = L"Windows.Security.Cryptography.Core.HashAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.HashAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IHashAlgorithmProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_HashAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_HashAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.HashAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmNames[] = L"Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_KeyDerivationAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.KeyDerivationParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IKeyDerivationParameters ** Default Interface **
 *    Windows.Security.Cryptography.Core.IKeyDerivationParameters2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_KeyDerivationParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_KeyDerivationParameters[] = L"Windows.Security.Cryptography.Core.KeyDerivationParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.MacAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_MacAlgorithmNames[] = L"Windows.Security.Cryptography.Core.MacAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.MacAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.IMacAlgorithmProvider ** Default Interface **
 *    Windows.Security.Cryptography.Core.IMacAlgorithmProvider2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_MacAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_MacAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.MacAlgorithmProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.PersistedKeyProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_PersistedKeyProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_PersistedKeyProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_PersistedKeyProvider[] = L"Windows.Security.Cryptography.Core.PersistedKeyProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.SymmetricAlgorithmNames
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricAlgorithmNames_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricAlgorithmNames_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_SymmetricAlgorithmNames[] = L"Windows.Security.Cryptography.Core.SymmetricAlgorithmNames";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricKeyAlgorithmProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_Core_SymmetricKeyAlgorithmProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_Core_SymmetricKeyAlgorithmProvider[] = L"Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider";
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
#endif // __windows2Esecurity2Ecryptography2Ecore_p_h__

#endif // __windows2Esecurity2Ecryptography2Ecore_h__
