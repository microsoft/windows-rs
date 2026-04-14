
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
#ifndef __windows2Eai2Emachinelearning_h__
#define __windows2Eai2Emachinelearning_h__
#ifndef __windows2Eai2Emachinelearning_p_h__
#define __windows2Eai2Emachinelearning_p_h__


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
#if !defined(WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION)
#define WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION)

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
#include "Windows.Graphics.h"
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Media.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface IImageFeatureDescriptor;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor ABI::Windows::AI::MachineLearning::IImageFeatureDescriptor

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface IImageFeatureDescriptor2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2 ABI::Windows::AI::MachineLearning::IImageFeatureDescriptor2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface IImageFeatureValue;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue ABI::Windows::AI::MachineLearning::IImageFeatureValue

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface IImageFeatureValueStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics ABI::Windows::AI::MachineLearning::IImageFeatureValueStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModel;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel ABI::Windows::AI::MachineLearning::ILearningModel

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelBinding;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding ABI::Windows::AI::MachineLearning::ILearningModelBinding

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelBindingFactory;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory ABI::Windows::AI::MachineLearning::ILearningModelBindingFactory

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelDevice;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice ABI::Windows::AI::MachineLearning::ILearningModelDevice

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelDeviceFactory;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory ABI::Windows::AI::MachineLearning::ILearningModelDeviceFactory

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelDeviceStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics ABI::Windows::AI::MachineLearning::ILearningModelDeviceStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelEvaluationResult;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult ABI::Windows::AI::MachineLearning::ILearningModelEvaluationResult

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelFeatureDescriptor;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelFeatureValue;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue ABI::Windows::AI::MachineLearning::ILearningModelFeatureValue

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelOperatorProvider;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider ABI::Windows::AI::MachineLearning::ILearningModelOperatorProvider

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelSession;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession ABI::Windows::AI::MachineLearning::ILearningModelSession

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelSessionFactory;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory ABI::Windows::AI::MachineLearning::ILearningModelSessionFactory

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelSessionFactory2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2 ABI::Windows::AI::MachineLearning::ILearningModelSessionFactory2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelSessionOptions;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions ABI::Windows::AI::MachineLearning::ILearningModelSessionOptions

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelSessionOptions2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2 ABI::Windows::AI::MachineLearning::ILearningModelSessionOptions2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelSessionOptions3;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3 ABI::Windows::AI::MachineLearning::ILearningModelSessionOptions3

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ILearningModelStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics ABI::Windows::AI::MachineLearning::ILearningModelStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface IMapFeatureDescriptor;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor ABI::Windows::AI::MachineLearning::IMapFeatureDescriptor

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ISequenceFeatureDescriptor;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor ABI::Windows::AI::MachineLearning::ISequenceFeatureDescriptor

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensor;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor ABI::Windows::AI::MachineLearning::ITensor

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorBoolean;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean ABI::Windows::AI::MachineLearning::ITensorBoolean

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorBooleanStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics ABI::Windows::AI::MachineLearning::ITensorBooleanStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorBooleanStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2 ABI::Windows::AI::MachineLearning::ITensorBooleanStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorDouble;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble ABI::Windows::AI::MachineLearning::ITensorDouble

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorDoubleStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics ABI::Windows::AI::MachineLearning::ITensorDoubleStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorDoubleStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2 ABI::Windows::AI::MachineLearning::ITensorDoubleStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFeatureDescriptor;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor ABI::Windows::AI::MachineLearning::ITensorFeatureDescriptor

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFloat;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat ABI::Windows::AI::MachineLearning::ITensorFloat

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFloat16Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit ABI::Windows::AI::MachineLearning::ITensorFloat16Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFloat16BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics ABI::Windows::AI::MachineLearning::ITensorFloat16BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFloat16BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2 ABI::Windows::AI::MachineLearning::ITensorFloat16BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFloatStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics ABI::Windows::AI::MachineLearning::ITensorFloatStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorFloatStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2 ABI::Windows::AI::MachineLearning::ITensorFloatStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt16Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit ABI::Windows::AI::MachineLearning::ITensorInt16Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt16BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics ABI::Windows::AI::MachineLearning::ITensorInt16BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt16BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2 ABI::Windows::AI::MachineLearning::ITensorInt16BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt32Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit ABI::Windows::AI::MachineLearning::ITensorInt32Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt32BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics ABI::Windows::AI::MachineLearning::ITensorInt32BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt32BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2 ABI::Windows::AI::MachineLearning::ITensorInt32BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt64Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit ABI::Windows::AI::MachineLearning::ITensorInt64Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt64BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics ABI::Windows::AI::MachineLearning::ITensorInt64BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt64BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2 ABI::Windows::AI::MachineLearning::ITensorInt64BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt8Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit ABI::Windows::AI::MachineLearning::ITensorInt8Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt8BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics ABI::Windows::AI::MachineLearning::ITensorInt8BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorInt8BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2 ABI::Windows::AI::MachineLearning::ITensorInt8BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorString;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString ABI::Windows::AI::MachineLearning::ITensorString

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorStringStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics ABI::Windows::AI::MachineLearning::ITensorStringStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorStringStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2 ABI::Windows::AI::MachineLearning::ITensorStringStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt16Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit ABI::Windows::AI::MachineLearning::ITensorUInt16Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt16BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics ABI::Windows::AI::MachineLearning::ITensorUInt16BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt16BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2 ABI::Windows::AI::MachineLearning::ITensorUInt16BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt32Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit ABI::Windows::AI::MachineLearning::ITensorUInt32Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt32BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics ABI::Windows::AI::MachineLearning::ITensorUInt32BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt32BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2 ABI::Windows::AI::MachineLearning::ITensorUInt32BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt64Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit ABI::Windows::AI::MachineLearning::ITensorUInt64Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt64BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics ABI::Windows::AI::MachineLearning::ITensorUInt64BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt64BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2 ABI::Windows::AI::MachineLearning::ITensorUInt64BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt8Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit ABI::Windows::AI::MachineLearning::ITensorUInt8Bit

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt8BitStatics;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics ABI::Windows::AI::MachineLearning::ITensorUInt8BitStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                interface ITensorUInt8BitStatics2;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2 ABI::Windows::AI::MachineLearning::ITensorUInt8BitStatics2

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class LearningModel;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_USE
#define DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("634ab3cb-406c-5ede-8a89-a7f9ca370326"))
IAsyncOperation<ABI::Windows::AI::MachineLearning::LearningModel*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::LearningModel*, ABI::Windows::AI::MachineLearning::ILearningModel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.AI.MachineLearning.LearningModel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::AI::MachineLearning::LearningModel*> __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_t;
#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("755da6df-ed55-5aaa-b542-c665f010f50c"))
IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::LearningModel*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::LearningModel*, ABI::Windows::AI::MachineLearning::ILearningModel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.AI.MachineLearning.LearningModel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::LearningModel*> __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class LearningModelEvaluationResult;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("28050590-1422-5a18-8c8b-847f2d2cf69a"))
IAsyncOperation<ABI::Windows::AI::MachineLearning::LearningModelEvaluationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::LearningModelEvaluationResult*, ABI::Windows::AI::MachineLearning::ILearningModelEvaluationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.AI.MachineLearning.LearningModelEvaluationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::AI::MachineLearning::LearningModelEvaluationResult*> __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_t;
#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8776114-9adf-50e8-b67f-22e0f1372f45"))
IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::LearningModelEvaluationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::LearningModelEvaluationResult*, ABI::Windows::AI::MachineLearning::ILearningModelEvaluationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.AI.MachineLearning.LearningModelEvaluationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::LearningModelEvaluationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_boolean_USE
#define DEF___FIIterator_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("740a0296-a535-572a-bf0b-17c18ff71fe6"))
IIterator<bool> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<bool> __FIIterator_1_boolean_t;
#define __FIIterator_1_boolean ABI::Windows::Foundation::Collections::__FIIterator_1_boolean_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_boolean_USE */



#ifndef DEF___FIIterable_1_boolean_USE
#define DEF___FIIterable_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("30160817-1d7d-54e9-99db-d7636266a476"))
IIterable<bool> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<bool> __FIIterable_1_boolean_t;
#define __FIIterable_1_boolean ABI::Windows::Foundation::Collections::__FIIterable_1_boolean_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_boolean_USE */



#ifndef DEF___FIIterator_1_double_USE
#define DEF___FIIterator_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("638a2cf4-f474-5318-9055-141cb909ac4b"))
IIterator<double> : IIterator_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<double> __FIIterator_1_double_t;
#define __FIIterator_1_double ABI::Windows::Foundation::Collections::__FIIterator_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_double_USE */



#ifndef DEF___FIIterable_1_double_USE
#define DEF___FIIterable_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c738964e-9c64-5bce-b5ce-61e9a282ec4a"))
IIterable<double> : IIterable_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<double> __FIIterable_1_double_t;
#define __FIIterable_1_double ABI::Windows::Foundation::Collections::__FIIterable_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_double_USE */



#ifndef DEF___FIIterator_1_short_USE
#define DEF___FIIterator_1_short_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5409069f-e7c1-5732-bb69-e5736f03f9a9"))
IIterator<short> : IIterator_impl<short>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Int16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<short> __FIIterator_1_short_t;
#define __FIIterator_1_short ABI::Windows::Foundation::Collections::__FIIterator_1_short_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_short_USE */



#ifndef DEF___FIIterable_1_short_USE
#define DEF___FIIterable_1_short_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("72ff2923-4b4e-53bb-8feb-41ec5f2bb734"))
IIterable<short> : IIterable_impl<short>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Int16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<short> __FIIterable_1_short_t;
#define __FIIterable_1_short ABI::Windows::Foundation::Collections::__FIIterable_1_short_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_short_USE */



#ifndef DEF___FIIterator_1_int_USE
#define DEF___FIIterator_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bfea7f78-50c2-5f1d-a6ea-9e978d2699ff"))
IIterator<int> : IIterator_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<int> __FIIterator_1_int_t;
#define __FIIterator_1_int ABI::Windows::Foundation::Collections::__FIIterator_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_int_USE */



#ifndef DEF___FIIterable_1_int_USE
#define DEF___FIIterable_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("81a643fb-f51c-5565-83c4-f96425777b66"))
IIterable<int> : IIterable_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<int> __FIIterable_1_int_t;
#define __FIIterable_1_int ABI::Windows::Foundation::Collections::__FIIterable_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_int_USE */



#ifndef DEF___FIIterator_1___z__zint64_USE
#define DEF___FIIterator_1___z__zint64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fb98034c-86b7-581f-8cd9-5ad0692201a9"))
IIterator<__int64> : IIterator_impl<__int64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Int64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__int64> __FIIterator_1___z__zint64_t;
#define __FIIterator_1___z__zint64 ABI::Windows::Foundation::Collections::__FIIterator_1___z__zint64_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___z__zint64_USE */



#ifndef DEF___FIIterable_1___z__zint64_USE
#define DEF___FIIterable_1___z__zint64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7784427e-f9cc-518d-964b-e50d5ce727f1"))
IIterable<__int64> : IIterable_impl<__int64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Int64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__int64> __FIIterable_1___z__zint64_t;
#define __FIIterable_1___z__zint64 ABI::Windows::Foundation::Collections::__FIIterable_1___z__zint64_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___z__zint64_USE */



