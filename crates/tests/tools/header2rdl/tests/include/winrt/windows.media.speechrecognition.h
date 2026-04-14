
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
#ifndef __windows2Emedia2Espeechrecognition_h__
#define __windows2Emedia2Espeechrecognition_h__
#ifndef __windows2Emedia2Espeechrecognition_p_h__
#define __windows2Emedia2Espeechrecognition_p_h__


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

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

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
#include "Windows.Globalization.h"
#include "Windows.Phone.h"
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechContinuousRecognitionCompletedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionCompletedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechContinuousRecognitionResultGeneratedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionResultGeneratedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechContinuousRecognitionSession;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionSession

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionCompilationResult;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionCompilationResult

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionGrammarFileConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionGrammarFileConstraint

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionGrammarFileConstraintFactory;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionGrammarFileConstraintFactory

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionHypothesis;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionHypothesis

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionHypothesisGeneratedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionHypothesisGeneratedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionListConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionListConstraint

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionListConstraintFactory;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionListConstraintFactory

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionQualityDegradingEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionQualityDegradingEventArgs

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionResult;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionResult2;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2 ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult2

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionSemanticInterpretation;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionSemanticInterpretation

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionTopicConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionTopicConstraint

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionTopicConstraintFactory;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionTopicConstraintFactory

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionVoiceCommandDefinitionConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionVoiceCommandDefinitionConstraint

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizer;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer ABI::Windows::Media::SpeechRecognition::ISpeechRecognizer

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizer2;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2 ABI::Windows::Media::SpeechRecognition::ISpeechRecognizer2

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizerFactory;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerFactory

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizerStateChangedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerStateChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizerStatics;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerStatics

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizerStatics2;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2 ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerStatics2

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizerTimeouts;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerTimeouts

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognizerUIOptions;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerUIOptions

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface IVoiceCommandManager;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager ABI::Windows::Media::SpeechRecognition::IVoiceCommandManager

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface IVoiceCommandSet;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet ABI::Windows::Media::SpeechRecognition::IVoiceCommandSet

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionCompilationResult;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a392249a-e28a-564a-9e73-1dda63ca643c"))
IAsyncOperation<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionCompilationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionCompilationResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionCompilationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionCompilationResult*> __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("78c859bd-14d4-5c40-abff-490616d5e92d"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionCompilationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionCompilationResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionCompilationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionCompilationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionResult;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ba3eebe8-8d7c-51f2-9ed4-ebafe3674db4"))
IAsyncOperation<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.SpeechRecognition.SpeechRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c2195c7d-dcc2-5c6d-9162-c8df66528762"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.SpeechRecognition.SpeechRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE */

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



#ifndef DEF___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bcde03ad-ea71-5077-a961-1c0ecff57202"))
IKeyValuePair<HSTRING, __FIVectorView_1_HSTRING*> : IKeyValuePair_impl<HSTRING, __FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, __FIVectorView_1_HSTRING*> __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("643b6d83-457e-5a43-800f-b0449f91d96b"))
IIterator<__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVectorView`1<String>>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a4cd6151-2cc1-56f1-9014-df6ba3410beb"))
IIterable<__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVectorView`1<String>>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class VoiceCommandSet;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2bda6840-c39e-5534-8099-22004ab33fc2"))
IKeyValuePair<HSTRING, ABI::Windows::Media::SpeechRecognition::VoiceCommandSet*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::VoiceCommandSet*, ABI::Windows::Media::SpeechRecognition::IVoiceCommandSet*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Media.SpeechRecognition.VoiceCommandSet>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Media::SpeechRecognition::VoiceCommandSet*> __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE */

#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d19b13f3-9058-53bd-8d53-3ee1ea84389b"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Media.SpeechRecognition.VoiceCommandSet>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE */

#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("86e4878a-45de-5203-be77-81c04c9b44e6"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Media.SpeechRecognition.VoiceCommandSet>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE */

#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class Language;
        } /* Globalization */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguage;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguage ABI::Windows::Globalization::ILanguage

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGlobalization__CLanguage_USE
#define DEF___FIIterator_1_Windows__CGlobalization__CLanguage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("30e99ae6-f414-5243-8db2-aab38ea3f1f1"))
IIterator<ABI::Windows::Globalization::Language*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Language*, ABI::Windows::Globalization::ILanguage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Globalization.Language>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Globalization::Language*> __FIIterator_1_Windows__CGlobalization__CLanguage_t;
#define __FIIterator_1_Windows__CGlobalization__CLanguage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGlobalization__CLanguage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGlobalization__CLanguage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGlobalization__CLanguage_USE
#define DEF___FIIterable_1_Windows__CGlobalization__CLanguage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("48409a10-61b6-5db1-a69d-8abc46ac608a"))
IIterable<ABI::Windows::Globalization::Language*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Language*, ABI::Windows::Globalization::ILanguage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Language>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Globalization::Language*> __FIIterable_1_Windows__CGlobalization__CLanguage_t;
#define __FIIterable_1_Windows__CGlobalization__CLanguage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGlobalization__CLanguage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGlobalization__CLanguage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#define DEF___FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("738f00b1-e18c-5140-a53a-f1788d10c93d"))
IIterator<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> : IIterator_impl<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t;
#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#define DEF___FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("88e6436c-3253-520b-9ed8-a63b178c44a2"))
IIterable<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> : IIterable_impl<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t;
#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#define DEF___FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("20756dd2-6d3f-5409-846a-0f0f01d7bf9a"))
IIterator<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.SpeechRecognition.SpeechRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t;
#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#define DEF___FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d9b7b48-98a1-5b22-9a66-6f812f5947aa"))
IIterable<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.SpeechRecognition.SpeechRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t;
#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapView_2_HSTRING___FIVectorView_1_HSTRING_USE
#define DEF___FIMapView_2_HSTRING___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2843d34f-d3e5-5fca-9fdc-b568dd5c1e64"))
IMapView<HSTRING, __FIVectorView_1_HSTRING*> : IMapView_impl<HSTRING, __FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, __FIVectorView_1_HSTRING*> __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_t;
#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING___FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING___FIVectorView_1_HSTRING_USE */


