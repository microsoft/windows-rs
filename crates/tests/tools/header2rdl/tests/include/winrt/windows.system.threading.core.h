
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
#ifndef __windows2Esystem2Ethreading2Ecore_h__
#define __windows2Esystem2Ethreading2Ecore_h__
#ifndef __windows2Esystem2Ethreading2Ecore_p_h__
#define __windows2Esystem2Ethreading2Ecore_p_h__


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
#include "Windows.System.Threading.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    interface ISignalHandler;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler ABI::Windows::System::Threading::Core::ISignalHandler

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    interface IPreallocatedWorkItem;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem ABI::Windows::System::Threading::Core::IPreallocatedWorkItem

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    interface IPreallocatedWorkItemFactory;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory ABI::Windows::System::Threading::Core::IPreallocatedWorkItemFactory

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    interface ISignalNotifier;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier ABI::Windows::System::Threading::Core::ISignalNotifier

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    interface ISignalNotifierStatics;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics ABI::Windows::System::Threading::Core::ISignalNotifierStatics

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                interface IWorkItemHandler;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler ABI::Windows::System::Threading::IWorkItemHandler

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                typedef enum WorkItemOptions : unsigned int WorkItemOptions;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                typedef enum WorkItemPriority : int WorkItemPriority;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    class PreallocatedWorkItem;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    class SignalNotifier;
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Delegate Windows.System.Threading.Core.SignalHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    MIDL_INTERFACE("923c402e-4721-440e-9dda-55b6f2e07710")
                    ISignalHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            ABI::Windows::System::Threading::Core::ISignalNotifier* signalNotifier,
                            boolean timedOut
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISignalHandler = __uuidof(ISignalHandler);
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.IPreallocatedWorkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.PreallocatedWorkItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_IPreallocatedWorkItem[] = L"Windows.System.Threading.Core.IPreallocatedWorkItem";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    MIDL_INTERFACE("b6daa9fc-bc5b-401a-a8b2-6e754d14daa6")
                    IPreallocatedWorkItem : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RunAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPreallocatedWorkItem = __uuidof(IPreallocatedWorkItem);
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.IPreallocatedWorkItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.PreallocatedWorkItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_IPreallocatedWorkItemFactory[] = L"Windows.System.Threading.Core.IPreallocatedWorkItemFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    MIDL_INTERFACE("e3d32b45-dfea-469b-82c5-f6e3cefdeafb")
                    IPreallocatedWorkItemFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateWorkItem(
                            ABI::Windows::System::Threading::IWorkItemHandler* handler,
                            ABI::Windows::System::Threading::Core::IPreallocatedWorkItem** workItem
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWorkItemWithPriority(
                            ABI::Windows::System::Threading::IWorkItemHandler* handler,
                            ABI::Windows::System::Threading::WorkItemPriority priority,
                            ABI::Windows::System::Threading::Core::IPreallocatedWorkItem** WorkItem
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWorkItemWithPriorityAndOptions(
                            ABI::Windows::System::Threading::IWorkItemHandler* handler,
                            ABI::Windows::System::Threading::WorkItemPriority priority,
                            ABI::Windows::System::Threading::WorkItemOptions options,
                            ABI::Windows::System::Threading::Core::IPreallocatedWorkItem** WorkItem
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPreallocatedWorkItemFactory = __uuidof(IPreallocatedWorkItemFactory);
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.ISignalNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.SignalNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_ISignalNotifier[] = L"Windows.System.Threading.Core.ISignalNotifier";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    MIDL_INTERFACE("14285e06-63a7-4713-b6d9-62f64b56fb8b")
                    ISignalNotifier : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Enable(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Terminate(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISignalNotifier = __uuidof(ISignalNotifier);
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.ISignalNotifierStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.SignalNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_ISignalNotifierStatics[] = L"Windows.System.Threading.Core.ISignalNotifierStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                namespace Core {
                    MIDL_INTERFACE("1c4e4566-8400-46d3-a115-7d0c0dfc9f62")
                    ISignalNotifierStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AttachToEvent(
                            HSTRING name,
                            ABI::Windows::System::Threading::Core::ISignalHandler* handler,
                            ABI::Windows::System::Threading::Core::ISignalNotifier** signalNotifier
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AttachToEventWithTimeout(
                            HSTRING name,
                            ABI::Windows::System::Threading::Core::ISignalHandler* handler,
                            ABI::Windows::Foundation::TimeSpan timeout,
                            ABI::Windows::System::Threading::Core::ISignalNotifier** signalNotifier
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AttachToSemaphore(
                            HSTRING name,
                            ABI::Windows::System::Threading::Core::ISignalHandler* handler,
                            ABI::Windows::System::Threading::Core::ISignalNotifier** signalNotifier
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AttachToSemaphoreWithTimeout(
                            HSTRING name,
                            ABI::Windows::System::Threading::Core::ISignalHandler* handler,
                            ABI::Windows::Foundation::TimeSpan timeout,
                            ABI::Windows::System::Threading::Core::ISignalNotifier** signalNotifier
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISignalNotifierStatics = __uuidof(ISignalNotifierStatics);
                } /* Core */
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.Core.PreallocatedWorkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Threading.Core.IPreallocatedWorkItemFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Threading.Core.IPreallocatedWorkItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_Core_PreallocatedWorkItem_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_Core_PreallocatedWorkItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_Core_PreallocatedWorkItem[] = L"Windows.System.Threading.Core.PreallocatedWorkItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.Core.SignalNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Threading.Core.ISignalNotifierStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Threading.Core.ISignalNotifier ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_Core_SignalNotifier_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_Core_SignalNotifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_Core_SignalNotifier[] = L"Windows.System.Threading.Core.SignalNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions;

typedef enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority;

/*
 *
 * Delegate Windows.System.Threading.Core.SignalHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* This,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* signalNotifier,
        boolean timedOut);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandlerVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_Invoke(This, signalNotifier, timedOut) \
    ((This)->lpVtbl->Invoke(This, signalNotifier, timedOut))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.IPreallocatedWorkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.PreallocatedWorkItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_IPreallocatedWorkItem[] = L"Windows.System.Threading.Core.IPreallocatedWorkItem";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RunAsync)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_RunAsync(This, operation) \
    ((This)->lpVtbl->RunAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.IPreallocatedWorkItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.PreallocatedWorkItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_IPreallocatedWorkItemFactory[] = L"Windows.System.Threading.Core.IPreallocatedWorkItemFactory";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWorkItem)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* handler,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem** workItem);
    HRESULT (STDMETHODCALLTYPE* CreateWorkItemWithPriority)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* handler,
        enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority priority,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem** WorkItem);
    HRESULT (STDMETHODCALLTYPE* CreateWorkItemWithPriorityAndOptions)(__x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory* This,
        __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* handler,
        enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority priority,
        enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions options,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItem** WorkItem);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_CreateWorkItem(This, handler, workItem) \
    ((This)->lpVtbl->CreateWorkItem(This, handler, workItem))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_CreateWorkItemWithPriority(This, handler, priority, WorkItem) \
    ((This)->lpVtbl->CreateWorkItemWithPriority(This, handler, priority, WorkItem))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_CreateWorkItemWithPriorityAndOptions(This, handler, priority, options, WorkItem) \
    ((This)->lpVtbl->CreateWorkItemWithPriorityAndOptions(This, handler, priority, options, WorkItem))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CIPreallocatedWorkItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.ISignalNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.SignalNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_ISignalNotifier[] = L"Windows.System.Threading.Core.ISignalNotifier";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Enable)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This);
    HRESULT (STDMETHODCALLTYPE* Terminate)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_Enable(This) \
    ((This)->lpVtbl->Enable(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_Terminate(This) \
    ((This)->lpVtbl->Terminate(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.Core.ISignalNotifierStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.Core.SignalNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_Core_ISignalNotifierStatics[] = L"Windows.System.Threading.Core.ISignalNotifierStatics";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AttachToEvent)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* handler,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier** signalNotifier);
    HRESULT (STDMETHODCALLTYPE* AttachToEventWithTimeout)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* handler,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan timeout,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier** signalNotifier);
    HRESULT (STDMETHODCALLTYPE* AttachToSemaphore)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* handler,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier** signalNotifier);
    HRESULT (STDMETHODCALLTYPE* AttachToSemaphoreWithTimeout)(__x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalHandler* handler,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan timeout,
        __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifier** signalNotifier);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_AttachToEvent(This, name, handler, signalNotifier) \
    ((This)->lpVtbl->AttachToEvent(This, name, handler, signalNotifier))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_AttachToEventWithTimeout(This, name, handler, timeout, signalNotifier) \
    ((This)->lpVtbl->AttachToEventWithTimeout(This, name, handler, timeout, signalNotifier))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_AttachToSemaphore(This, name, handler, signalNotifier) \
    ((This)->lpVtbl->AttachToSemaphore(This, name, handler, signalNotifier))

#define __x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_AttachToSemaphoreWithTimeout(This, name, handler, timeout, signalNotifier) \
    ((This)->lpVtbl->AttachToSemaphoreWithTimeout(This, name, handler, timeout, signalNotifier))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CCore_CISignalNotifierStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.Core.PreallocatedWorkItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Threading.Core.IPreallocatedWorkItemFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Threading.Core.IPreallocatedWorkItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_Core_PreallocatedWorkItem_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_Core_PreallocatedWorkItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_Core_PreallocatedWorkItem[] = L"Windows.System.Threading.Core.PreallocatedWorkItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.Core.SignalNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Threading.Core.ISignalNotifierStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Threading.Core.ISignalNotifier ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_Core_SignalNotifier_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_Core_SignalNotifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_Core_SignalNotifier[] = L"Windows.System.Threading.Core.SignalNotifier";
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
#endif // __windows2Esystem2Ethreading2Ecore_p_h__

#endif // __windows2Esystem2Ethreading2Ecore_h__