#ifndef DEF___FIIterator_1_float_USE
#define DEF___FIIterator_1_float_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("42614e61-b0aa-5e72-9354-2771db20b7a8"))
IIterator<float> : IIterator_impl<float>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Single>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<float> __FIIterator_1_float_t;
#define __FIIterator_1_float ABI::Windows::Foundation::Collections::__FIIterator_1_float_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_float_USE */



#ifndef DEF___FIIterable_1_float_USE
#define DEF___FIIterable_1_float_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b01bee51-063a-5fda-bd72-d76637bb8cb8"))
IIterable<float> : IIterable_impl<float>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Single>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<float> __FIIterable_1_float_t;
#define __FIIterable_1_float ABI::Windows::Foundation::Collections::__FIIterable_1_float_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_float_USE */



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



#ifndef DEF___FIIterator_1_UINT16_USE
#define DEF___FIIterator_1_UINT16_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5738fc25-402b-5fc1-b1e4-0aa24ef652f1"))
IIterator<UINT16> : IIterator_impl<UINT16>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT16> __FIIterator_1_UINT16_t;
#define __FIIterator_1_UINT16 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT16_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT16_USE */



#ifndef DEF___FIIterable_1_UINT16_USE
#define DEF___FIIterable_1_UINT16_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ecfa9a6f-fa2e-5345-b297-efb4e8c6be87"))
IIterable<UINT16> : IIterable_impl<UINT16>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT16> __FIIterable_1_UINT16_t;
#define __FIIterable_1_UINT16 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT16_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT16_USE */



#ifndef DEF___FIIterator_1_UINT32_USE
#define DEF___FIIterator_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f06a2739-9443-5ef0-b284-dc5aff3e7d10"))
IIterator<UINT32> : IIterator_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT32> __FIIterator_1_UINT32_t;
#define __FIIterator_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT32_USE */



#ifndef DEF___FIIterable_1_UINT32_USE
#define DEF___FIIterable_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("421d4b91-b13b-5f37-ae54-b5249bd80539"))
IIterable<UINT32> : IIterable_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT32> __FIIterable_1_UINT32_t;
#define __FIIterable_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT32_USE */



#ifndef DEF___FIIterator_1_UINT64_USE
#define DEF___FIIterator_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c473ed96-76e3-5ff2-9435-47feebfe9539"))
IIterator<UINT64> : IIterator_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT64> __FIIterator_1_UINT64_t;
#define __FIIterator_1_UINT64 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT64_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT64_USE */



#ifndef DEF___FIIterable_1_UINT64_USE
#define DEF___FIIterable_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b3a3229-7995-5f3c-b248-6c1f7e664f01"))
IIterable<UINT64> : IIterable_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT64> __FIIterable_1_UINT64_t;
#define __FIIterable_1_UINT64 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT64_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT64_USE */



#ifndef DEF___FIIterator_1_byte_USE
#define DEF___FIIterator_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("40556131-a2a1-5fab-aaee-5f35268ca26b"))
IIterator<::byte> : IIterator_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<::byte> __FIIterator_1_byte_t;
#define __FIIterator_1_byte ABI::Windows::Foundation::Collections::__FIIterator_1_byte_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_byte_USE */



#ifndef DEF___FIIterable_1_byte_USE
#define DEF___FIIterable_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("88318266-f3fd-50fc-8f08-b823a41b60c1"))
IIterable<::byte> : IIterable_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<::byte> __FIIterable_1_byte_t;
#define __FIIterable_1_byte ABI::Windows::Foundation::Collections::__FIIterable_1_byte_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_byte_USE */


#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE
#define DEF___FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0ef412a8-a1e6-593a-97f2-0d699ca6a567"))
IIterator<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*> : IIterator_impl<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.AI.MachineLearning.ILearningModelFeatureDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*> __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_t;
#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE
#define DEF___FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0fa50877-6792-56b7-af46-430a8901894a"))
IIterable<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*> : IIterable_impl<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.AI.MachineLearning.ILearningModelFeatureDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*> __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_t;
#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



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



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */



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



#ifndef DEF___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b0d3570-0877-5ec2-8a2c-3b9539506aca"))
IMap<HSTRING, IInspectable*> : IMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, IInspectable*> __FIMap_2_HSTRING_IInspectable_t;
#define __FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIVectorView_1_boolean_USE
#define DEF___FIVectorView_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("243a09cb-6f40-56af-a442-fe81431fbef5"))
IVectorView<bool> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<bool> __FIVectorView_1_boolean_t;
#define __FIVectorView_1_boolean ABI::Windows::Foundation::Collections::__FIVectorView_1_boolean_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_boolean_USE */



#ifndef DEF___FIVectorView_1_double_USE
#define DEF___FIVectorView_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af7586a8-6b21-5f61-bff1-1b682293ad96"))
IVectorView<double> : IVectorView_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<double> __FIVectorView_1_double_t;
#define __FIVectorView_1_double ABI::Windows::Foundation::Collections::__FIVectorView_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_double_USE */



#ifndef DEF___FIVectorView_1_short_USE
#define DEF___FIVectorView_1_short_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e53056ad-8a0e-5c41-a62d-c92e3ac2de58"))
IVectorView<short> : IVectorView_impl<short>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Int16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<short> __FIVectorView_1_short_t;
#define __FIVectorView_1_short ABI::Windows::Foundation::Collections::__FIVectorView_1_short_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_short_USE */



#ifndef DEF___FIVectorView_1_int_USE
#define DEF___FIVectorView_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8d720cdf-3934-5d3f-9a55-40e8063b086a"))
IVectorView<int> : IVectorView_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<int> __FIVectorView_1_int_t;
#define __FIVectorView_1_int ABI::Windows::Foundation::Collections::__FIVectorView_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_int_USE */



#ifndef DEF___FIVectorView_1___z__zint64_USE
#define DEF___FIVectorView_1___z__zint64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8221aa0e-d1d2-5b22-a918-05672812d12f"))
IVectorView<__int64> : IVectorView_impl<__int64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Int64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<__int64> __FIVectorView_1___z__zint64_t;
#define __FIVectorView_1___z__zint64 ABI::Windows::Foundation::Collections::__FIVectorView_1___z__zint64_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1___z__zint64_USE */



#ifndef DEF___FIVectorView_1_float_USE
#define DEF___FIVectorView_1_float_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7bca64fd-150c-5d50-b56b-9f4f474c5930"))
IVectorView<float> : IVectorView_impl<float>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Single>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<float> __FIVectorView_1_float_t;
#define __FIVectorView_1_float ABI::Windows::Foundation::Collections::__FIVectorView_1_float_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_float_USE */



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



#ifndef DEF___FIVectorView_1_UINT16_USE
#define DEF___FIVectorView_1_UINT16_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9d0d0d9f-6a82-55a3-98c5-228499df38f9"))
IVectorView<UINT16> : IVectorView_impl<UINT16>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt16>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT16> __FIVectorView_1_UINT16_t;
#define __FIVectorView_1_UINT16 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT16_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT16_USE */



#ifndef DEF___FIVectorView_1_UINT32_USE
#define DEF___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5ce1a07-8d33-5007-ba64-7d2508ccf85c"))
IVectorView<UINT32> : IVectorView_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT32> __FIVectorView_1_UINT32_t;
#define __FIVectorView_1_UINT32 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT32_USE */



#ifndef DEF___FIVectorView_1_UINT64_USE
#define DEF___FIVectorView_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("23d156c7-7ef9-5096-aaba-1e6c9ab5ceb4"))
IVectorView<UINT64> : IVectorView_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT64> __FIVectorView_1_UINT64_t;
#define __FIVectorView_1_UINT64 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT64_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT64_USE */



#ifndef DEF___FIVectorView_1_byte_USE
#define DEF___FIVectorView_1_byte_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6d05fb29-7885-544e-9382-a1ad391a3fa4"))
IVectorView<::byte> : IVectorView_impl<::byte>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt8>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<::byte> __FIVectorView_1_byte_t;
#define __FIVectorView_1_byte ABI::Windows::Foundation::Collections::__FIVectorView_1_byte_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_byte_USE */


#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE
#define DEF___FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("53e53120-a6e1-527f-af8a-c812902e175e"))
IVectorView<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*> : IVectorView_impl<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.AI.MachineLearning.ILearningModelFeatureDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor*> __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_t;
#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_USE */