#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#define DEF___FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("121115fd-3858-5888-8626-b69ded969bd6"))
IMapView<HSTRING, ABI::Windows::Media::SpeechRecognition::VoiceCommandSet*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::VoiceCommandSet*, ABI::Windows::Media::SpeechRecognition::IVoiceCommandSet*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Media.SpeechRecognition.VoiceCommandSet>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Media::SpeechRecognition::VoiceCommandSet*> __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t;
#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_USE */

#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGlobalization__CLanguage_USE
#define DEF___FIVectorView_1_Windows__CGlobalization__CLanguage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("144b0f3d-2d59-5dd2-b012-908ec3e06435"))
IVectorView<ABI::Windows::Globalization::Language*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Language*, ABI::Windows::Globalization::ILanguage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Language>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Globalization::Language*> __FIVectorView_1_Windows__CGlobalization__CLanguage_t;
#define __FIVectorView_1_Windows__CGlobalization__CLanguage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGlobalization__CLanguage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGlobalization__CLanguage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#define DEF___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("341dee1d-6ac2-5d06-9026-b30ada205665"))
IVectorView<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> : IVectorView_impl<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t;
#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#define DEF___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0e37810f-1de6-5199-833f-5a6b0bd91e23"))
IVectorView<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.SpeechRecognition.SpeechRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResult*> __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t;
#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_USE */

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

