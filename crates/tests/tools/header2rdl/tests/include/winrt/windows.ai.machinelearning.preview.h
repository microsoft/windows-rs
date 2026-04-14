
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
#ifndef __windows2Eai2Emachinelearning2Epreview_h__
#define __windows2Eai2Emachinelearning2Epreview_h__
#ifndef __windows2Eai2Emachinelearning2Epreview_p_h__
#define __windows2Eai2Emachinelearning2Epreview_p_h__


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
#if !defined(WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION)
#define WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION)

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
#include "Windows.Graphics.Imaging.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface IImageVariableDescriptorPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview ABI::Windows::AI::MachineLearning::Preview::IImageVariableDescriptorPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface IInferencingOptionsPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview ABI::Windows::AI::MachineLearning::Preview::IInferencingOptionsPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelBindingPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview ABI::Windows::AI::MachineLearning::Preview::ILearningModelBindingPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelBindingPreviewFactory;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory ABI::Windows::AI::MachineLearning::Preview::ILearningModelBindingPreviewFactory

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelDescriptionPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview ABI::Windows::AI::MachineLearning::Preview::ILearningModelDescriptionPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelEvaluationResultPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview ABI::Windows::AI::MachineLearning::Preview::ILearningModelEvaluationResultPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview ABI::Windows::AI::MachineLearning::Preview::ILearningModelPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelPreviewStatics;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics ABI::Windows::AI::MachineLearning::Preview::ILearningModelPreviewStatics

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ILearningModelVariableDescriptorPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface IMapVariableDescriptorPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview ABI::Windows::AI::MachineLearning::Preview::IMapVariableDescriptorPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ISequenceVariableDescriptorPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview ABI::Windows::AI::MachineLearning::Preview::ISequenceVariableDescriptorPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    interface ITensorVariableDescriptorPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview ABI::Windows::AI::MachineLearning::Preview::ITensorVariableDescriptorPreview

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    class LearningModelEvaluationResultPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_USE
#define DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c643f2b8-ea38-5230-9348-1094c06d917d"))
IAsyncOperation<ABI::Windows::AI::MachineLearning::Preview::LearningModelEvaluationResultPreview*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::Preview::LearningModelEvaluationResultPreview*, ABI::Windows::AI::MachineLearning::Preview::ILearningModelEvaluationResultPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::AI::MachineLearning::Preview::LearningModelEvaluationResultPreview*> __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_t;
#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_USE */

#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f21682d4-4fcb-5ad7-9b4e-1fe9c5942bfb"))
IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::Preview::LearningModelEvaluationResultPreview*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::Preview::LearningModelEvaluationResultPreview*, ABI::Windows::AI::MachineLearning::Preview::ILearningModelEvaluationResultPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::Preview::LearningModelEvaluationResultPreview*> __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_USE */

#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    class LearningModelPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_USE
#define DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("86cdc6bd-809d-5a2b-898b-5c2a92be7744"))
IAsyncOperation<ABI::Windows::AI::MachineLearning::Preview::LearningModelPreview*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::Preview::LearningModelPreview*, ABI::Windows::AI::MachineLearning::Preview::ILearningModelPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.AI.MachineLearning.Preview.LearningModelPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::AI::MachineLearning::Preview::LearningModelPreview*> __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_t;
#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_USE */

#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a76c9ad9-6f09-5d01-8c1f-516623cbfa85"))
IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::Preview::LearningModelPreview*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::MachineLearning::Preview::LearningModelPreview*, ABI::Windows::AI::MachineLearning::Preview::ILearningModelPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.AI.MachineLearning.Preview.LearningModelPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::AI::MachineLearning::Preview::LearningModelPreview*> __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_USE */

#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000


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


#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_USE
#define DEF___FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9696f7b-99cf-57ea-99ca-63e1ff9f4919"))
IIterator<ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview*> : IIterator_impl<ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview*> __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_t;
#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_USE */