#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DDevice;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct DisplayAdapterId DisplayAdapterId;
        } /* Graphics */
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                typedef enum LearningModelDeviceKind : int LearningModelDeviceKind;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                typedef enum LearningModelFeatureKind : int LearningModelFeatureKind;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                typedef enum LearningModelPixelRange : int LearningModelPixelRange;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                typedef enum TensorKind : int TensorKind;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class ImageFeatureValue;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class LearningModelBinding;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class LearningModelDevice;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class LearningModelSession;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class LearningModelSessionOptions;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorBoolean;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorDouble;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorFloat;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorFloat16Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorInt16Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorInt32Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorInt64Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorInt8Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorString;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorUInt16Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorUInt32Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorUInt64Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                class TensorUInt8Bit;
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.AI.MachineLearning.LearningModelDeviceKind
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                enum LearningModelDeviceKind : int
                {
                    LearningModelDeviceKind_Default = 0,
                    LearningModelDeviceKind_Cpu = 1,
                    LearningModelDeviceKind_DirectX = 2,
                    LearningModelDeviceKind_DirectXHighPerformance = 3,
                    LearningModelDeviceKind_DirectXMinPower = 4,
                };
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.LearningModelFeatureKind
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                enum LearningModelFeatureKind : int
                {
                    LearningModelFeatureKind_Tensor = 0,
                    LearningModelFeatureKind_Sequence = 1,
                    LearningModelFeatureKind_Map = 2,
                    LearningModelFeatureKind_Image = 3,
                };
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.LearningModelPixelRange
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 5.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                enum LearningModelPixelRange : int
                {
                    LearningModelPixelRange_ZeroTo255 = 0,
                    LearningModelPixelRange_ZeroToOne = 1,
                    LearningModelPixelRange_MinusOneToOne = 2,
                };
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.AI.MachineLearning.TensorKind
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                enum TensorKind : int
                {
                    TensorKind_Undefined = 0,
                    TensorKind_Float = 1,
                    TensorKind_UInt8 = 2,
                    TensorKind_Int8 = 3,
                    TensorKind_UInt16 = 4,
                    TensorKind_Int16 = 5,
                    TensorKind_Int32 = 6,
                    TensorKind_Int64 = 7,
                    TensorKind_String = 8,
                    TensorKind_Boolean = 9,
                    TensorKind_Float16 = 10,
                    TensorKind_Double = 11,
                    TensorKind_UInt32 = 12,
                    TensorKind_UInt64 = 13,
                    TensorKind_Complex64 = 14,
                    TensorKind_Complex128 = 15,
                };
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureDescriptor[] = L"Windows.AI.MachineLearning.IImageFeatureDescriptor";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("365585a5-171a-4a2a-985f-265159d3895a")
                IImageFeatureDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapPixelFormat(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapAlphaMode(
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageFeatureDescriptor = __uuidof(IImageFeatureDescriptor);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureDescriptor2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureDescriptor2[] = L"Windows.AI.MachineLearning.IImageFeatureDescriptor2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("2b27cca7-d533-5862-bb98-1611b155b0e1")
                IImageFeatureDescriptor2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PixelRange(
                        ABI::Windows::AI::MachineLearning::LearningModelPixelRange* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageFeatureDescriptor2 = __uuidof(IImageFeatureDescriptor2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureValue
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureValue
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureValue[] = L"Windows.AI.MachineLearning.IImageFeatureValue";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("f0414fd9-c9aa-4405-b7fb-94f87c8a3037")
                IImageFeatureValue : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VideoFrame(
                        ABI::Windows::Media::IVideoFrame** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageFeatureValue = __uuidof(IImageFeatureValue);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureValueStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureValue
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureValueStatics[] = L"Windows.AI.MachineLearning.IImageFeatureValueStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("1bc317fd-23cb-4610-b085-c8e1c87ebaa0")
                IImageFeatureValueStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromVideoFrame(
                        ABI::Windows::Media::IVideoFrame* image,
                        ABI::Windows::AI::MachineLearning::IImageFeatureValue** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageFeatureValueStatics = __uuidof(IImageFeatureValueStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModel
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModel
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModel[] = L"Windows.AI.MachineLearning.ILearningModel";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("5b8e4920-489f-4e86-9128-265a327b78fa")
                ILearningModel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Author(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Domain(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Version(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Metadata(
                        __FIMapView_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputFeatures(
                        __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputFeatures(
                        __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModel = __uuidof(ILearningModel);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModel;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelBinding
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelBinding
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelBinding[] = L"Windows.AI.MachineLearning.ILearningModelBinding";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("ea312f20-168f-4f8c-94fe-2e7ac31b4aa8")
                ILearningModelBinding : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Bind(
                        HSTRING name,
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BindWithProperties(
                        HSTRING name,
                        IInspectable* value,
                        ABI::Windows::Foundation::Collections::IPropertySet* props
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelBinding = __uuidof(ILearningModelBinding);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelBindingFactory
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelBinding
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelBindingFactory[] = L"Windows.AI.MachineLearning.ILearningModelBindingFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("c95f7a7a-e788-475e-8917-23aa381faf0b")
                ILearningModelBindingFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromSession(
                        ABI::Windows::AI::MachineLearning::ILearningModelSession* session,
                        ABI::Windows::AI::MachineLearning::ILearningModelBinding** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelBindingFactory = __uuidof(ILearningModelBindingFactory);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelDevice
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelDevice
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelDevice[] = L"Windows.AI.MachineLearning.ILearningModelDevice";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("f5c2c8fe-3f56-4a8c-ac5f-fdb92d8b8252")
                ILearningModelDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AdapterId(
                        ABI::Windows::Graphics::DisplayAdapterId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Direct3D11Device(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelDevice = __uuidof(ILearningModelDevice);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelDeviceFactory
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelDevice
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelDeviceFactory[] = L"Windows.AI.MachineLearning.ILearningModelDeviceFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("9cffd74d-b1e5-4f20-80ad-0a56690db06b")
                ILearningModelDeviceFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::LearningModelDeviceKind deviceKind,
                        ABI::Windows::AI::MachineLearning::ILearningModelDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelDeviceFactory = __uuidof(ILearningModelDeviceFactory);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelDeviceStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelDevice
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelDeviceStatics[] = L"Windows.AI.MachineLearning.ILearningModelDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("49f32107-a8bf-42bb-92c7-10b12dc5d21f")
                ILearningModelDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromDirect3D11Device(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device,
                        ABI::Windows::AI::MachineLearning::ILearningModelDevice** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelDeviceStatics = __uuidof(ILearningModelDeviceStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelEvaluationResult
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelEvaluationResult
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelEvaluationResult[] = L"Windows.AI.MachineLearning.ILearningModelEvaluationResult";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("b2f9bfcd-960e-49c0-8593-eb190ae3eee2")
                ILearningModelEvaluationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CorrelationId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ErrorStatus(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Outputs(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelEvaluationResult = __uuidof(ILearningModelEvaluationResult);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelFeatureDescriptor[] = L"Windows.AI.MachineLearning.ILearningModelFeatureDescriptor";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("bc08cf7c-6ed0-4004-97ba-b9a2eecd2b4f")
                ILearningModelFeatureDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::AI::MachineLearning::LearningModelFeatureKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRequired(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelFeatureDescriptor = __uuidof(ILearningModelFeatureDescriptor);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelFeatureValue
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelFeatureValue[] = L"Windows.AI.MachineLearning.ILearningModelFeatureValue";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("f51005db-4085-4dfe-9fed-95eb0c0cf75c")
                ILearningModelFeatureValue : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::AI::MachineLearning::LearningModelFeatureKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelFeatureValue = __uuidof(ILearningModelFeatureValue);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelOperatorProvider
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelOperatorProvider[] = L"Windows.AI.MachineLearning.ILearningModelOperatorProvider";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("2a222e5d-afb1-47ed-bfad-b5b3a459ec04")
                ILearningModelOperatorProvider : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_ILearningModelOperatorProvider = __uuidof(ILearningModelOperatorProvider);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSession
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSession
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSession[] = L"Windows.AI.MachineLearning.ILearningModelSession";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("8e58f8f6-b787-4c11-90f0-7129aeca74a9")
                ILearningModelSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Model(
                        ABI::Windows::AI::MachineLearning::ILearningModel** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Device(
                        ABI::Windows::AI::MachineLearning::ILearningModelDevice** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EvaluationProperties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EvaluateAsync(
                        ABI::Windows::AI::MachineLearning::ILearningModelBinding* bindings,
                        HSTRING correlationId,
                        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EvaluateFeaturesAsync(
                        __FIMap_2_HSTRING_IInspectable* features,
                        HSTRING correlationId,
                        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Evaluate(
                        ABI::Windows::AI::MachineLearning::ILearningModelBinding* bindings,
                        HSTRING correlationId,
                        ABI::Windows::AI::MachineLearning::ILearningModelEvaluationResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EvaluateFeatures(
                        __FIMap_2_HSTRING_IInspectable* features,
                        HSTRING correlationId,
                        ABI::Windows::AI::MachineLearning::ILearningModelEvaluationResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelSession = __uuidof(ILearningModelSession);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionFactory
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSession
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionFactory[] = L"Windows.AI.MachineLearning.ILearningModelSessionFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("0f6b881d-1c9b-47b6-bfe0-f1cf62a67579")
                ILearningModelSessionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromModel(
                        ABI::Windows::AI::MachineLearning::ILearningModel* model,
                        ABI::Windows::AI::MachineLearning::ILearningModelSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromModelOnDevice(
                        ABI::Windows::AI::MachineLearning::ILearningModel* model,
                        ABI::Windows::AI::MachineLearning::ILearningModelDevice* deviceToRunOn,
                        ABI::Windows::AI::MachineLearning::ILearningModelSession** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelSessionFactory = __uuidof(ILearningModelSessionFactory);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionFactory2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSession
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionFactory2[] = L"Windows.AI.MachineLearning.ILearningModelSessionFactory2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("4e5c88bf-0a1f-5fec-ade0-2fd91e4ef29b")
                ILearningModelSessionFactory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromModelOnDeviceWithSessionOptions(
                        ABI::Windows::AI::MachineLearning::ILearningModel* model,
                        ABI::Windows::AI::MachineLearning::ILearningModelDevice* deviceToRunOn,
                        ABI::Windows::AI::MachineLearning::ILearningModelSessionOptions* learningModelSessionOptions,
                        ABI::Windows::AI::MachineLearning::ILearningModelSession** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelSessionFactory2 = __uuidof(ILearningModelSessionFactory2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionOptions
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionOptions[] = L"Windows.AI.MachineLearning.ILearningModelSessionOptions";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("b8f63fa1-134d-5133-8cff-3a5c3c263beb")
                ILearningModelSessionOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BatchSizeOverride(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BatchSizeOverride(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelSessionOptions = __uuidof(ILearningModelSessionOptions);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionOptions2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionOptions2[] = L"Windows.AI.MachineLearning.ILearningModelSessionOptions2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("6fcd1dc4-175f-5bd2-8de5-2f2006a25adf")
                ILearningModelSessionOptions2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CloseModelOnSessionCreation(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CloseModelOnSessionCreation(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelSessionOptions2 = __uuidof(ILearningModelSessionOptions2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionOptions3
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionOptions3[] = L"Windows.AI.MachineLearning.ILearningModelSessionOptions3";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("58e15cee-d8c2-56fc-92e8-76d751081086")
                ILearningModelSessionOptions3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE OverrideNamedDimension(
                        HSTRING name,
                        UINT32 dimension
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelSessionOptions3 = __uuidof(ILearningModelSessionOptions3);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModel
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelStatics[] = L"Windows.AI.MachineLearning.ILearningModelStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("e3b977e8-6952-4e47-8ef4-1f7f07897c6d")
                ILearningModelStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStorageFileAsync(
                        ABI::Windows::Storage::IStorageFile* modelFile,
                        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* modelStream,
                        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromFilePath(
                        HSTRING filePath,
                        ABI::Windows::AI::MachineLearning::ILearningModel** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStream(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* modelStream,
                        ABI::Windows::AI::MachineLearning::ILearningModel** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStorageFileWithOperatorProviderAsync(
                        ABI::Windows::Storage::IStorageFile* modelFile,
                        ABI::Windows::AI::MachineLearning::ILearningModelOperatorProvider* operatorProvider,
                        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStreamWithOperatorProviderAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* modelStream,
                        ABI::Windows::AI::MachineLearning::ILearningModelOperatorProvider* operatorProvider,
                        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromFilePathWithOperatorProvider(
                        HSTRING filePath,
                        ABI::Windows::AI::MachineLearning::ILearningModelOperatorProvider* operatorProvider,
                        ABI::Windows::AI::MachineLearning::ILearningModel** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStreamWithOperatorProvider(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* modelStream,
                        ABI::Windows::AI::MachineLearning::ILearningModelOperatorProvider* operatorProvider,
                        ABI::Windows::AI::MachineLearning::ILearningModel** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILearningModelStatics = __uuidof(ILearningModelStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IMapFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.MapFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IMapFeatureDescriptor[] = L"Windows.AI.MachineLearning.IMapFeatureDescriptor";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("530424bd-a257-436d-9e60-c2981f7cc5c4")
                IMapFeatureDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_KeyKind(
                        ABI::Windows::AI::MachineLearning::TensorKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ValueDescriptor(
                        ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMapFeatureDescriptor = __uuidof(IMapFeatureDescriptor);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ISequenceFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.SequenceFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ISequenceFeatureDescriptor[] = L"Windows.AI.MachineLearning.ISequenceFeatureDescriptor";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("84f6945a-562b-4d62-a851-739aced96668")
                ISequenceFeatureDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ElementDescriptor(
                        ABI::Windows::AI::MachineLearning::ILearningModelFeatureDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISequenceFeatureDescriptor = __uuidof(ISequenceFeatureDescriptor);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.ILearningModelFeatureValue
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensor[] = L"Windows.AI.MachineLearning.ITensor";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("05489593-a305-4a25-ad09-440119b4b7f6")
                ITensor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TensorKind(
                        ABI::Windows::AI::MachineLearning::TensorKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Shape(
                        __FIVectorView_1___z__zint64** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensor = __uuidof(ITensor);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorBoolean
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorBoolean
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorBoolean[] = L"Windows.AI.MachineLearning.ITensorBoolean";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("50f311ed-29e9-4a5c-a44d-8fc512584eed")
                ITensorBoolean : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorBoolean = __uuidof(ITensorBoolean);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorBooleanStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorBoolean
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorBooleanStatics[] = L"Windows.AI.MachineLearning.ITensorBooleanStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("2796862c-2357-49a7-b476-d0aa3dfe6866")
                ITensorBooleanStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorBoolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorBoolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        boolean* data,
                        ABI::Windows::AI::MachineLearning::ITensorBoolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_boolean* data,
                        ABI::Windows::AI::MachineLearning::ITensorBoolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorBooleanStatics = __uuidof(ITensorBooleanStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorBooleanStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorBoolean
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorBooleanStatics2[] = L"Windows.AI.MachineLearning.ITensorBooleanStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("a3a4a501-6a2d-52d7-b04b-c435baee0115")
                ITensorBooleanStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        boolean* data,
                        ABI::Windows::AI::MachineLearning::ITensorBoolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorBoolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorBooleanStatics2 = __uuidof(ITensorBooleanStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorDouble
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorDouble
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorDouble[] = L"Windows.AI.MachineLearning.ITensorDouble";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("91e41252-7a8f-4f0e-a28f-9637ffc8a3d0")
                ITensorDouble : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_double** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorDouble = __uuidof(ITensorDouble);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorDoubleStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorDouble
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorDoubleStatics[] = L"Windows.AI.MachineLearning.ITensorDoubleStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("a86693c5-9538-44e7-a3ca-5df374a5a70c")
                ITensorDoubleStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorDouble** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorDouble** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        DOUBLE* data,
                        ABI::Windows::AI::MachineLearning::ITensorDouble** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_double* data,
                        ABI::Windows::AI::MachineLearning::ITensorDouble** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorDoubleStatics = __uuidof(ITensorDoubleStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorDoubleStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorDouble
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorDoubleStatics2[] = L"Windows.AI.MachineLearning.ITensorDoubleStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("93a570de-5e9a-5094-85c8-592c655e68ac")
                ITensorDoubleStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        DOUBLE* data,
                        ABI::Windows::AI::MachineLearning::ITensorDouble** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorDouble** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorDoubleStatics2 = __uuidof(ITensorDoubleStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFeatureDescriptor[] = L"Windows.AI.MachineLearning.ITensorFeatureDescriptor";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("74455c80-946a-4310-a19c-ee0af028fce4")
                ITensorFeatureDescriptor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TensorKind(
                        ABI::Windows::AI::MachineLearning::TensorKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Shape(
                        __FIVectorView_1___z__zint64** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFeatureDescriptor = __uuidof(ITensorFeatureDescriptor);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat[] = L"Windows.AI.MachineLearning.ITensorFloat";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("f2282d82-aa02-42c8-a0c8-df1efc9676e1")
                ITensorFloat : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_float** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFloat = __uuidof(ITensorFloat);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat16Bit[] = L"Windows.AI.MachineLearning.ITensorFloat16Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("0ab994fc-5b89-4c3c-b5e4-5282a5316c0a")
                ITensorFloat16Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_float** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFloat16Bit = __uuidof(ITensorFloat16Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat16BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat16BitStatics[] = L"Windows.AI.MachineLearning.ITensorFloat16BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("a52db6f5-318a-44d4-820b-0cdc7054a84a")
                ITensorFloat16BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorFloat16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorFloat16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        FLOAT* data,
                        ABI::Windows::AI::MachineLearning::ITensorFloat16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_float* data,
                        ABI::Windows::AI::MachineLearning::ITensorFloat16Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFloat16BitStatics = __uuidof(ITensorFloat16BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat16BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat16BitStatics2[] = L"Windows.AI.MachineLearning.ITensorFloat16BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("68545726-2dc7-51bf-b470-0b344cc2a1bc")
                ITensorFloat16BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        FLOAT* data,
                        ABI::Windows::AI::MachineLearning::ITensorFloat16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorFloat16Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFloat16BitStatics2 = __uuidof(ITensorFloat16BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloatStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloatStatics[] = L"Windows.AI.MachineLearning.ITensorFloatStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("dbcd395b-3ba3-452f-b10d-3c135e573fa9")
                ITensorFloatStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorFloat** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorFloat** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        FLOAT* data,
                        ABI::Windows::AI::MachineLearning::ITensorFloat** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_float* data,
                        ABI::Windows::AI::MachineLearning::ITensorFloat** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFloatStatics = __uuidof(ITensorFloatStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloatStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloatStatics2[] = L"Windows.AI.MachineLearning.ITensorFloatStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("24610bc1-5e44-5713-b281-8f4ad4d555e8")
                ITensorFloatStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        FLOAT* data,
                        ABI::Windows::AI::MachineLearning::ITensorFloat** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorFloat** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorFloatStatics2 = __uuidof(ITensorFloatStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt16Bit[] = L"Windows.AI.MachineLearning.ITensorInt16Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("98a32d39-e6d6-44af-8afa-baebc44dc020")
                ITensorInt16Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_short** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt16Bit = __uuidof(ITensorInt16Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt16BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt16BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt16BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("98646293-266e-4b1a-821f-e60d70898b91")
                ITensorInt16BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        INT16* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_short* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt16Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt16BitStatics = __uuidof(ITensorInt16BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt16BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt16BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt16BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("0cd70cf4-696c-5e5f-95d8-5ebf9670148b")
                ITensorInt16BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        INT16* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorInt16Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt16BitStatics2 = __uuidof(ITensorInt16BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt32Bit[] = L"Windows.AI.MachineLearning.ITensorInt32Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("2c0c28d3-207c-4486-a7d2-884522c5e589")
                ITensorInt32Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_int** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt32Bit = __uuidof(ITensorInt32Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt32BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt32BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt32BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("6539864b-52fa-4e35-907c-834cac417b50")
                ITensorInt32BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        INT32* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_int* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt32Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt32BitStatics = __uuidof(ITensorInt32BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt32BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt32BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt32BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("7c4b079a-e956-5ce0-a3bd-157d9d79b5ec")
                ITensorInt32BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        INT32* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorInt32Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt32BitStatics2 = __uuidof(ITensorInt32BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt64Bit[] = L"Windows.AI.MachineLearning.ITensorInt64Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("499665ba-1fa2-45ad-af25-a0bd9bda4c87")
                ITensorInt64Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1___z__zint64** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt64Bit = __uuidof(ITensorInt64Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt64BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt64BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt64BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("9648ad9d-1198-4d74-9517-783ab62b9cc2")
                ITensorInt64BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        INT64* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1___z__zint64* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt64Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt64BitStatics = __uuidof(ITensorInt64BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt64BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt64BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt64BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("6d3d9dcb-ff40-5ec2-89fe-084e2b6bc6db")
                ITensorInt64BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        INT64* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorInt64Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt64BitStatics2 = __uuidof(ITensorInt64BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt8Bit[] = L"Windows.AI.MachineLearning.ITensorInt8Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("cddd97c5-ffd8-4fef-aefb-30e1a485b2ee")
                ITensorInt8Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_byte** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt8Bit = __uuidof(ITensorInt8Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt8BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt8BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt8BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("b1a12284-095c-4c76-a661-ac4cee1f3e8b")
                ITensorInt8BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        BYTE* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_byte* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt8Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt8BitStatics = __uuidof(ITensorInt8BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt8BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt8BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt8BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("c0d59637-c468-56fb-9535-c052bdb93dc0")
                ITensorInt8BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        BYTE* data,
                        ABI::Windows::AI::MachineLearning::ITensorInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorInt8Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorInt8BitStatics2 = __uuidof(ITensorInt8BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorString
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorString
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorString[] = L"Windows.AI.MachineLearning.ITensorString";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("582335c8-bdb1-4610-bc75-35e9cbf009b7")
                ITensorString : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_HSTRING** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorString = __uuidof(ITensorString);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorString;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorStringStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorString
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorStringStatics[] = L"Windows.AI.MachineLearning.ITensorStringStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("83623324-cf26-4f17-a2d4-20ef8d097d53")
                ITensorStringStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorString** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorString** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        HSTRING* data,
                        ABI::Windows::AI::MachineLearning::ITensorString** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_HSTRING* data,
                        ABI::Windows::AI::MachineLearning::ITensorString** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorStringStatics = __uuidof(ITensorStringStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorStringStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorString
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorStringStatics2[] = L"Windows.AI.MachineLearning.ITensorStringStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("9e355ed0-c8e2-5254-9137-0193a3668fd8")
                ITensorStringStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        HSTRING* data,
                        ABI::Windows::AI::MachineLearning::ITensorString** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorStringStatics2 = __uuidof(ITensorStringStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt16Bit[] = L"Windows.AI.MachineLearning.ITensorUInt16Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("68140f4b-23c0-42f3-81f6-a891c011bc3f")
                ITensorUInt16Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_UINT16** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt16Bit = __uuidof(ITensorUInt16Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt16BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt16BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt16BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("5df745dd-028a-481a-a27c-c7e6435e52dd")
                ITensorUInt16BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorUInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorUInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        UINT16* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_UINT16* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt16Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt16BitStatics = __uuidof(ITensorUInt16BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt16BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt16BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt16BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("8af40c64-d69f-5315-9348-490877bbd642")
                ITensorUInt16BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        UINT16* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt16Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorUInt16Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt16BitStatics2 = __uuidof(ITensorUInt16BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt32Bit[] = L"Windows.AI.MachineLearning.ITensorUInt32Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("d8c9c2ff-7511-45a3-bfac-c38f370d2237")
                ITensorUInt32Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_UINT32** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt32Bit = __uuidof(ITensorUInt32Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt32BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt32BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt32BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("417c3837-e773-4378-8e7f-0cc33dbea697")
                ITensorUInt32BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorUInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorUInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        UINT32* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_UINT32* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt32Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt32BitStatics = __uuidof(ITensorUInt32BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt32BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt32BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt32BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("ef1a1f1c-314e-569d-b496-5c8447d20cd2")
                ITensorUInt32BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        UINT32* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt32Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorUInt32Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt32BitStatics2 = __uuidof(ITensorUInt32BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt64Bit[] = L"Windows.AI.MachineLearning.ITensorUInt64Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("2e70ffad-04bf-4825-839a-82baef8c7886")
                ITensorUInt64Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_UINT64** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt64Bit = __uuidof(ITensorUInt64Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt64BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt64BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt64BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("7a7e20eb-242f-47cb-a9c6-f602ecfbfee4")
                ITensorUInt64BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorUInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorUInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        UINT64* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_UINT64* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt64Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt64BitStatics = __uuidof(ITensorUInt64BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt64BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt64BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt64BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("085a687d-67e1-5b1e-b232-4fabe9ca20b3")
                ITensorUInt64BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        UINT64* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt64Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorUInt64Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt64BitStatics2 = __uuidof(ITensorUInt64BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt8Bit[] = L"Windows.AI.MachineLearning.ITensorUInt8Bit";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("58e1ae27-622b-48e3-be22-d867aed1daac")
                ITensorUInt8Bit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsVectorView(
                        __FIVectorView_1_byte** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt8Bit = __uuidof(ITensorUInt8Bit);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt8BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt8BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt8BitStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("05f67583-bc24-4220-8a41-2dcd8c5ed33c")
                ITensorUInt8BitStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::AI::MachineLearning::ITensorUInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Create2(
                        __FIIterable_1___z__zint64* shape,
                        ABI::Windows::AI::MachineLearning::ITensorUInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromArray(
                        __FIIterable_1___z__zint64* shape,
                        UINT32 dataLength,
                        BYTE* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromIterable(
                        __FIIterable_1___z__zint64* shape,
                        __FIIterable_1_byte* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt8Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt8BitStatics = __uuidof(ITensorUInt8BitStatics);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt8BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt8BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt8BitStatics2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                MIDL_INTERFACE("2ba042d6-373e-5a3a-a2fc-a6c41bd52789")
                ITensorUInt8BitStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromShapeArrayAndDataArray(
                        UINT32 shapeLength,
                        INT64* shape,
                        UINT32 dataLength,
                        BYTE* data,
                        ABI::Windows::AI::MachineLearning::ITensorUInt8Bit** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        UINT32 shapeLength,
                        INT64* shape,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::AI::MachineLearning::ITensorUInt8Bit** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITensorUInt8BitStatics2 = __uuidof(ITensorUInt8BitStatics2);
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.MachineLearning.ImageFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.IImageFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.IImageFeatureDescriptor2
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_ImageFeatureDescriptor[] = L"Windows.AI.MachineLearning.ImageFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.ImageFeatureValue
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.IImageFeatureValueStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.IImageFeatureValue ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureValue_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_ImageFeatureValue[] = L"Windows.AI.MachineLearning.ImageFeatureValue";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModel
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ILearningModelStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModel ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModel_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModel[] = L"Windows.AI.MachineLearning.LearningModel";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelBinding
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelBindingFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelBinding ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelBinding_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelBinding_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelBinding[] = L"Windows.AI.MachineLearning.LearningModelBinding";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelDevice
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelDeviceFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ILearningModelDeviceStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelDevice_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelDevice[] = L"Windows.AI.MachineLearning.LearningModelDevice";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelEvaluationResult
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelEvaluationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelEvaluationResult_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelEvaluationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelEvaluationResult[] = L"Windows.AI.MachineLearning.LearningModelEvaluationResult";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelSession
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelSessionFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelSessionFactory2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSession_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelSession[] = L"Windows.AI.MachineLearning.LearningModelSession";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelSessionOptions ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelSessionOptions2
 *    Windows.AI.MachineLearning.ILearningModelSessionOptions3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSessionOptions_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSessionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelSessionOptions[] = L"Windows.AI.MachineLearning.LearningModelSessionOptions";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.MachineLearning.MapFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.IMapFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_MapFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_MapFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_MapFeatureDescriptor[] = L"Windows.AI.MachineLearning.MapFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.SequenceFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ISequenceFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_SequenceFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_SequenceFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_SequenceFeatureDescriptor[] = L"Windows.AI.MachineLearning.SequenceFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorBoolean
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorBooleanStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorBooleanStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorBoolean ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorBoolean_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorBoolean_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorBoolean[] = L"Windows.AI.MachineLearning.TensorBoolean";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorDouble
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorDoubleStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorDoubleStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorDouble ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorDouble_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorDouble_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorDouble[] = L"Windows.AI.MachineLearning.TensorDouble";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorFeatureDescriptor[] = L"Windows.AI.MachineLearning.TensorFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorFloat
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloatStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloatStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorFloat ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorFloat[] = L"Windows.AI.MachineLearning.TensorFloat";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorFloat16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloat16BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloat16BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorFloat16Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat16Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat16Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorFloat16Bit[] = L"Windows.AI.MachineLearning.TensorFloat16Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt16BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt16BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt16Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt16Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt16Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt16Bit[] = L"Windows.AI.MachineLearning.TensorInt16Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt32BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt32BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt32Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt32Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt32Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt32Bit[] = L"Windows.AI.MachineLearning.TensorInt32Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt64BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt64BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt64Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt64Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt64Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt64Bit[] = L"Windows.AI.MachineLearning.TensorInt64Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt8BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt8BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt8Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt8Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt8Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt8Bit[] = L"Windows.AI.MachineLearning.TensorInt8Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorString
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorStringStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorStringStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorString ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorString_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorString_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorString[] = L"Windows.AI.MachineLearning.TensorString";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt16BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt16BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt16Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt16Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt16Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt16Bit[] = L"Windows.AI.MachineLearning.TensorUInt16Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt32BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt32BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt32Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt32Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt32Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt32Bit[] = L"Windows.AI.MachineLearning.TensorUInt32Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt64BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt64BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt64Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt64Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt64Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt64Bit[] = L"Windows.AI.MachineLearning.TensorUInt64Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt8BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt8BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt8Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt8Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt8Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt8Bit[] = L"Windows.AI.MachineLearning.TensorUInt8Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2 __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2 __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2 __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3 __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensor __x_ABI_CWindows_CAI_CMachineLearning_CITensor;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorString __x_ABI_CWindows_CAI_CMachineLearning_CITensorString;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2 __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel;

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel;

typedef struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelVtbl;

interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel* This,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModel_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult;

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult;

typedef struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResultVtbl;

interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* This,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_boolean_INTERFACE_DEFINED__)
#define ____FIIterator_1_boolean_INTERFACE_DEFINED__

typedef interface __FIIterator_1_boolean __FIIterator_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_boolean;

typedef struct __FIIterator_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_boolean* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_boolean* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_boolean* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_boolean* This,
        UINT32 itemsLength,
        boolean* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_booleanVtbl;

interface __FIIterator_1_boolean
{
    CONST_VTBL struct __FIIterator_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_boolean_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_boolean_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_boolean_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_boolean_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_boolean_INTERFACE_DEFINED__)
#define ____FIIterable_1_boolean_INTERFACE_DEFINED__

typedef interface __FIIterable_1_boolean __FIIterable_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_boolean;

typedef struct __FIIterable_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_boolean* This,
        __FIIterator_1_boolean** result);

    END_INTERFACE
} __FIIterable_1_booleanVtbl;

interface __FIIterable_1_boolean
{
    CONST_VTBL struct __FIIterable_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_boolean_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_double_INTERFACE_DEFINED__)
#define ____FIIterator_1_double_INTERFACE_DEFINED__

typedef interface __FIIterator_1_double __FIIterator_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_double;

typedef struct __FIIterator_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_double* This,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_double* This,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_doubleVtbl;

interface __FIIterator_1_double
{
    CONST_VTBL struct __FIIterator_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_double_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_double_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_double_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_double_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_double_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_double_INTERFACE_DEFINED__)
#define ____FIIterable_1_double_INTERFACE_DEFINED__

typedef interface __FIIterable_1_double __FIIterable_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_double;

typedef struct __FIIterable_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_double* This,
        __FIIterator_1_double** result);

    END_INTERFACE
} __FIIterable_1_doubleVtbl;

interface __FIIterable_1_double
{
    CONST_VTBL struct __FIIterable_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_double_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_double_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_short_INTERFACE_DEFINED__)
#define ____FIIterator_1_short_INTERFACE_DEFINED__

typedef interface __FIIterator_1_short __FIIterator_1_short;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_short;

typedef struct __FIIterator_1_shortVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_short* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_short* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_short* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_short* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_short* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_short* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_short* This,
        INT16* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_short* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_short* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_short* This,
        UINT32 itemsLength,
        INT16* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_shortVtbl;

interface __FIIterator_1_short
{
    CONST_VTBL struct __FIIterator_1_shortVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_short_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_short_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_short_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_short_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_short_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_short_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_short_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_short_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_short_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_short_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_short_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_short_INTERFACE_DEFINED__)
#define ____FIIterable_1_short_INTERFACE_DEFINED__

typedef interface __FIIterable_1_short __FIIterable_1_short;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_short;

typedef struct __FIIterable_1_shortVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_short* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_short* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_short* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_short* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_short* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_short* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_short* This,
        __FIIterator_1_short** result);

    END_INTERFACE
} __FIIterable_1_shortVtbl;

interface __FIIterable_1_short
{
    CONST_VTBL struct __FIIterable_1_shortVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_short_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_short_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_short_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_short_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_short_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_short_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_short_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_short_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_int_INTERFACE_DEFINED__)
#define ____FIIterator_1_int_INTERFACE_DEFINED__

typedef interface __FIIterator_1_int __FIIterator_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_int;

typedef struct __FIIterator_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_int* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_int* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_int* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_int* This,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_intVtbl;

interface __FIIterator_1_int
{
    CONST_VTBL struct __FIIterator_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_int_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_int_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_int_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_int_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_int_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_int_INTERFACE_DEFINED__)
#define ____FIIterable_1_int_INTERFACE_DEFINED__

typedef interface __FIIterable_1_int __FIIterable_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_int;

typedef struct __FIIterable_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_int* This,
        __FIIterator_1_int** result);

    END_INTERFACE
} __FIIterable_1_intVtbl;

interface __FIIterable_1_int
{
    CONST_VTBL struct __FIIterable_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_int_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_int_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___z__zint64_INTERFACE_DEFINED__)
#define ____FIIterator_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIIterator_1___z__zint64 __FIIterator_1___z__zint64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___z__zint64;

typedef struct __FIIterator_1___z__zint64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___z__zint64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___z__zint64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___z__zint64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___z__zint64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___z__zint64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___z__zint64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___z__zint64* This,
        INT64* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___z__zint64* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___z__zint64* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___z__zint64* This,
        UINT32 itemsLength,
        INT64* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___z__zint64Vtbl;

interface __FIIterator_1___z__zint64
{
    CONST_VTBL struct __FIIterator_1___z__zint64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___z__zint64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___z__zint64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___z__zint64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___z__zint64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___z__zint64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___z__zint64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___z__zint64_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___z__zint64_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___z__zint64_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___z__zint64_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___z__zint64_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___z__zint64_INTERFACE_DEFINED__)
#define ____FIIterable_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIIterable_1___z__zint64 __FIIterable_1___z__zint64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___z__zint64;

typedef struct __FIIterable_1___z__zint64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___z__zint64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___z__zint64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___z__zint64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___z__zint64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___z__zint64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___z__zint64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___z__zint64* This,
        __FIIterator_1___z__zint64** result);

    END_INTERFACE
} __FIIterable_1___z__zint64Vtbl;

interface __FIIterable_1___z__zint64
{
    CONST_VTBL struct __FIIterable_1___z__zint64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___z__zint64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___z__zint64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___z__zint64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___z__zint64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___z__zint64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___z__zint64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___z__zint64_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___z__zint64_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_float_INTERFACE_DEFINED__)
#define ____FIIterator_1_float_INTERFACE_DEFINED__

typedef interface __FIIterator_1_float __FIIterator_1_float;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_float;

typedef struct __FIIterator_1_floatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_float* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_float* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_float* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_float* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_float* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_float* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_float* This,
        FLOAT* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_float* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_float* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_float* This,
        UINT32 itemsLength,
        FLOAT* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_floatVtbl;

interface __FIIterator_1_float
{
    CONST_VTBL struct __FIIterator_1_floatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_float_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_float_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_float_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_float_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_float_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_float_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_float_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_float_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_float_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_float_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_float_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_float_INTERFACE_DEFINED__)
#define ____FIIterable_1_float_INTERFACE_DEFINED__

typedef interface __FIIterable_1_float __FIIterable_1_float;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_float;

typedef struct __FIIterable_1_floatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_float* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_float* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_float* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_float* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_float* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_float* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_float* This,
        __FIIterator_1_float** result);

    END_INTERFACE
} __FIIterable_1_floatVtbl;

interface __FIIterable_1_float
{
    CONST_VTBL struct __FIIterable_1_floatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_float_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_float_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_float_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_float_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_float_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_float_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_float_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_float_INTERFACE_DEFINED__

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

#if !defined(____FIIterator_1_UINT16_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT16_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT16 __FIIterator_1_UINT16;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT16;

typedef struct __FIIterator_1_UINT16Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT16* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT16* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT16* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT16* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT16* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT16* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT16* This,
        UINT16* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT16* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT16* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT16* This,
        UINT32 itemsLength,
        UINT16* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT16Vtbl;

interface __FIIterator_1_UINT16
{
    CONST_VTBL struct __FIIterator_1_UINT16Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT16_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT16_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT16_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT16_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT16_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT16_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT16_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT16_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT16_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT16_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT16_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT16_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT16_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT16 __FIIterable_1_UINT16;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT16;

typedef struct __FIIterable_1_UINT16Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT16* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT16* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT16* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT16* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT16* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT16* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT16* This,
        __FIIterator_1_UINT16** result);

    END_INTERFACE
} __FIIterable_1_UINT16Vtbl;

interface __FIIterable_1_UINT16
{
    CONST_VTBL struct __FIIterable_1_UINT16Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT16_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT16_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT16_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT16_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT16_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT16_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT16_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT16_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT32 __FIIterator_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT32;

typedef struct __FIIterator_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT32Vtbl;

interface __FIIterator_1_UINT32
{
    CONST_VTBL struct __FIIterator_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT32 __FIIterable_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT32;

typedef struct __FIIterable_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT32* This,
        __FIIterator_1_UINT32** result);

    END_INTERFACE
} __FIIterable_1_UINT32Vtbl;

interface __FIIterable_1_UINT32
{
    CONST_VTBL struct __FIIterable_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_UINT64_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT64 __FIIterator_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT64;

typedef struct __FIIterator_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT64* This,
        UINT64* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT64* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT64* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT64* This,
        UINT32 itemsLength,
        UINT64* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT64Vtbl;

interface __FIIterator_1_UINT64
{
    CONST_VTBL struct __FIIterator_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT64_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT64_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT64_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT64_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT64_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT64_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT64 __FIIterable_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT64;

typedef struct __FIIterable_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT64* This,
        __FIIterator_1_UINT64** result);

    END_INTERFACE
} __FIIterable_1_UINT64Vtbl;

interface __FIIterable_1_UINT64
{
    CONST_VTBL struct __FIIterable_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT64_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT64_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_byte_INTERFACE_DEFINED__)
#define ____FIIterator_1_byte_INTERFACE_DEFINED__

typedef interface __FIIterator_1_byte __FIIterator_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_byte;

typedef struct __FIIterator_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_byte* This,
        BYTE* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_byte* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_byte* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_byte* This,
        UINT32 itemsLength,
        BYTE* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_byteVtbl;

interface __FIIterator_1_byte
{
    CONST_VTBL struct __FIIterator_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_byte_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_byte_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_byte_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_byte_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_byte_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_byte_INTERFACE_DEFINED__)
#define ____FIIterable_1_byte_INTERFACE_DEFINED__

typedef interface __FIIterable_1_byte __FIIterable_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_byte;

typedef struct __FIIterable_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_byte* This,
        __FIIterator_1_byte** result);

    END_INTERFACE
} __FIIterable_1_byteVtbl;

interface __FIIterable_1_byte
{
    CONST_VTBL struct __FIIterable_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_byte_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_byte_INTERFACE_DEFINED__

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor;

typedef struct __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl;

interface __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor;

typedef struct __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        __FIIterator_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl;

interface __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

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

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

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

#if !defined(____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_IInspectable __FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_IInspectable;

typedef struct __FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_IInspectable* This);

    END_INTERFACE
} __FIMap_2_HSTRING_IInspectableVtbl;

interface __FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_boolean_INTERFACE_DEFINED__)
#define ____FIVectorView_1_boolean_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_boolean __FIVectorView_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_boolean;

typedef struct __FIVectorView_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_boolean* This,
        UINT32 index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_boolean* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_boolean* This,
        boolean value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_boolean* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        boolean* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_booleanVtbl;

interface __FIVectorView_1_boolean
{
    CONST_VTBL struct __FIVectorView_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_boolean_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_boolean_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_boolean_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_boolean_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_double_INTERFACE_DEFINED__)
#define ____FIVectorView_1_double_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_double __FIVectorView_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_double;

typedef struct __FIVectorView_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_double* This,
        UINT32 index,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_double* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_double* This,
        DOUBLE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_double* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_doubleVtbl;

interface __FIVectorView_1_double
{
    CONST_VTBL struct __FIVectorView_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_double_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_double_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_double_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_double_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_double_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_short_INTERFACE_DEFINED__)
#define ____FIVectorView_1_short_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_short __FIVectorView_1_short;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_short;

typedef struct __FIVectorView_1_shortVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_short* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_short* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_short* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_short* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_short* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_short* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_short* This,
        UINT32 index,
        INT16* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_short* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_short* This,
        INT16 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_short* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        INT16* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_shortVtbl;

interface __FIVectorView_1_short
{
    CONST_VTBL struct __FIVectorView_1_shortVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_short_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_short_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_short_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_short_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_short_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_short_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_short_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_short_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_short_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_short_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_short_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_int_INTERFACE_DEFINED__)
#define ____FIVectorView_1_int_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_int __FIVectorView_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_int;

typedef struct __FIVectorView_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_int* This,
        UINT32 index,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_int* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_int* This,
        INT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_int* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_intVtbl;

interface __FIVectorView_1_int
{
    CONST_VTBL struct __FIVectorView_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_int_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_int_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_int_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_int_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_int_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1___z__zint64_INTERFACE_DEFINED__)
#define ____FIVectorView_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIVectorView_1___z__zint64 __FIVectorView_1___z__zint64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1___z__zint64;

typedef struct __FIVectorView_1___z__zint64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1___z__zint64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1___z__zint64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1___z__zint64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1___z__zint64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1___z__zint64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1___z__zint64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1___z__zint64* This,
        UINT32 index,
        INT64* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1___z__zint64* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1___z__zint64* This,
        INT64 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1___z__zint64* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        INT64* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1___z__zint64Vtbl;

interface __FIVectorView_1___z__zint64
{
    CONST_VTBL struct __FIVectorView_1___z__zint64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1___z__zint64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1___z__zint64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1___z__zint64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1___z__zint64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1___z__zint64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1___z__zint64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1___z__zint64_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1___z__zint64_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1___z__zint64_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1___z__zint64_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1___z__zint64_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_float_INTERFACE_DEFINED__)
#define ____FIVectorView_1_float_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_float __FIVectorView_1_float;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_float;

typedef struct __FIVectorView_1_floatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_float* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_float* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_float* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_float* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_float* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_float* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_float* This,
        UINT32 index,
        FLOAT* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_float* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_float* This,
        FLOAT value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_float* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        FLOAT* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_floatVtbl;

interface __FIVectorView_1_float
{
    CONST_VTBL struct __FIVectorView_1_floatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_float_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_float_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_float_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_float_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_float_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_float_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_float_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_float_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_float_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_float_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_float_INTERFACE_DEFINED__

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

#if !defined(____FIVectorView_1_UINT16_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT16_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT16 __FIVectorView_1_UINT16;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT16;

typedef struct __FIVectorView_1_UINT16Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT16* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT16* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT16* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT16* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT16* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT16* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT16* This,
        UINT32 index,
        UINT16* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT16* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT16* This,
        UINT16 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT16* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT16* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT16Vtbl;

interface __FIVectorView_1_UINT16
{
    CONST_VTBL struct __FIVectorView_1_UINT16Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT16_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT16_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT16_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT16_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT16_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT16_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT16_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT16_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT16_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT16_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT16_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT32 __FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT32;

typedef struct __FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT32Vtbl;

interface __FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_UINT64_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT64 __FIVectorView_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT64;

typedef struct __FIVectorView_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT64* This,
        UINT32 index,
        UINT64* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT64* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT64* This,
        UINT64 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT64* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT64* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT64Vtbl;

interface __FIVectorView_1_UINT64
{
    CONST_VTBL struct __FIVectorView_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT64_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT64_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT64_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT64_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT64_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_byte_INTERFACE_DEFINED__)
#define ____FIVectorView_1_byte_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_byte __FIVectorView_1_byte;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_byte;

typedef struct __FIVectorView_1_byteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_byte* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_byte* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_byte* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_byte* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_byte* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_byte* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_byte* This,
        UINT32 index,
        BYTE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_byte* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_byte* This,
        BYTE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_byte* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        BYTE* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_byteVtbl;

interface __FIVectorView_1_byte
{
    CONST_VTBL struct __FIVectorView_1_byteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_byte_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_byte_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_byte_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_byte_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_byte_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_byte_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_byte_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_byte_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_byte_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_byte_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_byte_INTERFACE_DEFINED__

#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor;

typedef struct __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl;

interface __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId __x_ABI_CWindows_CGraphics_CDisplayAdapterId;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame __x_ABI_CWindows_CMedia_CIVideoFrame;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelDeviceKind __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelDeviceKind;

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelFeatureKind __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelFeatureKind;

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelPixelRange __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelPixelRange;

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CTensorKind __x_ABI_CWindows_CAI_CMachineLearning_CTensorKind;

/*
 *
 * Struct Windows.AI.MachineLearning.LearningModelDeviceKind
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelDeviceKind
{
    LearningModelDeviceKind_Default = 0,
    LearningModelDeviceKind_Cpu = 1,
    LearningModelDeviceKind_DirectX = 2,
    LearningModelDeviceKind_DirectXHighPerformance = 3,
    LearningModelDeviceKind_DirectXMinPower = 4,
};
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.LearningModelFeatureKind
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelFeatureKind
{
    LearningModelFeatureKind_Tensor = 0,
    LearningModelFeatureKind_Sequence = 1,
    LearningModelFeatureKind_Map = 2,
    LearningModelFeatureKind_Image = 3,
};
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.LearningModelPixelRange
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 5.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelPixelRange
{
    LearningModelPixelRange_ZeroTo255 = 0,
    LearningModelPixelRange_ZeroToOne = 1,
    LearningModelPixelRange_MinusOneToOne = 2,
};
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.AI.MachineLearning.TensorKind
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CAI_CMachineLearning_CTensorKind
{
    TensorKind_Undefined = 0,
    TensorKind_Float = 1,
    TensorKind_UInt8 = 2,
    TensorKind_Int8 = 3,
    TensorKind_UInt16 = 4,
    TensorKind_Int16 = 5,
    TensorKind_Int32 = 6,
    TensorKind_Int64 = 7,
    TensorKind_String = 8,
    TensorKind_Boolean = 9,
    TensorKind_Float16 = 10,
    TensorKind_Double = 11,
    TensorKind_UInt32 = 12,
    TensorKind_UInt64 = 13,
    TensorKind_Complex64 = 14,
    TensorKind_Complex128 = 15,
};
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureDescriptor[] = L"Windows.AI.MachineLearning.IImageFeatureDescriptor";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BitmapPixelFormat)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapAlphaMode)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode* value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptorVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_get_BitmapPixelFormat(This, value) \
    ((This)->lpVtbl->get_BitmapPixelFormat(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_get_BitmapAlphaMode(This, value) \
    ((This)->lpVtbl->get_BitmapAlphaMode(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureDescriptor2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureDescriptor2[] = L"Windows.AI.MachineLearning.IImageFeatureDescriptor2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelRange)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelPixelRange* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_get_PixelRange(This, value) \
    ((This)->lpVtbl->get_PixelRange(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureDescriptor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureValue
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureValue
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureValue[] = L"Windows.AI.MachineLearning.IImageFeatureValue";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VideoFrame)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_get_VideoFrame(This, value) \
    ((This)->lpVtbl->get_VideoFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IImageFeatureValueStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.ImageFeatureValue
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IImageFeatureValueStatics[] = L"Windows.AI.MachineLearning.IImageFeatureValueStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromVideoFrame)(__x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame* image,
        __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValue** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_CreateFromVideoFrame(This, image, result) \
    ((This)->lpVtbl->CreateFromVideoFrame(This, image, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIImageFeatureValueStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModel
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModel
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModel[] = L"Windows.AI.MachineLearning.ILearningModel";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Domain)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Metadata)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        __FIMapView_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_InputFeatures)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputFeatures)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* This,
        __FIVectorView_1_Windows__CAI__CMachineLearning__CILearningModelFeatureDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_Domain(This, value) \
    ((This)->lpVtbl->get_Domain(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_Metadata(This, value) \
    ((This)->lpVtbl->get_Metadata(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_InputFeatures(This, value) \
    ((This)->lpVtbl->get_InputFeatures(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_get_OutputFeatures(This, value) \
    ((This)->lpVtbl->get_OutputFeatures(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModel;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelBinding
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelBinding
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelBinding[] = L"Windows.AI.MachineLearning.ILearningModelBinding";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Bind)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This,
        HSTRING name,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* BindWithProperties)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This,
        HSTRING name,
        IInspectable* value,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* props);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* This);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_Bind(This, name, value) \
    ((This)->lpVtbl->Bind(This, name, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_BindWithProperties(This, name, value, props) \
    ((This)->lpVtbl->BindWithProperties(This, name, value, props))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelBindingFactory
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelBinding
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelBindingFactory[] = L"Windows.AI.MachineLearning.ILearningModelBindingFactory";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromSession)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* session,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactoryVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_CreateFromSession(This, session, value) \
    ((This)->lpVtbl->CreateFromSession(This, session, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBindingFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelDevice
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelDevice
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelDevice[] = L"Windows.AI.MachineLearning.ILearningModelDevice";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdapterId)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This,
        struct __x_ABI_CWindows_CGraphics_CDisplayAdapterId* value);
    HRESULT (STDMETHODCALLTYPE* get_Direct3D11Device)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_get_AdapterId(This, value) \
    ((This)->lpVtbl->get_AdapterId(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_get_Direct3D11Device(This, value) \
    ((This)->lpVtbl->get_Direct3D11Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelDeviceFactory
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelDevice
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelDeviceFactory[] = L"Windows.AI.MachineLearning.ILearningModelDeviceFactory";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelDeviceKind deviceKind,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactoryVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_Create(This, deviceKind, value) \
    ((This)->lpVtbl->Create(This, deviceKind, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelDeviceStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelDevice
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelDeviceStatics[] = L"Windows.AI.MachineLearning.ILearningModelDeviceStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromDirect3D11Device)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_CreateFromDirect3D11Device(This, device, result) \
    ((This)->lpVtbl->CreateFromDirect3D11Device(This, device, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelEvaluationResult
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelEvaluationResult
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelEvaluationResult[] = L"Windows.AI.MachineLearning.ILearningModelEvaluationResult";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CorrelationId)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorStatus)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Outputs)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResultVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_get_CorrelationId(This, value) \
    ((This)->lpVtbl->get_CorrelationId(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_get_ErrorStatus(This, value) \
    ((This)->lpVtbl->get_ErrorStatus(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_get_Outputs(This, value) \
    ((This)->lpVtbl->get_Outputs(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelFeatureDescriptor[] = L"Windows.AI.MachineLearning.ILearningModelFeatureDescriptor";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelFeatureKind* value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequired)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptorVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_get_IsRequired(This, value) \
    ((This)->lpVtbl->get_IsRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelFeatureValue
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelFeatureValue[] = L"Windows.AI.MachineLearning.ILearningModelFeatureValue";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CLearningModelFeatureKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValueVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelOperatorProvider
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelOperatorProvider[] = L"Windows.AI.MachineLearning.ILearningModelOperatorProvider";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProviderVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSession
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSession
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSession[] = L"Windows.AI.MachineLearning.ILearningModelSession";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Model)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel** value);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice** value);
    HRESULT (STDMETHODCALLTYPE* get_EvaluationProperties)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* EvaluateAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* bindings,
        HSTRING correlationId,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult** operation);
    HRESULT (STDMETHODCALLTYPE* EvaluateFeaturesAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __FIMap_2_HSTRING_IInspectable* features,
        HSTRING correlationId,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModelEvaluationResult** operation);
    HRESULT (STDMETHODCALLTYPE* Evaluate)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelBinding* bindings,
        HSTRING correlationId,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult** result);
    HRESULT (STDMETHODCALLTYPE* EvaluateFeatures)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession* This,
        __FIMap_2_HSTRING_IInspectable* features,
        HSTRING correlationId,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelEvaluationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_get_Model(This, value) \
    ((This)->lpVtbl->get_Model(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_get_EvaluationProperties(This, value) \
    ((This)->lpVtbl->get_EvaluationProperties(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_EvaluateAsync(This, bindings, correlationId, operation) \
    ((This)->lpVtbl->EvaluateAsync(This, bindings, correlationId, operation))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_EvaluateFeaturesAsync(This, features, correlationId, operation) \
    ((This)->lpVtbl->EvaluateFeaturesAsync(This, features, correlationId, operation))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_Evaluate(This, bindings, correlationId, result) \
    ((This)->lpVtbl->Evaluate(This, bindings, correlationId, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_EvaluateFeatures(This, features, correlationId, result) \
    ((This)->lpVtbl->EvaluateFeatures(This, features, correlationId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionFactory
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSession
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionFactory[] = L"Windows.AI.MachineLearning.ILearningModelSessionFactory";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromModel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* model,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromModelOnDevice)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* model,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* deviceToRunOn,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactoryVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_CreateFromModel(This, model, value) \
    ((This)->lpVtbl->CreateFromModel(This, model, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_CreateFromModelOnDevice(This, model, deviceToRunOn, value) \
    ((This)->lpVtbl->CreateFromModelOnDevice(This, model, deviceToRunOn, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionFactory2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSession
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionFactory2[] = L"Windows.AI.MachineLearning.ILearningModelSessionFactory2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromModelOnDeviceWithSessionOptions)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel* model,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelDevice* deviceToRunOn,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* learningModelSessionOptions,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSession** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_CreateFromModelOnDeviceWithSessionOptions(This, model, deviceToRunOn, learningModelSessionOptions, value) \
    ((This)->lpVtbl->CreateFromModelOnDeviceWithSessionOptions(This, model, deviceToRunOn, learningModelSessionOptions, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionOptions
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionOptions[] = L"Windows.AI.MachineLearning.ILearningModelSessionOptions";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BatchSizeOverride)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_BatchSizeOverride)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptionsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_get_BatchSizeOverride(This, value) \
    ((This)->lpVtbl->get_BatchSizeOverride(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_put_BatchSizeOverride(This, value) \
    ((This)->lpVtbl->put_BatchSizeOverride(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionOptions2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionOptions2[] = L"Windows.AI.MachineLearning.ILearningModelSessionOptions2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CloseModelOnSessionCreation)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CloseModelOnSessionCreation)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_get_CloseModelOnSessionCreation(This, value) \
    ((This)->lpVtbl->get_CloseModelOnSessionCreation(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_put_CloseModelOnSessionCreation(This, value) \
    ((This)->lpVtbl->put_CloseModelOnSessionCreation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelSessionOptions3
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelSessionOptions3[] = L"Windows.AI.MachineLearning.ILearningModelSessionOptions3";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OverrideNamedDimension)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3* This,
        HSTRING name,
        UINT32 dimension);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_OverrideNamedDimension(This, name, dimension) \
    ((This)->lpVtbl->OverrideNamedDimension(This, name, dimension))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelSessionOptions3_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.AI.MachineLearning.ILearningModelStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.LearningModel
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ILearningModelStatics[] = L"Windows.AI.MachineLearning.ILearningModelStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadFromStorageFileAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* modelFile,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation);
    HRESULT (STDMETHODCALLTYPE* LoadFromStreamAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* modelStream,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation);
    HRESULT (STDMETHODCALLTYPE* LoadFromFilePath)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        HSTRING filePath,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel** result);
    HRESULT (STDMETHODCALLTYPE* LoadFromStream)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* modelStream,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel** result);
    HRESULT (STDMETHODCALLTYPE* LoadFromStorageFileWithOperatorProviderAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* modelFile,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* operatorProvider,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation);
    HRESULT (STDMETHODCALLTYPE* LoadFromStreamWithOperatorProviderAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* modelStream,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* operatorProvider,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CLearningModel** operation);
    HRESULT (STDMETHODCALLTYPE* LoadFromFilePathWithOperatorProvider)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        HSTRING filePath,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* operatorProvider,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel** result);
    HRESULT (STDMETHODCALLTYPE* LoadFromStreamWithOperatorProvider)(__x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* modelStream,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelOperatorProvider* operatorProvider,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModel** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromStorageFileAsync(This, modelFile, operation) \
    ((This)->lpVtbl->LoadFromStorageFileAsync(This, modelFile, operation))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromStreamAsync(This, modelStream, operation) \
    ((This)->lpVtbl->LoadFromStreamAsync(This, modelStream, operation))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromFilePath(This, filePath, result) \
    ((This)->lpVtbl->LoadFromFilePath(This, filePath, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromStream(This, modelStream, result) \
    ((This)->lpVtbl->LoadFromStream(This, modelStream, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromStorageFileWithOperatorProviderAsync(This, modelFile, operatorProvider, operation) \
    ((This)->lpVtbl->LoadFromStorageFileWithOperatorProviderAsync(This, modelFile, operatorProvider, operation))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromStreamWithOperatorProviderAsync(This, modelStream, operatorProvider, operation) \
    ((This)->lpVtbl->LoadFromStreamWithOperatorProviderAsync(This, modelStream, operatorProvider, operation))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromFilePathWithOperatorProvider(This, filePath, operatorProvider, result) \
    ((This)->lpVtbl->LoadFromFilePathWithOperatorProvider(This, filePath, operatorProvider, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_LoadFromStreamWithOperatorProvider(This, modelStream, operatorProvider, result) \
    ((This)->lpVtbl->LoadFromStreamWithOperatorProvider(This, modelStream, operatorProvider, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CILearningModelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.IMapFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.MapFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_IMapFeatureDescriptor[] = L"Windows.AI.MachineLearning.IMapFeatureDescriptor";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeyKind)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CTensorKind* value);
    HRESULT (STDMETHODCALLTYPE* get_ValueDescriptor)(__x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptorVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_get_KeyKind(This, value) \
    ((This)->lpVtbl->get_KeyKind(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_get_ValueDescriptor(This, value) \
    ((This)->lpVtbl->get_ValueDescriptor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CIMapFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ISequenceFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.SequenceFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ISequenceFeatureDescriptor[] = L"Windows.AI.MachineLearning.ISequenceFeatureDescriptor";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ElementDescriptor)(__x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CILearningModelFeatureDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptorVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_get_ElementDescriptor(This, value) \
    ((This)->lpVtbl->get_ElementDescriptor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CISequenceFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.ILearningModelFeatureValue
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensor[] = L"Windows.AI.MachineLearning.ITensor";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TensorKind)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CTensorKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Shape)(__x_ABI_CWindows_CAI_CMachineLearning_CITensor* This,
        __FIVectorView_1___z__zint64** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensor
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_get_TensorKind(This, value) \
    ((This)->lpVtbl->get_TensorKind(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensor_get_Shape(This, value) \
    ((This)->lpVtbl->get_Shape(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorBoolean
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorBoolean
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorBoolean[] = L"Windows.AI.MachineLearning.ITensorBoolean";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean* This,
        __FIVectorView_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorBooleanStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorBoolean
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorBooleanStatics[] = L"Windows.AI.MachineLearning.ITensorBooleanStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        boolean* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_boolean* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorBooleanStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorBoolean
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorBooleanStatics2[] = L"Windows.AI.MachineLearning.ITensorBooleanStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        boolean* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorBoolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorBooleanStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorDouble
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorDouble
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorDouble[] = L"Windows.AI.MachineLearning.ITensorDouble";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble* This,
        __FIVectorView_1_double** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorDoubleStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorDouble
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorDoubleStatics[] = L"Windows.AI.MachineLearning.ITensorDoubleStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        DOUBLE* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_double* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorDoubleStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorDouble
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorDoubleStatics2[] = L"Windows.AI.MachineLearning.ITensorDoubleStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        DOUBLE* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorDouble** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorDoubleStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFeatureDescriptor
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFeatureDescriptor[] = L"Windows.AI.MachineLearning.ITensorFeatureDescriptor";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TensorKind)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CTensorKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Shape)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor* This,
        __FIVectorView_1___z__zint64** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptorVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_get_TensorKind(This, value) \
    ((This)->lpVtbl->get_TensorKind(This, value))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_get_Shape(This, value) \
    ((This)->lpVtbl->get_Shape(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFeatureDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat[] = L"Windows.AI.MachineLearning.ITensorFloat";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat* This,
        __FIVectorView_1_float** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat16Bit[] = L"Windows.AI.MachineLearning.ITensorFloat16Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit* This,
        __FIVectorView_1_float** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat16BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat16BitStatics[] = L"Windows.AI.MachineLearning.ITensorFloat16BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        FLOAT* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_float* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloat16BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloat16BitStatics2[] = L"Windows.AI.MachineLearning.ITensorFloat16BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        FLOAT* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat16BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloatStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloatStatics[] = L"Windows.AI.MachineLearning.ITensorFloatStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        FLOAT* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_float* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorFloatStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorFloat
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorFloatStatics2[] = L"Windows.AI.MachineLearning.ITensorFloatStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        FLOAT* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloat** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorFloatStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt16Bit[] = L"Windows.AI.MachineLearning.ITensorInt16Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit* This,
        __FIVectorView_1_short** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt16BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt16BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt16BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        INT16* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_short* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt16BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt16BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt16BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        INT16* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt16BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt32Bit[] = L"Windows.AI.MachineLearning.ITensorInt32Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit* This,
        __FIVectorView_1_int** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt32BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt32BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt32BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        INT32* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_int* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt32BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt32BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt32BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        INT32* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt32BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt64Bit[] = L"Windows.AI.MachineLearning.ITensorInt64Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit* This,
        __FIVectorView_1___z__zint64** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt64BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt64BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt64BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        INT64* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1___z__zint64* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt64BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt64BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt64BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        INT64* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt64BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt8Bit[] = L"Windows.AI.MachineLearning.ITensorInt8Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit* This,
        __FIVectorView_1_byte** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt8BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt8BitStatics[] = L"Windows.AI.MachineLearning.ITensorInt8BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        BYTE* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_byte* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorInt8BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorInt8BitStatics2[] = L"Windows.AI.MachineLearning.ITensorInt8BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        BYTE* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorInt8BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorString
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorString
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorString[] = L"Windows.AI.MachineLearning.ITensorString";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorString* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorString
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorString_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorString;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorString_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorStringStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorString
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorStringStatics[] = L"Windows.AI.MachineLearning.ITensorStringStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorString** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorString** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        HSTRING* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorString** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_HSTRING* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorString** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorStringStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorString
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorStringStatics2[] = L"Windows.AI.MachineLearning.ITensorStringStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        HSTRING* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorString** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorStringStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt16Bit[] = L"Windows.AI.MachineLearning.ITensorUInt16Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit* This,
        __FIVectorView_1_UINT16** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt16BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt16BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt16BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        UINT16* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_UINT16* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt16BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt16Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt16BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt16BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        UINT16* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt16BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt32Bit[] = L"Windows.AI.MachineLearning.ITensorUInt32Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit* This,
        __FIVectorView_1_UINT32** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt32BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt32BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt32BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        UINT32* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_UINT32* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt32BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt32Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt32BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt32BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        UINT32* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt32BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt64Bit[] = L"Windows.AI.MachineLearning.ITensorUInt64Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit* This,
        __FIVectorView_1_UINT64** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt64BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt64BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt64BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        UINT64* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_UINT64* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt64BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt64Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt64BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt64BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        UINT64* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt64BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt8Bit[] = L"Windows.AI.MachineLearning.ITensorUInt8Bit";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsVectorView)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit* This,
        __FIVectorView_1_byte** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_GetAsVectorView(This, result) \
    ((This)->lpVtbl->GetAsVectorView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt8BitStatics
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt8BitStatics[] = L"Windows.AI.MachineLearning.ITensorUInt8BitStatics";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* Create2)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        UINT32 dataLength,
        BYTE* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromIterable)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics* This,
        __FIIterable_1___z__zint64* shape,
        __FIIterable_1_byte* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_Create(This, result) \
    ((This)->lpVtbl->Create(This, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_Create2(This, shape, result) \
    ((This)->lpVtbl->Create2(This, shape, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_CreateFromArray(This, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromArray(This, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_CreateFromIterable(This, shape, data, result) \
    ((This)->lpVtbl->CreateFromIterable(This, shape, data, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.ITensorUInt8BitStatics2
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.TensorUInt8Bit
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_ITensorUInt8BitStatics2[] = L"Windows.AI.MachineLearning.ITensorUInt8BitStatics2";
typedef struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromShapeArrayAndDataArray)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        UINT32 dataLength,
        BYTE* data,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2* This,
        UINT32 shapeLength,
        INT64* shape,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8Bit** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2Vtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result) \
    ((This)->lpVtbl->CreateFromShapeArrayAndDataArray(This, shapeLength, shape, dataLength, data, result))

#define __x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_CreateFromBuffer(This, shapeLength, shape, buffer, result) \
    ((This)->lpVtbl->CreateFromBuffer(This, shapeLength, shape, buffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CITensorUInt8BitStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.MachineLearning.ImageFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.IImageFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.IImageFeatureDescriptor2
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_ImageFeatureDescriptor[] = L"Windows.AI.MachineLearning.ImageFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.ImageFeatureValue
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.IImageFeatureValueStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.IImageFeatureValue ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureValue_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_ImageFeatureValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_ImageFeatureValue[] = L"Windows.AI.MachineLearning.ImageFeatureValue";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModel
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ILearningModelStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModel ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModel_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModel[] = L"Windows.AI.MachineLearning.LearningModel";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelBinding
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelBindingFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelBinding ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelBinding_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelBinding_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelBinding[] = L"Windows.AI.MachineLearning.LearningModelBinding";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelDevice
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelDeviceFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ILearningModelDeviceStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelDevice_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelDevice[] = L"Windows.AI.MachineLearning.LearningModelDevice";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelEvaluationResult
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelEvaluationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelEvaluationResult_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelEvaluationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelEvaluationResult[] = L"Windows.AI.MachineLearning.LearningModelEvaluationResult";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelSession
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelSessionFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Type can be activated via the Windows.AI.MachineLearning.ILearningModelSessionFactory2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSession_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelSession[] = L"Windows.AI.MachineLearning.LearningModelSession";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.LearningModelSessionOptions
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ILearningModelSessionOptions ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelSessionOptions2
 *    Windows.AI.MachineLearning.ILearningModelSessionOptions3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSessionOptions_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_LearningModelSessionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_LearningModelSessionOptions[] = L"Windows.AI.MachineLearning.LearningModelSessionOptions";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.MachineLearning.MapFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.IMapFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_MapFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_MapFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_MapFeatureDescriptor[] = L"Windows.AI.MachineLearning.MapFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.SequenceFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ISequenceFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_SequenceFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_SequenceFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_SequenceFeatureDescriptor[] = L"Windows.AI.MachineLearning.SequenceFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorBoolean
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorBooleanStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorBooleanStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorBoolean ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorBoolean_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorBoolean_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorBoolean[] = L"Windows.AI.MachineLearning.TensorBoolean";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorDouble
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorDoubleStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorDoubleStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorDouble ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorDouble_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorDouble_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorDouble[] = L"Windows.AI.MachineLearning.TensorDouble";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorFeatureDescriptor
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorFeatureDescriptor ** Default Interface **
 *    Windows.AI.MachineLearning.ILearningModelFeatureDescriptor
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorFeatureDescriptor_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorFeatureDescriptor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorFeatureDescriptor[] = L"Windows.AI.MachineLearning.TensorFeatureDescriptor";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorFloat
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloatStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloatStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorFloat ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorFloat[] = L"Windows.AI.MachineLearning.TensorFloat";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorFloat16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloat16BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorFloat16BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorFloat16Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat16Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorFloat16Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorFloat16Bit[] = L"Windows.AI.MachineLearning.TensorFloat16Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt16BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt16BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt16Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt16Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt16Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt16Bit[] = L"Windows.AI.MachineLearning.TensorInt16Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt32BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt32BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt32Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt32Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt32Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt32Bit[] = L"Windows.AI.MachineLearning.TensorInt32Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt64BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt64BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt64Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt64Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt64Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt64Bit[] = L"Windows.AI.MachineLearning.TensorInt64Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt8BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorInt8BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorInt8Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt8Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorInt8Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorInt8Bit[] = L"Windows.AI.MachineLearning.TensorInt8Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorString
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorStringStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorStringStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorString ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorString_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorString_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorString[] = L"Windows.AI.MachineLearning.TensorString";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt16Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt16BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt16BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt16Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt16Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt16Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt16Bit[] = L"Windows.AI.MachineLearning.TensorUInt16Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt32Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt32BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt32BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt32Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt32Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt32Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt32Bit[] = L"Windows.AI.MachineLearning.TensorUInt32Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt64Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt64BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt64BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt64Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt64Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt64Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt64Bit[] = L"Windows.AI.MachineLearning.TensorUInt64Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.TensorUInt8Bit
 *
 * Introduced to Windows.AI.MachineLearning.MachineLearningContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt8BitStatics2 interface starting with version 2.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *   Static Methods exist on the Windows.AI.MachineLearning.ITensorUInt8BitStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.MachineLearningContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.ITensorUInt8Bit ** Default Interface **
 *    Windows.AI.MachineLearning.ITensor
 *    Windows.AI.MachineLearning.ILearningModelFeatureValue
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt8Bit_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_TensorUInt8Bit_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_TensorUInt8Bit[] = L"Windows.AI.MachineLearning.TensorUInt8Bit";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_MACHINELEARNINGCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eai2Emachinelearning_p_h__

#endif // __windows2Eai2Emachinelearning_h__
