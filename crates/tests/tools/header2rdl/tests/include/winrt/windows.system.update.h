
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
#ifndef __windows2Esystem2Eupdate_h__
#define __windows2Esystem2Eupdate_h__
#ifndef __windows2Esystem2Eupdate_p_h__
#define __windows2Esystem2Eupdate_p_h__


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

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                interface ISystemUpdateItem;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem ABI::Windows::System::Update::ISystemUpdateItem

#endif // ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                interface ISystemUpdateLastErrorInfo;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo ABI::Windows::System::Update::ISystemUpdateLastErrorInfo

#endif // ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                interface ISystemUpdateManagerStatics;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics ABI::Windows::System::Update::ISystemUpdateManagerStatics

#endif // ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_FWD_DEFINED__

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
        namespace System {
            namespace Update {
                class SystemUpdateItem;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE
#define DEF___FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4ae5176-c068-542f-81b4-8900f72bd742"))
IIterator<ABI::Windows::System::Update::SystemUpdateItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Update::SystemUpdateItem*, ABI::Windows::System::Update::ISystemUpdateItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.Update.SystemUpdateItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::Update::SystemUpdateItem*> __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_t;
#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE */

#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE
#define DEF___FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("318a3078-918c-5521-b460-0b4210360aa1"))
IIterable<ABI::Windows::System::Update::SystemUpdateItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Update::SystemUpdateItem*, ABI::Windows::System::Update::ISystemUpdateItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.Update.SystemUpdateItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::Update::SystemUpdateItem*> __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_t;
#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE */

#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000


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


#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE
#define DEF___FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7c77b64c-8be2-50e0-8ca5-d8265d80902b"))
IVectorView<ABI::Windows::System::Update::SystemUpdateItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Update::SystemUpdateItem*, ABI::Windows::System::Update::ISystemUpdateItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.Update.SystemUpdateItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::Update::SystemUpdateItem*> __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_t;
#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_USE */

#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000


#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


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
        namespace System {
            namespace Update {
                typedef enum SystemUpdateAttentionRequiredReason : int SystemUpdateAttentionRequiredReason;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                typedef enum SystemUpdateItemState : int SystemUpdateItemState;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                typedef enum SystemUpdateManagerState : int SystemUpdateManagerState;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                typedef enum SystemUpdateStartInstallAction : int SystemUpdateStartInstallAction;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                class SystemUpdateLastErrorInfo;
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Update.SystemUpdateAttentionRequiredReason
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                enum SystemUpdateAttentionRequiredReason : int
                {
                    SystemUpdateAttentionRequiredReason_None = 0,
                    SystemUpdateAttentionRequiredReason_NetworkRequired = 1,
                    SystemUpdateAttentionRequiredReason_InsufficientDiskSpace = 2,
                    SystemUpdateAttentionRequiredReason_InsufficientBattery = 3,
                    SystemUpdateAttentionRequiredReason_UpdateBlocked = 4,
                };
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.Update.SystemUpdateItemState
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                enum SystemUpdateItemState : int
                {
                    SystemUpdateItemState_NotStarted = 0,
                    SystemUpdateItemState_Initializing = 1,
                    SystemUpdateItemState_Preparing = 2,
                    SystemUpdateItemState_Calculating = 3,
                    SystemUpdateItemState_Downloading = 4,
                    SystemUpdateItemState_Installing = 5,
                    SystemUpdateItemState_Completed = 6,
                    SystemUpdateItemState_RebootRequired = 7,
                    SystemUpdateItemState_Error = 8,
                };
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.Update.SystemUpdateManagerState
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                enum SystemUpdateManagerState : int
                {
                    SystemUpdateManagerState_Idle = 0,
                    SystemUpdateManagerState_Detecting = 1,
                    SystemUpdateManagerState_ReadyToDownload = 2,
                    SystemUpdateManagerState_Downloading = 3,
                    SystemUpdateManagerState_ReadyToInstall = 4,
                    SystemUpdateManagerState_Installing = 5,
                    SystemUpdateManagerState_RebootRequired = 6,
                    SystemUpdateManagerState_ReadyToFinalize = 7,
                    SystemUpdateManagerState_Finalizing = 8,
                    SystemUpdateManagerState_Completed = 9,
                    SystemUpdateManagerState_AttentionRequired = 10,
                    SystemUpdateManagerState_Error = 11,
                };
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.Update.SystemUpdateStartInstallAction
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                enum SystemUpdateStartInstallAction : int
                {
                    SystemUpdateStartInstallAction_UpToReboot = 0,
                    SystemUpdateStartInstallAction_AllowReboot = 1,
                };
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Update.ISystemUpdateItem
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Update.SystemUpdateItem
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Update_ISystemUpdateItem[] = L"Windows.System.Update.ISystemUpdateItem";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                MIDL_INTERFACE("779740eb-5624-519e-a8e2-09e9173b3fb7")
                ISystemUpdateItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::System::Update::SystemUpdateItemState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Revision(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DownloadProgress(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstallProgress(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemUpdateItem = __uuidof(ISystemUpdateItem);
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Update.ISystemUpdateLastErrorInfo
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Update.SystemUpdateLastErrorInfo
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Update_ISystemUpdateLastErrorInfo[] = L"Windows.System.Update.ISystemUpdateLastErrorInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                MIDL_INTERFACE("7ee887f7-8a44-5b6e-bd07-7aece4116ea9")
                ISystemUpdateLastErrorInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::System::Update::SystemUpdateManagerState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInteractive(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemUpdateLastErrorInfo = __uuidof(ISystemUpdateLastErrorInfo);
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Update.ISystemUpdateManagerStatics
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Update.SystemUpdateManager
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Update_ISystemUpdateManagerStatics[] = L"Windows.System.Update.ISystemUpdateManagerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Update {
                MIDL_INTERFACE("b2d3fcef-2971-51be-b41a-8bd703bb701a")
                ISystemUpdateManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::System::Update::SystemUpdateManagerState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DownloadProgress(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstallProgress(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserActiveHoursStart(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserActiveHoursEnd(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserActiveHoursMax(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetUserActiveHours(
                        ABI::Windows::Foundation::TimeSpan start,
                        ABI::Windows::Foundation::TimeSpan end,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastUpdateCheckTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastUpdateInstallTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastErrorInfo(
                        ABI::Windows::System::Update::ISystemUpdateLastErrorInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAutomaticRebootBlockIds(
                        __FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BlockAutomaticRebootAsync(
                        HSTRING lockId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnblockAutomaticRebootAsync(
                        HSTRING lockId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUpdateItems(
                        __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttentionRequiredReason(
                        ABI::Windows::System::Update::SystemUpdateAttentionRequiredReason* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetFlightRing(
                        HSTRING flightRing,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFlightRing(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartInstall(
                        ABI::Windows::System::Update::SystemUpdateStartInstallAction action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RebootToCompleteInstall(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartCancelUpdates(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemUpdateManagerStatics = __uuidof(ISystemUpdateManagerStatics);
            } /* Update */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Update.SystemUpdateItem
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Update.ISystemUpdateItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_Update_SystemUpdateItem_DEFINED
#define RUNTIMECLASS_Windows_System_Update_SystemUpdateItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Update_SystemUpdateItem[] = L"Windows.System.Update.SystemUpdateItem";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Update.SystemUpdateLastErrorInfo
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Update.ISystemUpdateLastErrorInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_Update_SystemUpdateLastErrorInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Update_SystemUpdateLastErrorInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Update_SystemUpdateLastErrorInfo[] = L"Windows.System.Update.SystemUpdateLastErrorInfo";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Update.SystemUpdateManager
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Update.ISystemUpdateManagerStatics interface starting with version 6.0 of the Windows.System.SystemManagementContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_Update_SystemUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_System_Update_SystemUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Update_SystemUpdateManager[] = L"Windows.System.Update.SystemUpdateManager";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem;

#endif // ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo;

#endif // ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics;

#endif // ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_FWD_DEFINED__

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

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem;

typedef struct __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl;

interface __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem;

typedef struct __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        __FIIterator_1_Windows__CSystem__CUpdate__CSystemUpdateItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl;

interface __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

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

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem;

typedef struct __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl;

interface __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateAttentionRequiredReason __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateAttentionRequiredReason;

typedef enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateItemState __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateItemState;

typedef enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateManagerState __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateManagerState;

typedef enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateStartInstallAction __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateStartInstallAction;

/*
 *
 * Struct Windows.System.Update.SystemUpdateAttentionRequiredReason
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateAttentionRequiredReason
{
    SystemUpdateAttentionRequiredReason_None = 0,
    SystemUpdateAttentionRequiredReason_NetworkRequired = 1,
    SystemUpdateAttentionRequiredReason_InsufficientDiskSpace = 2,
    SystemUpdateAttentionRequiredReason_InsufficientBattery = 3,
    SystemUpdateAttentionRequiredReason_UpdateBlocked = 4,
};
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.Update.SystemUpdateItemState
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateItemState
{
    SystemUpdateItemState_NotStarted = 0,
    SystemUpdateItemState_Initializing = 1,
    SystemUpdateItemState_Preparing = 2,
    SystemUpdateItemState_Calculating = 3,
    SystemUpdateItemState_Downloading = 4,
    SystemUpdateItemState_Installing = 5,
    SystemUpdateItemState_Completed = 6,
    SystemUpdateItemState_RebootRequired = 7,
    SystemUpdateItemState_Error = 8,
};
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.Update.SystemUpdateManagerState
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateManagerState
{
    SystemUpdateManagerState_Idle = 0,
    SystemUpdateManagerState_Detecting = 1,
    SystemUpdateManagerState_ReadyToDownload = 2,
    SystemUpdateManagerState_Downloading = 3,
    SystemUpdateManagerState_ReadyToInstall = 4,
    SystemUpdateManagerState_Installing = 5,
    SystemUpdateManagerState_RebootRequired = 6,
    SystemUpdateManagerState_ReadyToFinalize = 7,
    SystemUpdateManagerState_Finalizing = 8,
    SystemUpdateManagerState_Completed = 9,
    SystemUpdateManagerState_AttentionRequired = 10,
    SystemUpdateManagerState_Error = 11,
};
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.System.Update.SystemUpdateStartInstallAction
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateStartInstallAction
{
    SystemUpdateStartInstallAction_UpToReboot = 0,
    SystemUpdateStartInstallAction_AllowReboot = 1,
};
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Update.ISystemUpdateItem
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Update.SystemUpdateItem
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Update_ISystemUpdateItem[] = L"Windows.System.Update.ISystemUpdateItem";
typedef struct __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateItemState* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Revision)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DownloadProgress)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_InstallProgress)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItemVtbl;

interface __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_Revision(This, value) \
    ((This)->lpVtbl->get_Revision(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_DownloadProgress(This, value) \
    ((This)->lpVtbl->get_DownloadProgress(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_InstallProgress(This, value) \
    ((This)->lpVtbl->get_InstallProgress(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Update.ISystemUpdateLastErrorInfo
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Update.SystemUpdateLastErrorInfo
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Update_ISystemUpdateLastErrorInfo[] = L"Windows.System.Update.ISystemUpdateLastErrorInfo";
typedef struct __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateManagerState* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInteractive)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfoVtbl;

interface __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_get_IsInteractive(This, value) \
    ((This)->lpVtbl->get_IsInteractive(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Update.ISystemUpdateManagerStatics
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Update.SystemUpdateManager
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Update_ISystemUpdateManagerStatics[] = L"Windows.System.Update.ISystemUpdateManagerStatics";
typedef struct __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateManagerState* value);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_DownloadProgress)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_InstallProgress)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_UserActiveHoursStart)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UserActiveHoursEnd)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UserActiveHoursMax)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* TrySetUserActiveHours)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan start,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan end,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_LastUpdateCheckTime)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_LastUpdateInstallTime)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_LastErrorInfo)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateLastErrorInfo** value);
    HRESULT (STDMETHODCALLTYPE* GetAutomaticRebootBlockIds)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* BlockAutomaticRebootAsync)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        HSTRING lockId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* UnblockAutomaticRebootAsync)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        HSTRING lockId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* GetUpdateItems)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        __FIVectorView_1_Windows__CSystem__CUpdate__CSystemUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* get_AttentionRequiredReason)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateAttentionRequiredReason* value);
    HRESULT (STDMETHODCALLTYPE* SetFlightRing)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        HSTRING flightRing,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetFlightRing)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* StartInstall)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CUpdate_CSystemUpdateStartInstallAction action);
    HRESULT (STDMETHODCALLTYPE* RebootToCompleteInstall)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* StartCancelUpdates)(__x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_add_StateChanged(This, handler, token) \
    ((This)->lpVtbl->add_StateChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_remove_StateChanged(This, token) \
    ((This)->lpVtbl->remove_StateChanged(This, token))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_DownloadProgress(This, value) \
    ((This)->lpVtbl->get_DownloadProgress(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_InstallProgress(This, value) \
    ((This)->lpVtbl->get_InstallProgress(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_UserActiveHoursStart(This, value) \
    ((This)->lpVtbl->get_UserActiveHoursStart(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_UserActiveHoursEnd(This, value) \
    ((This)->lpVtbl->get_UserActiveHoursEnd(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_UserActiveHoursMax(This, value) \
    ((This)->lpVtbl->get_UserActiveHoursMax(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_TrySetUserActiveHours(This, start, end, result) \
    ((This)->lpVtbl->TrySetUserActiveHours(This, start, end, result))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_LastUpdateCheckTime(This, value) \
    ((This)->lpVtbl->get_LastUpdateCheckTime(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_LastUpdateInstallTime(This, value) \
    ((This)->lpVtbl->get_LastUpdateInstallTime(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_LastErrorInfo(This, value) \
    ((This)->lpVtbl->get_LastErrorInfo(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_GetAutomaticRebootBlockIds(This, result) \
    ((This)->lpVtbl->GetAutomaticRebootBlockIds(This, result))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_BlockAutomaticRebootAsync(This, lockId, operation) \
    ((This)->lpVtbl->BlockAutomaticRebootAsync(This, lockId, operation))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_UnblockAutomaticRebootAsync(This, lockId, operation) \
    ((This)->lpVtbl->UnblockAutomaticRebootAsync(This, lockId, operation))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_GetUpdateItems(This, result) \
    ((This)->lpVtbl->GetUpdateItems(This, result))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_get_AttentionRequiredReason(This, value) \
    ((This)->lpVtbl->get_AttentionRequiredReason(This, value))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_SetFlightRing(This, flightRing, result) \
    ((This)->lpVtbl->SetFlightRing(This, flightRing, result))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_GetFlightRing(This, result) \
    ((This)->lpVtbl->GetFlightRing(This, result))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_StartInstall(This, action) \
    ((This)->lpVtbl->StartInstall(This, action))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_RebootToCompleteInstall(This) \
    ((This)->lpVtbl->RebootToCompleteInstall(This))

#define __x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_StartCancelUpdates(This) \
    ((This)->lpVtbl->StartCancelUpdates(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUpdate_CISystemUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Update.SystemUpdateItem
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Update.ISystemUpdateItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_Update_SystemUpdateItem_DEFINED
#define RUNTIMECLASS_Windows_System_Update_SystemUpdateItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Update_SystemUpdateItem[] = L"Windows.System.Update.SystemUpdateItem";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Update.SystemUpdateLastErrorInfo
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Update.ISystemUpdateLastErrorInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_Update_SystemUpdateLastErrorInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Update_SystemUpdateLastErrorInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Update_SystemUpdateLastErrorInfo[] = L"Windows.System.Update.SystemUpdateLastErrorInfo";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.Update.SystemUpdateManager
 *
 * Introduced to Windows.System.SystemManagementContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Update.ISystemUpdateManagerStatics interface starting with version 6.0 of the Windows.System.SystemManagementContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_Update_SystemUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_System_Update_SystemUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Update_SystemUpdateManager[] = L"Windows.System.Update.SystemUpdateManager";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Eupdate_p_h__

#endif // __windows2Esystem2Eupdate_h__
