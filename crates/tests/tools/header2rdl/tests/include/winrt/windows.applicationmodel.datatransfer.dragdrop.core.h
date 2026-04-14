
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
#ifndef __windows2Eapplicationmodel2Edatatransfer2Edragdrop2Ecore_h__
#define __windows2Eapplicationmodel2Edatatransfer2Edragdrop2Ecore_h__
#ifndef __windows2Eapplicationmodel2Edatatransfer2Edragdrop2Ecore_p_h__
#define __windows2Eapplicationmodel2Edatatransfer2Edragdrop2Ecore_p_h__


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
#include "Windows.ApplicationModel.DataTransfer.h"
#include "Windows.ApplicationModel.DataTransfer.DragDrop.h"
#include "Windows.Graphics.Imaging.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragDropManager;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragDropManager

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragDropManagerStatics;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragDropManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragInfo;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragInfo2;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2 ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragInfo2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragOperation;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragOperation2;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2 ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragOperation2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDragUIOverride;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragUIOverride

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDropOperationTarget;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDropOperationTarget

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        interface ICoreDropOperationTargetRequestedEventArgs;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDropOperationTargetRequestedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                typedef enum DataPackageOperation : unsigned int DataPackageOperation;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8b98aea9-64f0-5672-b30e-dfd9c2e4f6fe"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.DataTransfer.DataPackageOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation> __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("add21d46-17df-5a43-a685-3262fce84643"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.DataTransfer.DataPackageOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        class CoreDragDropManager;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        class CoreDropOperationTargetRequestedEventArgs;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4c3b1c1-b8ad-58cb-acc0-8ef37eae4ed4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragDropManager*, ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDropOperationTargetRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragDropManager*, ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragDropManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDropOperationTargetRequestedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDropOperationTargetRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager, Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragDropManager*, ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDropOperationTargetRequestedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackage;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackage;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage ABI::Windows::ApplicationModel::DataTransfer::IDataPackage

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackageView;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackageView;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    typedef enum DragDropModifiers : unsigned int DragDropModifiers;
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
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
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap ABI::Windows::Graphics::Imaging::ISoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        typedef enum CoreDragUIContentMode : unsigned int CoreDragUIContentMode;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        class CoreDragInfo;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        class CoreDragUIOverride;
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        enum CoreDragUIContentMode : unsigned int
                        {
                            CoreDragUIContentMode_Auto = 0,
                            CoreDragUIContentMode_Deferred = 0x1,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(CoreDragUIContentMode)
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragDropManager[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("7d56d344-8464-4faf-aa49-37ea6e2d7bd1")
                        ICoreDragDropManager : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE add_TargetRequested(
                                __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs* value,
                                EventRegistrationToken* returnValue
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_TargetRequested(
                                EventRegistrationToken value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AreConcurrentOperationsEnabled(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AreConcurrentOperationsEnabled(
                                boolean value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragDropManager = __uuidof(ICoreDragDropManager);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragDropManagerStatics[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("9542fdca-da12-4c1c-8d06-041db29733c3")
                        ICoreDragDropManagerStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragDropManager** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragDropManagerStatics = __uuidof(ICoreDragDropManagerStatics);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragInfo[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("48353a8b-cb50-464e-9575-cd4e3a7ab028")
                        ICoreDragInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Data(
                                ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Modifiers(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Position(
                                ABI::Windows::Foundation::Point* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragInfo = __uuidof(ICoreDragInfo);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragInfo2[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("c54691e5-e6fb-4d74-b4b1-8a3c17f25e9e")
                        ICoreDragInfo2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AllowedOperations(
                                ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragInfo2 = __uuidof(ICoreDragInfo2);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragOperation[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("cc06de4f-6db0-4e62-ab1b-a74a02dc6d85")
                        ICoreDragOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Data(
                                ABI::Windows::ApplicationModel::DataTransfer::IDataPackage** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetPointerId(
                                UINT32 pointerId
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetDragUIContentFromSoftwareBitmap(
                                ABI::Windows::Graphics::Imaging::ISoftwareBitmap* softwareBitmap
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetDragUIContentFromSoftwareBitmapWithAnchorPoint(
                                ABI::Windows::Graphics::Imaging::ISoftwareBitmap* softwareBitmap,
                                ABI::Windows::Foundation::Point anchorPoint
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DragUIContentMode(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragUIContentMode* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_DragUIContentMode(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragUIContentMode value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartAsync(
                                __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragOperation = __uuidof(ICoreDragOperation);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragOperation2[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("824b1e2c-d99a-4fc3-8507-6c182f33b46a")
                        ICoreDragOperation2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AllowedOperations(
                                ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AllowedOperations(
                                ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragOperation2 = __uuidof(ICoreDragOperation2);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragUIOverride[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("89a85064-3389-4f4f-8897-7e8a3ffb3c93")
                        ICoreDragUIOverride : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE SetContentFromSoftwareBitmap(
                                ABI::Windows::Graphics::Imaging::ISoftwareBitmap* softwareBitmap
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetContentFromSoftwareBitmapWithAnchorPoint(
                                ABI::Windows::Graphics::Imaging::ISoftwareBitmap* softwareBitmap,
                                ABI::Windows::Foundation::Point anchorPoint
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsContentVisible(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_IsContentVisible(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Caption(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Caption(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsCaptionVisible(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_IsCaptionVisible(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsGlyphVisible(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_IsGlyphVisible(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDragUIOverride = __uuidof(ICoreDragUIOverride);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDropOperationTarget[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("d9126196-4c5b-417d-bb37-76381def8db4")
                        ICoreDropOperationTarget : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE EnterAsync(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragInfo* dragInfo,
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragUIOverride* dragUIOverride,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** returnValue
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE OverAsync(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragInfo* dragInfo,
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragUIOverride* dragUIOverride,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** returnValue
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE LeaveAsync(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragInfo* dragInfo,
                                ABI::Windows::Foundation::IAsyncAction** returnValue
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE DropAsync(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDragInfo* dragInfo,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** returnValue
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDropOperationTarget = __uuidof(ICoreDropOperationTarget);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDropOperationTargetRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace DragDrop {
                    namespace Core {
                        MIDL_INTERFACE("2aca929a-5e28-4ea6-829e-29134e665d6d")
                        ICoreDropOperationTargetRequestedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE SetTarget(
                                ABI::Windows::ApplicationModel::DataTransfer::DragDrop::Core::ICoreDropOperationTarget* target
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreDropOperationTargetRequestedEventArgs = __uuidof(ICoreDropOperationTargetRequestedEventArgs);
                    } /* Core */
                } /* DragDrop */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragDropManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragDropManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragDropManager[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragInfo[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragOperation[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragUIOverride_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragUIOverride_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragUIOverride[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDropOperationTargetRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDropOperationTargetRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDropOperationTargetRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperationVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CDragDropModifiers __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CDragDropModifiers;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CCoreDragUIContentMode __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CCoreDragUIContentMode;

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CCoreDragUIContentMode
{
    CoreDragUIContentMode_Auto = 0,
    CoreDragUIContentMode_Deferred = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragDropManager[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_TargetRequested)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDragDropManager_Windows__CApplicationModel__CDataTransfer__CDragDrop__CCore__CCoreDropOperationTargetRequestedEventArgs* value,
        EventRegistrationToken* returnValue);
    HRESULT (STDMETHODCALLTYPE* remove_TargetRequested)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        EventRegistrationToken value);
    HRESULT (STDMETHODCALLTYPE* get_AreConcurrentOperationsEnabled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AreConcurrentOperationsEnabled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_add_TargetRequested(This, value, returnValue) \
    ((This)->lpVtbl->add_TargetRequested(This, value, returnValue))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_remove_TargetRequested(This, value) \
    ((This)->lpVtbl->remove_TargetRequested(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_get_AreConcurrentOperationsEnabled(This, value) \
    ((This)->lpVtbl->get_AreConcurrentOperationsEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_put_AreConcurrentOperationsEnabled(This, value) \
    ((This)->lpVtbl->put_AreConcurrentOperationsEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragDropManagerStatics[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_GetForCurrentView(This, value) \
    ((This)->lpVtbl->GetForCurrentView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragDropManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragInfo[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** value);
    HRESULT (STDMETHODCALLTYPE* get_Modifiers)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CDragDropModifiers* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_get_Modifiers(This, value) \
    ((This)->lpVtbl->get_Modifiers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragInfo2[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowedOperations)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_get_AllowedOperations(This, value) \
    ((This)->lpVtbl->get_AllowedOperations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragOperation[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage** value);
    HRESULT (STDMETHODCALLTYPE* SetPointerId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        UINT32 pointerId);
    HRESULT (STDMETHODCALLTYPE* SetDragUIContentFromSoftwareBitmap)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* softwareBitmap);
    HRESULT (STDMETHODCALLTYPE* SetDragUIContentFromSoftwareBitmapWithAnchorPoint)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* softwareBitmap,
        struct __x_ABI_CWindows_CFoundation_CPoint anchorPoint);
    HRESULT (STDMETHODCALLTYPE* get_DragUIContentMode)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CCoreDragUIContentMode* value);
    HRESULT (STDMETHODCALLTYPE* put_DragUIContentMode)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CCoreDragUIContentMode value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_SetPointerId(This, pointerId) \
    ((This)->lpVtbl->SetPointerId(This, pointerId))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_SetDragUIContentFromSoftwareBitmap(This, softwareBitmap) \
    ((This)->lpVtbl->SetDragUIContentFromSoftwareBitmap(This, softwareBitmap))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_SetDragUIContentFromSoftwareBitmapWithAnchorPoint(This, softwareBitmap, anchorPoint) \
    ((This)->lpVtbl->SetDragUIContentFromSoftwareBitmapWithAnchorPoint(This, softwareBitmap, anchorPoint))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_get_DragUIContentMode(This, value) \
    ((This)->lpVtbl->get_DragUIContentMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_put_DragUIContentMode(This, value) \
    ((This)->lpVtbl->put_DragUIContentMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_StartAsync(This, value) \
    ((This)->lpVtbl->StartAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragOperation2[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowedOperations)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowedOperations)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_get_AllowedOperations(This, value) \
    ((This)->lpVtbl->get_AllowedOperations(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_put_AllowedOperations(This, value) \
    ((This)->lpVtbl->put_AllowedOperations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDragUIOverride[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverrideVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetContentFromSoftwareBitmap)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* softwareBitmap);
    HRESULT (STDMETHODCALLTYPE* SetContentFromSoftwareBitmapWithAnchorPoint)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* softwareBitmap,
        struct __x_ABI_CWindows_CFoundation_CPoint anchorPoint);
    HRESULT (STDMETHODCALLTYPE* get_IsContentVisible)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsContentVisible)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Caption)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Caption)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsCaptionVisible)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCaptionVisible)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsGlyphVisible)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsGlyphVisible)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverrideVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverrideVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_SetContentFromSoftwareBitmap(This, softwareBitmap) \
    ((This)->lpVtbl->SetContentFromSoftwareBitmap(This, softwareBitmap))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_SetContentFromSoftwareBitmapWithAnchorPoint(This, softwareBitmap, anchorPoint) \
    ((This)->lpVtbl->SetContentFromSoftwareBitmapWithAnchorPoint(This, softwareBitmap, anchorPoint))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_get_IsContentVisible(This, value) \
    ((This)->lpVtbl->get_IsContentVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_put_IsContentVisible(This, value) \
    ((This)->lpVtbl->put_IsContentVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_get_Caption(This, value) \
    ((This)->lpVtbl->get_Caption(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_put_Caption(This, value) \
    ((This)->lpVtbl->put_Caption(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_get_IsCaptionVisible(This, value) \
    ((This)->lpVtbl->get_IsCaptionVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_put_IsCaptionVisible(This, value) \
    ((This)->lpVtbl->put_IsCaptionVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_get_IsGlyphVisible(This, value) \
    ((This)->lpVtbl->get_IsGlyphVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_put_IsGlyphVisible(This, value) \
    ((This)->lpVtbl->put_IsGlyphVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDropOperationTarget[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* EnterAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* dragInfo,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* dragUIOverride,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** returnValue);
    HRESULT (STDMETHODCALLTYPE* OverAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* dragInfo,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragUIOverride* dragUIOverride,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** returnValue);
    HRESULT (STDMETHODCALLTYPE* LeaveAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* dragInfo,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** returnValue);
    HRESULT (STDMETHODCALLTYPE* DropAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDragInfo* dragInfo,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageOperation** returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_EnterAsync(This, dragInfo, dragUIOverride, returnValue) \
    ((This)->lpVtbl->EnterAsync(This, dragInfo, dragUIOverride, returnValue))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_OverAsync(This, dragInfo, dragUIOverride, returnValue) \
    ((This)->lpVtbl->OverAsync(This, dragInfo, dragUIOverride, returnValue))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_LeaveAsync(This, dragInfo, returnValue) \
    ((This)->lpVtbl->LeaveAsync(This, dragInfo, returnValue))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_DropAsync(This, dragInfo, returnValue) \
    ((This)->lpVtbl->DropAsync(This, dragInfo, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_DragDrop_Core_ICoreDropOperationTargetRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetTarget)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTarget* target);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_SetTarget(This, target) \
    ((This)->lpVtbl->SetTarget(This, target))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CDragDrop_CCore_CICoreDropOperationTargetRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragDropManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragDropManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragDropManager[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragInfo[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragOperation[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragUIOverride_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragUIOverride_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDragUIOverride[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDropOperationTargetRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDropOperationTargetRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DragDrop_Core_CoreDropOperationTargetRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
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
#endif // __windows2Eapplicationmodel2Edatatransfer2Edragdrop2Ecore_p_h__

#endif // __windows2Eapplicationmodel2Edatatransfer2Edragdrop2Ecore_h__