#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_USE
#define DEF___FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("df23db35-f789-51a1-856d-87cd7cd042f1"))
IIterable<ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview*> : IIterable_impl<ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview*> __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_t;
#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_USE */

#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000


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
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapPixelFormat : int BitmapPixelFormat;
            } /* Imaging */
        } /* Graphics */
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
                namespace Preview {
                    typedef enum FeatureElementKindPreview : int FeatureElementKindPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    typedef enum LearningModelDeviceKindPreview : int LearningModelDeviceKindPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    typedef enum LearningModelFeatureKindPreview : int LearningModelFeatureKindPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    class InferencingOptionsPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    class LearningModelBindingPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    class LearningModelDescriptionPreview;
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.AI.MachineLearning.Preview.FeatureElementKindPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    enum
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use TensorKind instead of FeatureElementKindPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    FeatureElementKindPreview : int
                    {
                        FeatureElementKindPreview_Undefined = 0,
                        FeatureElementKindPreview_Float = 1,
                        FeatureElementKindPreview_UInt8 = 2,
                        FeatureElementKindPreview_Int8 = 3,
                        FeatureElementKindPreview_UInt16 = 4,
                        FeatureElementKindPreview_Int16 = 5,
                        FeatureElementKindPreview_Int32 = 6,
                        FeatureElementKindPreview_Int64 = 7,
                        FeatureElementKindPreview_String = 8,
                        FeatureElementKindPreview_Boolean = 9,
                        FeatureElementKindPreview_Float16 = 10,
                        FeatureElementKindPreview_Double = 11,
                        FeatureElementKindPreview_UInt32 = 12,
                        FeatureElementKindPreview_UInt64 = 13,
                        FeatureElementKindPreview_Complex64 = 14,
                        FeatureElementKindPreview_Complex128 = 15,
                    };
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.Preview.LearningModelDeviceKindPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    enum
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use LearningModelDeviceKind instead of LearningModelDeviceKindPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    LearningModelDeviceKindPreview : int
                    {
                        LearningModelDeviceKindPreview_LearningDeviceAny = 0,
                        LearningModelDeviceKindPreview_LearningDeviceCpu = 1,
                        LearningModelDeviceKindPreview_LearningDeviceGpu = 2,
                        LearningModelDeviceKindPreview_LearningDeviceNpu = 3,
                        LearningModelDeviceKindPreview_LearningDeviceDsp = 4,
                        LearningModelDeviceKindPreview_LearningDeviceFpga = 5,
                    };
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.Preview.LearningModelFeatureKindPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    enum
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use LearningModelFeatureKind instead of LearningModelFeatureKindPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    LearningModelFeatureKindPreview : int
                    {
                        LearningModelFeatureKindPreview_Undefined = 0,
                        LearningModelFeatureKindPreview_Tensor = 1,
                        LearningModelFeatureKindPreview_Sequence = 2,
                        LearningModelFeatureKindPreview_Map = 3,
                        LearningModelFeatureKindPreview_Image = 4,
                    };
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_IImageVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("7ae1fa72-029e-4dc5-a2f8-5fb763154150")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    IImageVariableDescriptorPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_BitmapPixelFormat(
                            ABI::Windows::Graphics::Imaging::BitmapPixelFormat* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Width(
                            UINT32* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Height(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IImageVariableDescriptorPreview = __uuidof(IImageVariableDescriptorPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.InferencingOptionsPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_IInferencingOptionsPreview[] = L"Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("47bc8205-4d36-47a9-8f68-ffcb339dd0fc")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    IInferencingOptionsPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_PreferredDeviceKind(
                            ABI::Windows::AI::MachineLearning::Preview::LearningModelDeviceKindPreview* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_PreferredDeviceKind(
                            ABI::Windows::AI::MachineLearning::Preview::LearningModelDeviceKindPreview value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_IsTracingEnabled(
                            boolean* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_IsTracingEnabled(
                            boolean value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_MaxBatchSize(
                            INT32* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_MaxBatchSize(
                            INT32 value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_MinimizeMemoryAllocation(
                            boolean* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_MinimizeMemoryAllocation(
                            boolean value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_ReclaimMemoryAfterEvaluation(
                            boolean* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_ReclaimMemoryAfterEvaluation(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInferencingOptionsPreview = __uuidof(IInferencingOptionsPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelBindingPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMapView`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelBindingPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("93c901e8-6c78-4b4f-aec1-a6bb9e691624")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelBindingPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE Bind(
                            HSTRING name,
                            IInspectable* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE BindWithProperties(
                            HSTRING name,
                            IInspectable* value,
                            ABI::Windows::Foundation::Collections::IPropertySet* metadata
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelBindingPreview = __uuidof(ILearningModelBindingPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelBindingPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelBindingPreviewFactory[] = L"Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("48b8219f-1e51-4d77-ae50-3ec164ad3480")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModelBindingFactory instead of ILearningModelBindingPreviewFactory. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelBindingPreviewFactory : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelBindingFactory instead of ILearningModelBindingPreviewFactory. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE CreateFromModel(
                            ABI::Windows::AI::MachineLearning::Preview::ILearningModelPreview* model,
                            ABI::Windows::AI::MachineLearning::Preview::ILearningModelBindingPreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelBindingPreviewFactory = __uuidof(ILearningModelBindingPreviewFactory);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelDescriptionPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("f52c09c6-8611-40ad-8e59-de3fd7030a40")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelDescriptionPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Author(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Domain(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Version(
                            INT64* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Metadata(
                            __FIMapView_2_HSTRING_HSTRING** value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_InputFeatures(
                            __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview** value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_OutputFeatures(
                            __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelDescriptionPreview = __uuidof(ILearningModelDescriptionPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelEvaluationResultPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("df25ea9f-9863-4088-8498-87a1f4686f92")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelEvaluationResultPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_CorrelationId(
                            HSTRING* correlationId
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Outputs(
                            __FIMapView_2_HSTRING_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelEvaluationResultPreview = __uuidof(ILearningModelEvaluationResultPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("049c266a-93b4-478c-aeb8-70157bf0ff94")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE EvaluateAsync(
                            ABI::Windows::AI::MachineLearning::Preview::ILearningModelBindingPreview* binding,
                            HSTRING correlationId,
                            __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview** evalOperation
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE EvaluateFeaturesAsync(
                            __FIMap_2_HSTRING_IInspectable* features,
                            HSTRING correlationId,
                            __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview** evalOperation
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            ABI::Windows::AI::MachineLearning::Preview::ILearningModelDescriptionPreview** returnValue
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_InferencingOptions(
                            ABI::Windows::AI::MachineLearning::Preview::IInferencingOptionsPreview** value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE put_InferencingOptions(
                            ABI::Windows::AI::MachineLearning::Preview::IInferencingOptionsPreview* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelPreview = __uuidof(ILearningModelPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelPreviewStatics[] = L"Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("164bbb60-8465-4786-8b93-2c16a89289d7")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelPreviewStatics : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE LoadModelFromStorageFileAsync(
                            ABI::Windows::Storage::IStorageFile* modelFile,
                            __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview** modelCreationOperation
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE LoadModelFromStreamAsync(
                            ABI::Windows::Storage::Streams::IRandomAccessStreamReference* modelStream,
                            __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview** modelCreationOperation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelPreviewStatics = __uuidof(ILearningModelPreviewStatics);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("b13df682-fc30-492b-8ea0-ed1f53c0b038")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ILearningModelVariableDescriptorPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_ModelFeatureKind(
                            ABI::Windows::AI::MachineLearning::Preview::LearningModelFeatureKindPreview* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_IsRequired(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILearningModelVariableDescriptorPreview = __uuidof(ILearningModelVariableDescriptorPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_IMapVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("3cb38370-c02b-4236-b3e8-6bdca49c3129")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    IMapVariableDescriptorPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_KeyKind(
                            ABI::Windows::AI::MachineLearning::Preview::FeatureElementKindPreview* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_ValidStringKeys(
                            __FIIterable_1_HSTRING** value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_ValidIntegerKeys(
                            __FIIterable_1___z__zint64** value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Fields(
                            ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMapVariableDescriptorPreview = __uuidof(IMapVariableDescriptorPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ISequenceVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("9cd8f292-98b2-4530-a1b6-2ded5fecbc26")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ISequenceFeatureDescriptor instead of ISequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ISequenceVariableDescriptorPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ISequenceFeatureDescriptor instead of ISequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_ElementType(
                            ABI::Windows::AI::MachineLearning::Preview::ILearningModelVariableDescriptorPreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISequenceVariableDescriptorPreview = __uuidof(ISequenceVariableDescriptorPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ITensorVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace MachineLearning {
                namespace Preview {
                    MIDL_INTERFACE("a80f501a-9aac-4233-9784-aceaf92510b5")
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                    ITensorVariableDescriptorPreview : public IInspectable
                    {
                    public:
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_DataType(
                            ABI::Windows::AI::MachineLearning::Preview::FeatureElementKindPreview* value
                            ) = 0;
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
                        virtual HRESULT STDMETHODCALLTYPE get_Shape(
                            __FIIterable_1___z__zint64** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITensorVariableDescriptorPreview = __uuidof(ITensorVariableDescriptorPreview);
                } /* Preview */
            } /* MachineLearning */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_ImageVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_ImageVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ImageFeatureDescriptor instead of ImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_ImageVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.InferencingOptionsPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_InferencingOptionsPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_InferencingOptionsPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelSession instead of InferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_InferencingOptionsPreview[] = L"Windows.AI.MachineLearning.Preview.InferencingOptionsPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelBindingPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelBindingPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelBindingPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelBinding instead of LearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelBindingPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelBindingPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelDescriptionPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelDescriptionPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModel instead of LearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelDescriptionPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelEvaluationResultPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelEvaluationResultPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelEvaluationResult instead of LearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelEvaluationResultPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModel instead of LearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelFeatureDescriptor instead of LearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_MapVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_MapVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use MapFeatureDescriptor instead of MapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_MapVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_SequenceVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_SequenceVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use SequenceFeatureDescriptor instead of SequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_SequenceVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_TensorVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_TensorVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use TensorFeatureDescriptor instead of TensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_TensorVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview;

#endif // ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview;

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview;

typedef struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreviewVtbl;

interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* This,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreviewVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview;

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview;

typedef struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreviewVtbl;

interface __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* This,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreviewVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

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

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview;

typedef struct __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreviewVtbl;

interface __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview
{
    CONST_VTBL struct __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview;

typedef struct __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview* This,
        __FIIterator_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview** result);

    END_INTERFACE
} __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreviewVtbl;

interface __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview
{
    CONST_VTBL struct __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CFeatureElementKindPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CFeatureElementKindPreview;

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelDeviceKindPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelDeviceKindPreview;

typedef enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelFeatureKindPreview __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelFeatureKindPreview;

/*
 *
 * Struct Windows.AI.MachineLearning.Preview.FeatureElementKindPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use TensorKind instead of FeatureElementKindPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CFeatureElementKindPreview
{
    FeatureElementKindPreview_Undefined = 0,
    FeatureElementKindPreview_Float = 1,
    FeatureElementKindPreview_UInt8 = 2,
    FeatureElementKindPreview_Int8 = 3,
    FeatureElementKindPreview_UInt16 = 4,
    FeatureElementKindPreview_Int16 = 5,
    FeatureElementKindPreview_Int32 = 6,
    FeatureElementKindPreview_Int64 = 7,
    FeatureElementKindPreview_String = 8,
    FeatureElementKindPreview_Boolean = 9,
    FeatureElementKindPreview_Float16 = 10,
    FeatureElementKindPreview_Double = 11,
    FeatureElementKindPreview_UInt32 = 12,
    FeatureElementKindPreview_UInt64 = 13,
    FeatureElementKindPreview_Complex64 = 14,
    FeatureElementKindPreview_Complex128 = 15,
};
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.Preview.LearningModelDeviceKindPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelDeviceKind instead of LearningModelDeviceKindPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelDeviceKindPreview
{
    LearningModelDeviceKindPreview_LearningDeviceAny = 0,
    LearningModelDeviceKindPreview_LearningDeviceCpu = 1,
    LearningModelDeviceKindPreview_LearningDeviceGpu = 2,
    LearningModelDeviceKindPreview_LearningDeviceNpu = 3,
    LearningModelDeviceKindPreview_LearningDeviceDsp = 4,
    LearningModelDeviceKindPreview_LearningDeviceFpga = 5,
};
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.MachineLearning.Preview.LearningModelFeatureKindPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelFeatureKind instead of LearningModelFeatureKindPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelFeatureKindPreview
{
    LearningModelFeatureKindPreview_Undefined = 0,
    LearningModelFeatureKindPreview_Tensor = 1,
    LearningModelFeatureKindPreview_Sequence = 2,
    LearningModelFeatureKindPreview_Map = 3,
    LearningModelFeatureKindPreview_Image = 4,
};
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_IImageVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_BitmapPixelFormat)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        UINT32* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_get_BitmapPixelFormat(This, value) \
    ((This)->lpVtbl->get_BitmapPixelFormat(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IImageFeatureDescriptor instead of IImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIImageVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.InferencingOptionsPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_IInferencingOptionsPreview[] = L"Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_PreferredDeviceKind)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelDeviceKindPreview* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_PreferredDeviceKind)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelDeviceKindPreview value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_IsTracingEnabled)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        boolean* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_IsTracingEnabled)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        boolean value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_MaxBatchSize)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        INT32* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_MaxBatchSize)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        INT32 value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_MinimizeMemoryAllocation)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        boolean* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_MinimizeMemoryAllocation)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        boolean value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ReclaimMemoryAfterEvaluation)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        boolean* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_ReclaimMemoryAfterEvaluation)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_get_PreferredDeviceKind(This, value) \
    ((This)->lpVtbl->get_PreferredDeviceKind(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_put_PreferredDeviceKind(This, value) \
    ((This)->lpVtbl->put_PreferredDeviceKind(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_get_IsTracingEnabled(This, value) \
    ((This)->lpVtbl->get_IsTracingEnabled(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_put_IsTracingEnabled(This, value) \
    ((This)->lpVtbl->put_IsTracingEnabled(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_get_MaxBatchSize(This, value) \
    ((This)->lpVtbl->get_MaxBatchSize(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_put_MaxBatchSize(This, value) \
    ((This)->lpVtbl->put_MaxBatchSize(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_get_MinimizeMemoryAllocation(This, value) \
    ((This)->lpVtbl->get_MinimizeMemoryAllocation(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_put_MinimizeMemoryAllocation(This, value) \
    ((This)->lpVtbl->put_MinimizeMemoryAllocation(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_get_ReclaimMemoryAfterEvaluation(This, value) \
    ((This)->lpVtbl->get_ReclaimMemoryAfterEvaluation(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use LearningModel instead of IInferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_put_ReclaimMemoryAfterEvaluation(This, value) \
    ((This)->lpVtbl->put_ReclaimMemoryAfterEvaluation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelBindingPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMapView`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelBindingPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* Bind)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This,
        HSTRING name,
        IInspectable* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* BindWithProperties)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This,
        HSTRING name,
        IInspectable* value,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* metadata);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* This);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_Bind(This, name, value) \
    ((This)->lpVtbl->Bind(This, name, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_BindWithProperties(This, name, value, metadata) \
    ((This)->lpVtbl->BindWithProperties(This, name, value, metadata))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBinding instead of ILearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelBindingPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelBindingPreviewFactory[] = L"Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelBindingFactory instead of ILearningModelBindingPreviewFactory. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBindingFactory instead of ILearningModelBindingPreviewFactory. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* CreateFromModel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* model,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactoryVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelBindingFactory instead of ILearningModelBindingPreviewFactory. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_CreateFromModel(This, model, value) \
    ((This)->lpVtbl->CreateFromModel(This, model, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreviewFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelDescriptionPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        HSTRING* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        HSTRING* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Domain)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        HSTRING* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        HSTRING* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        INT64* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Metadata)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        __FIMapView_2_HSTRING_HSTRING** value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_InputFeatures)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview** value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_OutputFeatures)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview* This,
        __FIIterable_1_Windows__CAI__CMachineLearning__CPreview__CILearningModelVariableDescriptorPreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_Domain(This, value) \
    ((This)->lpVtbl->get_Domain(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_Metadata(This, value) \
    ((This)->lpVtbl->get_Metadata(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_InputFeatures(This, value) \
    ((This)->lpVtbl->get_InputFeatures(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_get_OutputFeatures(This, value) \
    ((This)->lpVtbl->get_OutputFeatures(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelEvaluationResultPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_CorrelationId)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This,
        HSTRING* correlationId);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Outputs)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_get_CorrelationId(This, correlationId) \
    ((This)->lpVtbl->get_CorrelationId(This, correlationId))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelEvaluationResult instead of ILearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_get_Outputs(This, value) \
    ((This)->lpVtbl->get_Outputs(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelEvaluationResultPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* EvaluateAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelBindingPreview* binding,
        HSTRING correlationId,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview** evalOperation);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* EvaluateFeaturesAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        __FIMap_2_HSTRING_IInspectable* features,
        HSTRING correlationId,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelEvaluationResultPreview** evalOperation);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelDescriptionPreview** returnValue);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_InferencingOptions)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview** value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_InferencingOptions)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIInferencingOptionsPreview* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_EvaluateAsync(This, binding, correlationId, evalOperation) \
    ((This)->lpVtbl->EvaluateAsync(This, binding, correlationId, evalOperation))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_EvaluateFeaturesAsync(This, features, correlationId, evalOperation) \
    ((This)->lpVtbl->EvaluateFeaturesAsync(This, features, correlationId, evalOperation))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_get_Description(This, returnValue) \
    ((This)->lpVtbl->get_Description(This, returnValue))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_get_InferencingOptions(This, value) \
    ((This)->lpVtbl->get_InferencingOptions(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModel instead of ILearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_put_InferencingOptions(This, value) \
    ((This)->lpVtbl->put_InferencingOptions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.LearningModelPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelPreviewStatics[] = L"Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* LoadModelFromStorageFileAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* modelFile,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview** modelCreationOperation);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* LoadModelFromStreamAsync)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* modelStream,
        __FIAsyncOperation_1_Windows__CAI__CMachineLearning__CPreview__CLearningModelPreview** modelCreationOperation);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStaticsVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_LoadModelFromStorageFileAsync(This, modelFile, modelCreationOperation) \
    ((This)->lpVtbl->LoadModelFromStorageFileAsync(This, modelFile, modelCreationOperation))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelStatics instead of ILearningModelPreviewStatics. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_LoadModelFromStreamAsync(This, modelStream, modelCreationOperation) \
    ((This)->lpVtbl->LoadModelFromStreamAsync(This, modelStream, modelCreationOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ILearningModelVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        HSTRING* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        HSTRING* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ModelFeatureKind)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CLearningModelFeatureKindPreview* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_IsRequired)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_get_ModelFeatureKind(This, value) \
    ((This)->lpVtbl->get_ModelFeatureKind(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ILearningModelFeatureDescriptor instead of ILearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_get_IsRequired(This, value) \
    ((This)->lpVtbl->get_IsRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_IMapVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_KeyKind)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CFeatureElementKindPreview* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ValidStringKeys)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        __FIIterable_1_HSTRING** value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ValidIntegerKeys)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        __FIIterable_1___z__zint64** value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Fields)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_get_KeyKind(This, value) \
    ((This)->lpVtbl->get_KeyKind(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_get_ValidStringKeys(This, value) \
    ((This)->lpVtbl->get_ValidStringKeys(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_get_ValidIntegerKeys(This, value) \
    ((This)->lpVtbl->get_ValidIntegerKeys(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use IMapFeatureDescriptor instead of IMapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_get_Fields(This, value) \
    ((This)->lpVtbl->get_Fields(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CIMapVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ISequenceVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ISequenceFeatureDescriptor instead of ISequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ISequenceFeatureDescriptor instead of ISequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ElementType)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview* This,
        __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CILearningModelVariableDescriptorPreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ISequenceFeatureDescriptor instead of ISequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_get_ElementType(This, value) \
    ((This)->lpVtbl->get_ElementType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CISequenceVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_MachineLearning_Preview_ITensorVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview";
typedef struct
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_DataType)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This,
        enum __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CFeatureElementKindPreview* value);
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Shape)(__x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview* This,
        __FIIterable_1___z__zint64** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreviewVtbl;

interface __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_get_DataType(This, value) \
    ((This)->lpVtbl->get_DataType(This, value))

#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
    DEPRECATED("Use ITensorFeatureDescriptor instead of ITensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_get_Shape(This, value) \
    ((This)->lpVtbl->get_Shape(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview;
#endif /* !defined(____x_ABI_CWindows_CAI_CMachineLearning_CPreview_CITensorVariableDescriptorPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_ImageVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_ImageVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ImageFeatureDescriptor instead of ImageVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_ImageVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.InferencingOptionsPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_InferencingOptionsPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_InferencingOptionsPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelSession instead of InferencingOptionsPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_InferencingOptionsPreview[] = L"Windows.AI.MachineLearning.Preview.InferencingOptionsPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelBindingPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory interface starting with version 1.0 of the Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelBindingPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelBindingPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelBinding instead of LearningModelBindingPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelBindingPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelBindingPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelDescriptionPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelDescriptionPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModel instead of LearningModelDescriptionPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelDescriptionPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelEvaluationResultPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelEvaluationResultPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModelEvaluationResult instead of LearningModelEvaluationResultPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelEvaluationResultPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics interface starting with version 1.0 of the Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use LearningModel instead of LearningModelPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview ** Default Interface **
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_LearningModelVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use ILearningModelFeatureDescriptor instead of LearningModelVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_LearningModelVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_MapVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_MapVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use MapFeatureDescriptor instead of MapVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_MapVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_SequenceVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_SequenceVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use SequenceFeatureDescriptor instead of SequenceVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_SequenceVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview
 *
 * Introduced to Windows.AI.MachineLearning.Preview.MachineLearningPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview ** Default Interface **
 *    Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview
 *
 */
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_MachineLearning_Preview_TensorVariableDescriptorPreview_DEFINED
#define RUNTIMECLASS_Windows_AI_MachineLearning_Preview_TensorVariableDescriptorPreview_DEFINED
#if WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
DEPRECATED("Use TensorFeatureDescriptor instead of TensorVariableDescriptorPreview. For more info, see MSDN.")
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_MachineLearning_Preview_TensorVariableDescriptorPreview[] = L"Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview";
#endif
#endif // WINDOWS_AI_MACHINELEARNING_PREVIEW_MACHINELEARNINGPREVIEWCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eai2Emachinelearning2Epreview_p_h__

#endif // __windows2Eai2Emachinelearning2Epreview_h__