#ifndef DEF___FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#define DEF___FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2691d763-561e-5060-bbc9-7b07361acc95"))
IVector<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> : IVector_impl<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint*> __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t;
#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechContinuousRecognitionSession;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechContinuousRecognitionCompletedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8103c018-7952-59f9-9f41-23b17d6e452d"))
ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession*, ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession*, ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionCompletedEventArgs*, ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession, Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession*, ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionCompletedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechContinuousRecognitionResultGeneratedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("26192073-a2c9-527d-9bd3-911c05e0011e"))
ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession*, ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionResultGeneratedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession*, ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionResultGeneratedEventArgs*, ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionResultGeneratedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession, Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession*, ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionResultGeneratedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognizer;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionHypothesisGeneratedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4cb45aba-7573-545a-b29a-e9be35bd4682"))
ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::SpeechRecognitionHypothesisGeneratedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionHypothesisGeneratedEventArgs*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionHypothesisGeneratedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SpeechRecognition.SpeechRecognizer, Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::SpeechRecognitionHypothesisGeneratedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionQualityDegradingEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("15ca7918-61d2-57b2-b933-44063e8bb662"))
ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::SpeechRecognitionQualityDegradingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognitionQualityDegradingEventArgs*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionQualityDegradingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SpeechRecognition.SpeechRecognizer, Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::SpeechRecognitionQualityDegradingEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognizerStateChangedEventArgs;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d1185e92-5c30-5561-b3e2-e82ddbd872c3"))
ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::SpeechRecognizerStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognizer*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechRecognition::SpeechRecognizerStateChangedEventArgs*, ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SpeechRecognition.SpeechRecognizer, Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SpeechRecognition::SpeechRecognizer*, ABI::Windows::Media::SpeechRecognition::SpeechRecognizerStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechContinuousRecognitionMode : int SpeechContinuousRecognitionMode;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognitionAudioProblem : int SpeechRecognitionAudioProblem;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognitionConfidence : int SpeechRecognitionConfidence;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognitionConstraintProbability : int SpeechRecognitionConstraintProbability;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognitionConstraintType : int SpeechRecognitionConstraintType;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognitionResultStatus : int SpeechRecognitionResultStatus;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognitionScenario : int SpeechRecognitionScenario;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                typedef enum SpeechRecognizerState : int SpeechRecognizerState;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionGrammarFileConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionHypothesis;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionListConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionSemanticInterpretation;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionTopicConstraint;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognizerTimeouts;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognizerUIOptions;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechContinuousRecognitionMode : int
                {
                    SpeechContinuousRecognitionMode_Default = 0,
                    SpeechContinuousRecognitionMode_PauseOnRecognition = 1,
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognitionAudioProblem : int
                {
                    SpeechRecognitionAudioProblem_None = 0,
                    SpeechRecognitionAudioProblem_TooNoisy = 1,
                    SpeechRecognitionAudioProblem_NoSignal = 2,
                    SpeechRecognitionAudioProblem_TooLoud = 3,
                    SpeechRecognitionAudioProblem_TooQuiet = 4,
                    SpeechRecognitionAudioProblem_TooFast = 5,
                    SpeechRecognitionAudioProblem_TooSlow = 6,
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionConfidence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognitionConfidence : int
                {
                    SpeechRecognitionConfidence_High = 0,
                    SpeechRecognitionConfidence_Medium = 1,
                    SpeechRecognitionConfidence_Low = 2,
                    SpeechRecognitionConfidence_Rejected = 3,
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognitionConstraintProbability : int
                {
                    SpeechRecognitionConstraintProbability_Default = 0,
                    SpeechRecognitionConstraintProbability_Min = 1,
                    SpeechRecognitionConstraintProbability_Max = 2,
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognitionConstraintType : int
                {
                    SpeechRecognitionConstraintType_Topic = 0,
                    SpeechRecognitionConstraintType_List = 1,
                    SpeechRecognitionConstraintType_Grammar = 2,
                    SpeechRecognitionConstraintType_VoiceCommandDefinition = 3,
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognitionResultStatus : int
                {
                    SpeechRecognitionResultStatus_Success = 0,
                    SpeechRecognitionResultStatus_TopicLanguageNotSupported = 1,
                    SpeechRecognitionResultStatus_GrammarLanguageMismatch = 2,
                    SpeechRecognitionResultStatus_GrammarCompilationFailure = 3,
                    SpeechRecognitionResultStatus_AudioQualityFailure = 4,
                    SpeechRecognitionResultStatus_UserCanceled = 5,
                    SpeechRecognitionResultStatus_Unknown = 6,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognitionResultStatus_TimeoutExceeded = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognitionResultStatus_PauseLimitExceeded = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognitionResultStatus_NetworkFailure = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognitionResultStatus_MicrophoneUnavailable = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognitionScenario : int
                {
                    SpeechRecognitionScenario_WebSearch = 0,
                    SpeechRecognitionScenario_Dictation = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognitionScenario_FormFilling = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognizerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                enum SpeechRecognizerState : int
                {
                    SpeechRecognizerState_Idle = 0,
                    SpeechRecognizerState_Capturing = 1,
                    SpeechRecognizerState_Processing = 2,
                    SpeechRecognizerState_SoundStarted = 3,
                    SpeechRecognizerState_SoundEnded = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognizerState_SpeechDetected = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SpeechRecognizerState_Paused = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechContinuousRecognitionCompletedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("e3d069bb-e30c-5e18-424b-7fbe81f8fbd0")
                ISpeechContinuousRecognitionCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResultStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechContinuousRecognitionCompletedEventArgs = __uuidof(ISpeechContinuousRecognitionCompletedEventArgs);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechContinuousRecognitionResultGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("19091e1e-6e7e-5a46-40fb-76594f786504")
                ISpeechContinuousRecognitionResultGeneratedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Result(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechContinuousRecognitionResultGeneratedEventArgs = __uuidof(ISpeechContinuousRecognitionResultGeneratedEventArgs);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechContinuousRecognitionSession[] = L"Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("6a213c04-6614-49f8-99a2-b5e9b3a085c8")
                ISpeechContinuousRecognitionSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AutoStopSilenceTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoStopSilenceTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartWithModeAsync(
                        ABI::Windows::Media::SpeechRecognition::SpeechContinuousRecognitionMode mode,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CancelAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PauseAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Resume(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Completed(
                        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs* value,
                        EventRegistrationToken* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Completed(
                        EventRegistrationToken value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ResultGenerated(
                        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs* value,
                        EventRegistrationToken* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ResultGenerated(
                        EventRegistrationToken value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechContinuousRecognitionSession = __uuidof(ISpeechContinuousRecognitionSession);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionCompilationResult[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("407e6c5d-6ac7-4da4-9cc1-2fce32cf7489")
                ISpeechRecognitionCompilationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResultStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionCompilationResult = __uuidof(ISpeechRecognitionCompilationResult);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("79ac1628-4d68-43c4-8911-40dc4101b55b")
                ISpeechRecognitionConstraint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionConstraintType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Probability(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionConstraintProbability* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Probability(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionConstraintProbability value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionConstraint = __uuidof(ISpeechRecognitionConstraint);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionGrammarFileConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("b5031a8f-85ca-4fa4-b11a-474fc41b3835")
                ISpeechRecognitionGrammarFileConstraint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_GrammarFile(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionGrammarFileConstraint = __uuidof(ISpeechRecognitionGrammarFileConstraint);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionGrammarFileConstraintFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("3da770eb-c479-4c27-9f19-89974ef392d1")
                ISpeechRecognitionGrammarFileConstraintFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionGrammarFileConstraint** constraint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTag(
                        ABI::Windows::Storage::IStorageFile* file,
                        HSTRING tag,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionGrammarFileConstraint** constraint
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionGrammarFileConstraintFactory = __uuidof(ISpeechRecognitionGrammarFileConstraintFactory);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionHypothesis[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("7a7b25b0-99c5-4f7d-bf84-10aa1302b634")
                ISpeechRecognitionHypothesis : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionHypothesis = __uuidof(ISpeechRecognitionHypothesis);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionHypothesisGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("55161a7a-8023-5866-411d-1213bb271476")
                ISpeechRecognitionHypothesisGeneratedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Hypothesis(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionHypothesis** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionHypothesisGeneratedEventArgs = __uuidof(ISpeechRecognitionHypothesisGeneratedEventArgs);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionListConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("09c487e9-e4ad-4526-81f2-4946fb481d98")
                ISpeechRecognitionListConstraint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Commands(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionListConstraint = __uuidof(ISpeechRecognitionListConstraint);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionListConstraintFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("40f3cdc7-562a-426a-9f3b-3b4e282be1d5")
                ISpeechRecognitionListConstraintFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1_HSTRING* commands,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionListConstraint** constraint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTag(
                        __FIIterable_1_HSTRING* commands,
                        HSTRING tag,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionListConstraint** constraint
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionListConstraintFactory = __uuidof(ISpeechRecognitionListConstraintFactory);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionQualityDegradingEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("4fe24105-8c3a-4c7e-8d0a-5bd4f5b14ad8")
                ISpeechRecognitionQualityDegradingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Problem(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionAudioProblem* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionQualityDegradingEventArgs = __uuidof(ISpeechRecognitionQualityDegradingEventArgs);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionResult[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("4e303157-034e-4652-857e-d0454cc4beec")
                ISpeechRecognitionResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionResultStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Confidence(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionConfidence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SemanticInterpretation(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionSemanticInterpretation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAlternates(
                        UINT32 maxAlternates,
                        __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** alternates
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Constraint(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionConstraint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RulePath(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawConfidence(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionResult = __uuidof(ISpeechRecognitionResult);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionResult2[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("af7ed1ba-451b-4166-a0c1-1ffe84032d03")
                ISpeechRecognitionResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PhraseStartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhraseDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionResult2 = __uuidof(ISpeechRecognitionResult2);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionSemanticInterpretation[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("aae1da9b-7e32-4c1f-89fe-0c65f486f52e")
                ISpeechRecognitionSemanticInterpretation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING___FIVectorView_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionSemanticInterpretation = __uuidof(ISpeechRecognitionSemanticInterpretation);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionTopicConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("bf6fdf19-825d-4e69-a681-36e48cf1c93e")
                ISpeechRecognitionTopicConstraint : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Scenario(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionScenario* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TopicHint(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionTopicConstraint = __uuidof(ISpeechRecognitionTopicConstraint);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionTopicConstraintFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("6e6863df-ec05-47d7-a5df-56a3431e58d2")
                ISpeechRecognitionTopicConstraintFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionScenario scenario,
                        HSTRING topicHint,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionTopicConstraint** constraint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTag(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognitionScenario scenario,
                        HSTRING topicHint,
                        HSTRING tag,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionTopicConstraint** constraint
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionTopicConstraintFactory = __uuidof(ISpeechRecognitionTopicConstraintFactory);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionVoiceCommandDefinitionConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("f2791c2b-1ef4-4ae7-9d77-b6ff10b8a3c2")
                ISpeechRecognitionVoiceCommandDefinitionConstraint : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognitionVoiceCommandDefinitionConstraint = __uuidof(ISpeechRecognitionVoiceCommandDefinitionConstraint);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizer[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizer";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("0bc3c9cb-c26a-40f2-aeb5-8096b2e48073")
                ISpeechRecognizer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentLanguage(
                        ABI::Windows::Globalization::ILanguage** language
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Constraints(
                        __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timeouts(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerTimeouts** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UIOptions(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognizerUIOptions** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CompileConstraintsAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RecognizeAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RecognizeWithUIAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RecognitionQualityDegrading(
                        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs* speechRecognitionQualityDegradingHandler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RecognitionQualityDegrading(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs* stateChangedHandler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizer = __uuidof(ISpeechRecognizer);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizer2[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizer2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("63c9baf1-91e3-4ea4-86a1-7c3867d084a6")
                ISpeechRecognizer2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContinuousRecognitionSession(
                        ABI::Windows::Media::SpeechRecognition::ISpeechContinuousRecognitionSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognizerState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopRecognitionAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HypothesisGenerated(
                        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs* value,
                        EventRegistrationToken* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HypothesisGenerated(
                        EventRegistrationToken value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizer2 = __uuidof(ISpeechRecognizer2);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("60c488dd-7fb8-4033-ac70-d046f64818e1")
                ISpeechRecognizerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Globalization::ILanguage* language,
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognizer** recognizer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizerFactory = __uuidof(ISpeechRecognizerFactory);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerStateChangedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("563d4f09-ba03-4bad-ad81-ddc6c4dab0c3")
                ISpeechRecognizerStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::SpeechRecognition::SpeechRecognizerState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizerStateChangedEventArgs = __uuidof(ISpeechRecognizerStateChangedEventArgs);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerStatics[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("87a35eac-a7dc-4b0b-bcc9-24f47c0b7ebf")
                ISpeechRecognizerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SystemSpeechLanguage(
                        ABI::Windows::Globalization::ILanguage** language
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedTopicLanguages(
                        __FIVectorView_1_Windows__CGlobalization__CLanguage** languages
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedGrammarLanguages(
                        __FIVectorView_1_Windows__CGlobalization__CLanguage** languages
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizerStatics = __uuidof(ISpeechRecognizerStatics);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerStatics2[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("1d1b0d95-7565-4ef9-a2f3-ba15162a96cf")
                ISpeechRecognizerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TrySetSystemSpeechLanguageAsync(
                        ABI::Windows::Globalization::ILanguage* speechLanguage,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizerStatics2 = __uuidof(ISpeechRecognizerStatics2);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerTimeouts[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("2ef76fca-6a3c-4dca-a153-df1bc88a79af")
                ISpeechRecognizerTimeouts : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InitialSilenceTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InitialSilenceTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndSilenceTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EndSilenceTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BabbleTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BabbleTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizerTimeouts = __uuidof(ISpeechRecognizerTimeouts);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerUIOptions[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("7888d641-b92b-44ba-a25f-d1864630641f")
                ISpeechRecognizerUIOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExampleText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExampleText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudiblePrompt(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AudiblePrompt(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsReadBackEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsReadBackEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShowConfirmation(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShowConfirmation(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechRecognizerUIOptions = __uuidof(ISpeechRecognizerUIOptions);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.IVoiceCommandManager
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.VoiceCommandManager
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_IVoiceCommandManager[] = L"Windows.Media.SpeechRecognition.IVoiceCommandManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("aa3a8dd5-b6e7-4ee2-baa9-dd6baced0a2b")
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                IVoiceCommandManager : public IInspectable
                {
                public:
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE InstallCommandSetsFromStorageFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Foundation::IAsyncAction** installAction
                        ) = 0;
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_InstalledCommandSets(
                        __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** voiceCommandSets
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoiceCommandManager = __uuidof(IVoiceCommandManager);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.IVoiceCommandSet
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.VoiceCommandSet
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_IVoiceCommandSet[] = L"Windows.Media.SpeechRecognition.IVoiceCommandSet";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                MIDL_INTERFACE("0bedda75-46e6-4b11-a088-5c68632899b5")
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                IVoiceCommandSet : public IInspectable
                {
                public:
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SetPhraseListAsync(
                        HSTRING phraseListName,
                        __FIIterable_1_HSTRING* phraseList,
                        ABI::Windows::Foundation::IAsyncAction** updateAction
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoiceCommandSet = __uuidof(IVoiceCommandSet);
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionCompletedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionResultGeneratedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionResultGeneratedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionResultGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionSession_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionSession[] = L"Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionCompilationResult_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionCompilationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionCompilationResult[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionGrammarFileConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionGrammarFileConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionGrammarFileConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesis_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesis_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesis[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesisGeneratedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesisGeneratedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesisGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionListConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionListConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionListConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionQualityDegradingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionQualityDegradingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionQualityDegradingEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionResult ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionResult_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionResult[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionSemanticInterpretation_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionSemanticInterpretation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionSemanticInterpretation[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionTopicConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionTopicConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionTopicConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionVoiceCommandDefinitionConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionVoiceCommandDefinitionConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionVoiceCommandDefinitionConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognizerFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.SpeechRecognition.ISpeechRecognizerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizer ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.SpeechRecognition.ISpeechRecognizer2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizer_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizer[] = L"Windows.Media.SpeechRecognition.SpeechRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizerStateChangedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerTimeouts_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerTimeouts_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizerTimeouts[] = L"Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerUIOptions_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerUIOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizerUIOptions[] = L"Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.VoiceCommandManager
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.SpeechRecognition.IVoiceCommandManager interface starting with version 1.0 of the Windows.Phone.PhoneContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandManager_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandManager_DEFINED
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_VoiceCommandManager[] = L"Windows.Media.SpeechRecognition.VoiceCommandManager";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.VoiceCommandSet
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.IVoiceCommandSet ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandSet_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandSet_DEFINED
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_VoiceCommandSet[] = L"Windows.Media.SpeechRecognition.VoiceCommandSet";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2 __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2 __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2 __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__
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

#if !defined(____FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguage __x_ABI_CWindows_CGlobalization_CILanguage;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGlobalization__CLanguage __FIIterator_1_Windows__CGlobalization__CLanguage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGlobalization__CLanguage;

typedef struct __FIIterator_1_Windows__CGlobalization__CLanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGlobalization__CLanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGlobalization__CLanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CILanguage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGlobalization__CLanguageVtbl;

interface __FIIterator_1_Windows__CGlobalization__CLanguage
{
    CONST_VTBL struct __FIIterator_1_Windows__CGlobalization__CLanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGlobalization__CLanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGlobalization__CLanguage __FIIterable_1_Windows__CGlobalization__CLanguage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGlobalization__CLanguage;

typedef struct __FIIterable_1_Windows__CGlobalization__CLanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGlobalization__CLanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGlobalization__CLanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        __FIIterator_1_Windows__CGlobalization__CLanguage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGlobalization__CLanguageVtbl;

interface __FIIterable_1_Windows__CGlobalization__CLanguage
{
    CONST_VTBL struct __FIIterable_1_Windows__CGlobalization__CLanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGlobalization__CLanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

typedef struct __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl;

interface __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

typedef struct __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        __FIIterator_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl;

interface __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

typedef struct __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl;

interface __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

typedef struct __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __FIIterator_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl;

interface __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING___FIVectorView_1_HSTRING __FIMapView_2_HSTRING___FIVectorView_1_HSTRING;

#if !defined(____FIMapView_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING___FIVectorView_1_HSTRING __FIMapView_2_HSTRING___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING___FIVectorView_1_HSTRING;

typedef struct __FIMapView_2_HSTRING___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING key,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING___FIVectorView_1_HSTRING* This,
        __FIMapView_2_HSTRING___FIVectorView_1_HSTRING** first,
        __FIMapView_2_HSTRING___FIVectorView_1_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING___FIVectorView_1_HSTRINGVtbl;

interface __FIMapView_2_HSTRING___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING___FIVectorView_1_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet;

typedef struct __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING key,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet* This,
        __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** first,
        __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl;

interface __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGlobalization__CLanguage __FIVectorView_1_Windows__CGlobalization__CLanguage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGlobalization__CLanguage;

typedef struct __FIVectorView_1_Windows__CGlobalization__CLanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        UINT32 index,
        __x_ABI_CWindows_CGlobalization_CILanguage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        __x_ABI_CWindows_CGlobalization_CILanguage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CILanguage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGlobalization__CLanguageVtbl;

interface __FIVectorView_1_Windows__CGlobalization__CLanguage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGlobalization__CLanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

typedef struct __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl;

interface __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult;

typedef struct __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl;

interface __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult_INTERFACE_DEFINED__
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
#if !defined(____FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint;

typedef struct __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl;

interface __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* sender,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* sender,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* sender,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* sender,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* sender,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechContinuousRecognitionMode __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechContinuousRecognitionMode;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionAudioProblem __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionAudioProblem;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConfidence __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConfidence;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintProbability __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintProbability;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintType __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintType;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionResultStatus __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionResultStatus;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionScenario __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionScenario;

typedef enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognizerState __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognizerState;

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechContinuousRecognitionMode
{
    SpeechContinuousRecognitionMode_Default = 0,
    SpeechContinuousRecognitionMode_PauseOnRecognition = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionAudioProblem
{
    SpeechRecognitionAudioProblem_None = 0,
    SpeechRecognitionAudioProblem_TooNoisy = 1,
    SpeechRecognitionAudioProblem_NoSignal = 2,
    SpeechRecognitionAudioProblem_TooLoud = 3,
    SpeechRecognitionAudioProblem_TooQuiet = 4,
    SpeechRecognitionAudioProblem_TooFast = 5,
    SpeechRecognitionAudioProblem_TooSlow = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionConfidence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConfidence
{
    SpeechRecognitionConfidence_High = 0,
    SpeechRecognitionConfidence_Medium = 1,
    SpeechRecognitionConfidence_Low = 2,
    SpeechRecognitionConfidence_Rejected = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintProbability
{
    SpeechRecognitionConstraintProbability_Default = 0,
    SpeechRecognitionConstraintProbability_Min = 1,
    SpeechRecognitionConstraintProbability_Max = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintType
{
    SpeechRecognitionConstraintType_Topic = 0,
    SpeechRecognitionConstraintType_List = 1,
    SpeechRecognitionConstraintType_Grammar = 2,
    SpeechRecognitionConstraintType_VoiceCommandDefinition = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionResultStatus
{
    SpeechRecognitionResultStatus_Success = 0,
    SpeechRecognitionResultStatus_TopicLanguageNotSupported = 1,
    SpeechRecognitionResultStatus_GrammarLanguageMismatch = 2,
    SpeechRecognitionResultStatus_GrammarCompilationFailure = 3,
    SpeechRecognitionResultStatus_AudioQualityFailure = 4,
    SpeechRecognitionResultStatus_UserCanceled = 5,
    SpeechRecognitionResultStatus_Unknown = 6,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognitionResultStatus_TimeoutExceeded = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognitionResultStatus_PauseLimitExceeded = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognitionResultStatus_NetworkFailure = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognitionResultStatus_MicrophoneUnavailable = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognitionScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionScenario
{
    SpeechRecognitionScenario_WebSearch = 0,
    SpeechRecognitionScenario_Dictation = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognitionScenario_FormFilling = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SpeechRecognition.SpeechRecognizerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognizerState
{
    SpeechRecognizerState_Idle = 0,
    SpeechRecognizerState_Capturing = 1,
    SpeechRecognizerState_Processing = 2,
    SpeechRecognizerState_SoundStarted = 3,
    SpeechRecognizerState_SoundEnded = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognizerState_SpeechDetected = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SpeechRecognizerState_Paused = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechContinuousRecognitionCompletedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionResultStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechContinuousRecognitionResultGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionResultGeneratedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechContinuousRecognitionSession[] = L"Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutoStopSilenceTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoStopSilenceTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* StartWithModeAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechContinuousRecognitionMode mode,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* CancelAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* PauseAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* Resume)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This);
    HRESULT (STDMETHODCALLTYPE* add_Completed)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionCompletedEventArgs* value,
        EventRegistrationToken* returnValue);
    HRESULT (STDMETHODCALLTYPE* remove_Completed)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        EventRegistrationToken value);
    HRESULT (STDMETHODCALLTYPE* add_ResultGenerated)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionSession_Windows__CMedia__CSpeechRecognition__CSpeechContinuousRecognitionResultGeneratedEventArgs* value,
        EventRegistrationToken* returnValue);
    HRESULT (STDMETHODCALLTYPE* remove_ResultGenerated)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession* This,
        EventRegistrationToken value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSessionVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_get_AutoStopSilenceTimeout(This, value) \
    ((This)->lpVtbl->get_AutoStopSilenceTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_put_AutoStopSilenceTimeout(This, value) \
    ((This)->lpVtbl->put_AutoStopSilenceTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_StartAsync(This, value) \
    ((This)->lpVtbl->StartAsync(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_StartWithModeAsync(This, mode, value) \
    ((This)->lpVtbl->StartWithModeAsync(This, mode, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_StopAsync(This, value) \
    ((This)->lpVtbl->StopAsync(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_CancelAsync(This, value) \
    ((This)->lpVtbl->CancelAsync(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_PauseAsync(This, value) \
    ((This)->lpVtbl->PauseAsync(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_Resume(This) \
    ((This)->lpVtbl->Resume(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_add_Completed(This, value, returnValue) \
    ((This)->lpVtbl->add_Completed(This, value, returnValue))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_remove_Completed(This, value) \
    ((This)->lpVtbl->remove_Completed(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_add_ResultGenerated(This, value, returnValue) \
    ((This)->lpVtbl->add_ResultGenerated(This, value, returnValue))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_remove_ResultGenerated(This, value) \
    ((This)->lpVtbl->remove_ResultGenerated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionCompilationResult[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionResultStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResultVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionCompilationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintType* value);
    HRESULT (STDMETHODCALLTYPE* get_Probability)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintProbability* value);
    HRESULT (STDMETHODCALLTYPE* put_Probability)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConstraintProbability value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraintVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_get_Probability(This, value) \
    ((This)->lpVtbl->get_Probability(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_put_Probability(This, value) \
    ((This)->lpVtbl->put_Probability(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionGrammarFileConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GrammarFile)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_get_GrammarFile(This, value) \
    ((This)->lpVtbl->get_GrammarFile(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionGrammarFileConstraintFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint** constraint);
    HRESULT (STDMETHODCALLTYPE* CreateWithTag)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING tag,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraint** constraint);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_Create(This, file, constraint) \
    ((This)->lpVtbl->Create(This, file, constraint))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_CreateWithTag(This, file, tag, constraint) \
    ((This)->lpVtbl->CreateWithTag(This, file, tag, constraint))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionGrammarFileConstraintFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionHypothesis[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionHypothesisGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Hypothesis)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesis** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_get_Hypothesis(This, value) \
    ((This)->lpVtbl->get_Hypothesis(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionHypothesisGeneratedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionListConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Commands)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_get_Commands(This, value) \
    ((This)->lpVtbl->get_Commands(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionListConstraintFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This,
        __FIIterable_1_HSTRING* commands,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint** constraint);
    HRESULT (STDMETHODCALLTYPE* CreateWithTag)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory* This,
        __FIIterable_1_HSTRING* commands,
        HSTRING tag,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraint** constraint);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_Create(This, commands, constraint) \
    ((This)->lpVtbl->Create(This, commands, constraint))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_CreateWithTag(This, commands, tag, constraint) \
    ((This)->lpVtbl->CreateWithTag(This, commands, tag, constraint))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionListConstraintFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionQualityDegradingEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Problem)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionAudioProblem* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_get_Problem(This, value) \
    ((This)->lpVtbl->get_Problem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionQualityDegradingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionResult[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionResult";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionResultStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Confidence)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionConfidence* value);
    HRESULT (STDMETHODCALLTYPE* get_SemanticInterpretation)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation** value);
    HRESULT (STDMETHODCALLTYPE* GetAlternates)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        UINT32 maxAlternates,
        __FIVectorView_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** alternates);
    HRESULT (STDMETHODCALLTYPE* get_Constraint)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionConstraint** value);
    HRESULT (STDMETHODCALLTYPE* get_RulePath)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_RawConfidence)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResultVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_Confidence(This, value) \
    ((This)->lpVtbl->get_Confidence(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_SemanticInterpretation(This, value) \
    ((This)->lpVtbl->get_SemanticInterpretation(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_GetAlternates(This, maxAlternates, alternates) \
    ((This)->lpVtbl->GetAlternates(This, maxAlternates, alternates))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_Constraint(This, value) \
    ((This)->lpVtbl->get_Constraint(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_RulePath(This, value) \
    ((This)->lpVtbl->get_RulePath(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_get_RawConfidence(This, value) \
    ((This)->lpVtbl->get_RawConfidence(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionResult2[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionResult2";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PhraseStartTime)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_PhraseDuration)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_get_PhraseStartTime(This, value) \
    ((This)->lpVtbl->get_PhraseStartTime(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_get_PhraseDuration(This, value) \
    ((This)->lpVtbl->get_PhraseDuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionSemanticInterpretation[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation* This,
        __FIMapView_2_HSTRING___FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretationVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionSemanticInterpretation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionTopicConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Scenario)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionScenario* value);
    HRESULT (STDMETHODCALLTYPE* get_TopicHint)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_get_Scenario(This, value) \
    ((This)->lpVtbl->get_Scenario(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_get_TopicHint(This, value) \
    ((This)->lpVtbl->get_TopicHint(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionTopicConstraintFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionScenario scenario,
        HSTRING topicHint,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint** constraint);
    HRESULT (STDMETHODCALLTYPE* CreateWithTag)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognitionScenario scenario,
        HSTRING topicHint,
        HSTRING tag,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraint** constraint);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_Create(This, scenario, topicHint, constraint) \
    ((This)->lpVtbl->Create(This, scenario, topicHint, constraint))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_CreateWithTag(This, scenario, topicHint, tag, constraint) \
    ((This)->lpVtbl->CreateWithTag(This, scenario, topicHint, tag, constraint))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionTopicConstraintFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognitionVoiceCommandDefinitionConstraint[] = L"Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraintVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraintVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraintVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionVoiceCommandDefinitionConstraint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizer[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizer";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentLanguage)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** language);
    HRESULT (STDMETHODCALLTYPE* get_Constraints)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __FIVector_1_Windows__CMedia__CSpeechRecognition__CISpeechRecognitionConstraint** value);
    HRESULT (STDMETHODCALLTYPE* get_Timeouts)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts** value);
    HRESULT (STDMETHODCALLTYPE* get_UIOptions)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions** value);
    HRESULT (STDMETHODCALLTYPE* CompileConstraintsAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionCompilationResult** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* RecognizeAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* RecognizeWithUIAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionResult** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* add_RecognitionQualityDegrading)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionQualityDegradingEventArgs* speechRecognitionQualityDegradingHandler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_RecognitionQualityDegrading)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognizerStateChangedEventArgs* stateChangedHandler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_get_CurrentLanguage(This, language) \
    ((This)->lpVtbl->get_CurrentLanguage(This, language))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_get_Constraints(This, value) \
    ((This)->lpVtbl->get_Constraints(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_get_Timeouts(This, value) \
    ((This)->lpVtbl->get_Timeouts(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_get_UIOptions(This, value) \
    ((This)->lpVtbl->get_UIOptions(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_CompileConstraintsAsync(This, asyncOperation) \
    ((This)->lpVtbl->CompileConstraintsAsync(This, asyncOperation))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_RecognizeAsync(This, asyncOperation) \
    ((This)->lpVtbl->RecognizeAsync(This, asyncOperation))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_RecognizeWithUIAsync(This, asyncOperation) \
    ((This)->lpVtbl->RecognizeWithUIAsync(This, asyncOperation))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_add_RecognitionQualityDegrading(This, speechRecognitionQualityDegradingHandler, cookie) \
    ((This)->lpVtbl->add_RecognitionQualityDegrading(This, speechRecognitionQualityDegradingHandler, cookie))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_remove_RecognitionQualityDegrading(This, cookie) \
    ((This)->lpVtbl->remove_RecognitionQualityDegrading(This, cookie))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_add_StateChanged(This, stateChangedHandler, cookie) \
    ((This)->lpVtbl->add_StateChanged(This, stateChangedHandler, cookie))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_remove_StateChanged(This, cookie) \
    ((This)->lpVtbl->remove_StateChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizer2[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizer2";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContinuousRecognitionSession)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechContinuousRecognitionSession** value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognizerState* value);
    HRESULT (STDMETHODCALLTYPE* StopRecognitionAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* add_HypothesisGenerated)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        __FITypedEventHandler_2_Windows__CMedia__CSpeechRecognition__CSpeechRecognizer_Windows__CMedia__CSpeechRecognition__CSpeechRecognitionHypothesisGeneratedEventArgs* value,
        EventRegistrationToken* returnValue);
    HRESULT (STDMETHODCALLTYPE* remove_HypothesisGenerated)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2* This,
        EventRegistrationToken value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_get_ContinuousRecognitionSession(This, value) \
    ((This)->lpVtbl->get_ContinuousRecognitionSession(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_StopRecognitionAsync(This, value) \
    ((This)->lpVtbl->StopRecognitionAsync(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_add_HypothesisGenerated(This, value, returnValue) \
    ((This)->lpVtbl->add_HypothesisGenerated(This, value, returnValue))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_remove_HypothesisGenerated(This, value) \
    ((This)->lpVtbl->remove_HypothesisGenerated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerFactory[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerFactory";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory* This,
        __x_ABI_CWindows_CGlobalization_CILanguage* language,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizer** recognizer);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_Create(This, language, recognizer) \
    ((This)->lpVtbl->Create(This, language, recognizer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerStateChangedEventArgs[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CSpeechRecognition_CSpeechRecognizerState* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerStatics[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerStatics";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SystemSpeechLanguage)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** language);
    HRESULT (STDMETHODCALLTYPE* get_SupportedTopicLanguages)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        __FIVectorView_1_Windows__CGlobalization__CLanguage** languages);
    HRESULT (STDMETHODCALLTYPE* get_SupportedGrammarLanguages)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics* This,
        __FIVectorView_1_Windows__CGlobalization__CLanguage** languages);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_get_SystemSpeechLanguage(This, language) \
    ((This)->lpVtbl->get_SystemSpeechLanguage(This, language))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_get_SupportedTopicLanguages(This, languages) \
    ((This)->lpVtbl->get_SupportedTopicLanguages(This, languages))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_get_SupportedGrammarLanguages(This, languages) \
    ((This)->lpVtbl->get_SupportedGrammarLanguages(This, languages))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerStatics2[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TrySetSystemSpeechLanguageAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2* This,
        __x_ABI_CWindows_CGlobalization_CILanguage* speechLanguage,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_TrySetSystemSpeechLanguageAsync(This, speechLanguage, result) \
    ((This)->lpVtbl->TrySetSystemSpeechLanguageAsync(This, speechLanguage, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerTimeouts[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeoutsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InitialSilenceTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_InitialSilenceTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_EndSilenceTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_EndSilenceTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_BabbleTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_BabbleTimeout)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeoutsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeoutsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_get_InitialSilenceTimeout(This, value) \
    ((This)->lpVtbl->get_InitialSilenceTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_put_InitialSilenceTimeout(This, value) \
    ((This)->lpVtbl->put_InitialSilenceTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_get_EndSilenceTimeout(This, value) \
    ((This)->lpVtbl->get_EndSilenceTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_put_EndSilenceTimeout(This, value) \
    ((This)->lpVtbl->put_EndSilenceTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_get_BabbleTimeout(This, value) \
    ((This)->lpVtbl->get_BabbleTimeout(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_put_BabbleTimeout(This, value) \
    ((This)->lpVtbl->put_BabbleTimeout(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerTimeouts_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_ISpeechRecognizerUIOptions[] = L"Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions";
typedef struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExampleText)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ExampleText)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AudiblePrompt)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AudiblePrompt)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsReadBackEnabled)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsReadBackEnabled)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShowConfirmation)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShowConfirmation)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptionsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_get_ExampleText(This, value) \
    ((This)->lpVtbl->get_ExampleText(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_put_ExampleText(This, value) \
    ((This)->lpVtbl->put_ExampleText(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_get_AudiblePrompt(This, value) \
    ((This)->lpVtbl->get_AudiblePrompt(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_put_AudiblePrompt(This, value) \
    ((This)->lpVtbl->put_AudiblePrompt(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_get_IsReadBackEnabled(This, value) \
    ((This)->lpVtbl->get_IsReadBackEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_put_IsReadBackEnabled(This, value) \
    ((This)->lpVtbl->put_IsReadBackEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_get_ShowConfirmation(This, value) \
    ((This)->lpVtbl->get_ShowConfirmation(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_put_ShowConfirmation(This, value) \
    ((This)->lpVtbl->put_ShowConfirmation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognizerUIOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.IVoiceCommandManager
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.VoiceCommandManager
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_IVoiceCommandManager[] = L"Windows.Media.SpeechRecognition.IVoiceCommandManager";
typedef struct
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This,
        TrustLevel* trustLevel);
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* InstallCommandSetsFromStorageFileAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** installAction);
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_InstalledCommandSets)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager* This,
        __FIMapView_2_HSTRING_Windows__CMedia__CSpeechRecognition__CVoiceCommandSet** voiceCommandSets);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManagerVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_InstallCommandSetsFromStorageFileAsync(This, file, installAction) \
    ((This)->lpVtbl->InstallCommandSetsFromStorageFileAsync(This, file, installAction))

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_get_InstalledCommandSets(This, voiceCommandSets) \
    ((This)->lpVtbl->get_InstalledCommandSets(This, voiceCommandSets))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechRecognition.IVoiceCommandSet
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechRecognition.VoiceCommandSet
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechRecognition_IVoiceCommandSet[] = L"Windows.Media.SpeechRecognition.IVoiceCommandSet";
typedef struct
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        TrustLevel* trustLevel);
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        HSTRING* value);
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        HSTRING* value);
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SetPhraseListAsync)(__x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet* This,
        HSTRING phraseListName,
        __FIIterable_1_HSTRING* phraseList,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** updateAction);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSetVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
    DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_SetPhraseListAsync(This, phraseListName, phraseList, updateAction) \
    ((This)->lpVtbl->SetPhraseListAsync(This, phraseListName, phraseList, updateAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechRecognition_CIVoiceCommandSet_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionCompletedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionResultGeneratedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionResultGeneratedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionResultGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionSession_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechContinuousRecognitionSession[] = L"Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionCompilationResult_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionCompilationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionCompilationResult[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionGrammarFileConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionGrammarFileConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionGrammarFileConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesis_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesis_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesis[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesisGeneratedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesisGeneratedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionHypothesisGeneratedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionListConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionListConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionListConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionQualityDegradingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionQualityDegradingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionQualityDegradingEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionResult ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionResult_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionResult[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionSemanticInterpretation_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionSemanticInterpretation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionSemanticInterpretation[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionTopicConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionTopicConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionTopicConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint ** Default Interface **
 *    Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionVoiceCommandDefinitionConstraint_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognitionVoiceCommandDefinitionConstraint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognitionVoiceCommandDefinitionConstraint[] = L"Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.SpeechRecognition.ISpeechRecognizerFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.SpeechRecognition.ISpeechRecognizerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizer ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.SpeechRecognition.ISpeechRecognizer2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizer_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizer[] = L"Windows.Media.SpeechRecognition.SpeechRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizerStateChangedEventArgs[] = L"Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerTimeouts_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerTimeouts_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizerTimeouts[] = L"Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerUIOptions_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_SpeechRecognizerUIOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_SpeechRecognizerUIOptions[] = L"Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.VoiceCommandManager
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.SpeechRecognition.IVoiceCommandManager interface starting with version 1.0 of the Windows.Phone.PhoneContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandManager_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandManager_DEFINED
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager instead of Windows.Media.SpeechRecognition.VoiceComandManager. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_VoiceCommandManager[] = L"Windows.Media.SpeechRecognition.VoiceCommandManager";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechRecognition.VoiceCommandSet
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechRecognition.IVoiceCommandSet ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandSet_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechRecognition_VoiceCommandSet_DEFINED
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
DEPRECATED("Use Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition instead of Windows.Media.SpeechRecognition.VoiceComandSet. For more info, see MSDN.")
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechRecognition_VoiceCommandSet[] = L"Windows.Media.SpeechRecognition.VoiceCommandSet";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Espeechrecognition_p_h__

#endif // __windows2Emedia2Espeechrecognition_h__
