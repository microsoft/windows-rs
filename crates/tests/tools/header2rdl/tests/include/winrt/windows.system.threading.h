
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
#ifndef __windows2Esystem2Ethreading_h__
#define __windows2Esystem2Ethreading_h__
#ifndef __windows2Esystem2Ethreading_p_h__
#define __windows2Esystem2Ethreading_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                interface ITimerDestroyedHandler;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler ABI::Windows::System::Threading::ITimerDestroyedHandler

#endif // ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                interface ITimerElapsedHandler;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler ABI::Windows::System::Threading::ITimerElapsedHandler

#endif // ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                interface IThreadPoolStatics;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics ABI::Windows::System::Threading::IThreadPoolStatics

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                interface IThreadPoolTimer;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer ABI::Windows::System::Threading::IThreadPoolTimer

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                interface IThreadPoolTimerStatics;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics ABI::Windows::System::Threading::IThreadPoolTimerStatics

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_FWD_DEFINED__

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
                class ThreadPoolTimer;
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Threading.WorkItemOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                enum WorkItemOptions : unsigned int
                {
                    WorkItemOptions_None = 0,
                    WorkItemOptions_TimeSliced = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(WorkItemOptions)
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Threading.WorkItemPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                enum WorkItemPriority : int
                {
                    WorkItemPriority_Low = -1,
                    WorkItemPriority_Normal = 0,
                    WorkItemPriority_High = 1,
                };
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.System.Threading.TimerDestroyedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                MIDL_INTERFACE("34ed19fa-8384-4eb9-8209-fb5094eeec35")
                ITimerDestroyedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::System::Threading::IThreadPoolTimer* timer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITimerDestroyedHandler = __uuidof(ITimerDestroyedHandler);
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.System.Threading.TimerElapsedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                MIDL_INTERFACE("faaea667-fbeb-49cb-adb2-71184c556e43")
                ITimerElapsedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::System::Threading::IThreadPoolTimer* timer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITimerElapsedHandler = __uuidof(ITimerElapsedHandler);
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.System.Threading.WorkItemHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                MIDL_INTERFACE("1d1a8b8b-fa66-414f-9cbd-b65fc99d17fa")
                IWorkItemHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Foundation::IAsyncAction* operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWorkItemHandler = __uuidof(IWorkItemHandler);
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.IThreadPoolStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.ThreadPool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_IThreadPoolStatics[] = L"Windows.System.Threading.IThreadPoolStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                MIDL_INTERFACE("b6bf67dd-84bd-44f8-ac1c-93ebcb9dba91")
                IThreadPoolStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RunAsync(
                        ABI::Windows::System::Threading::IWorkItemHandler* handler,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RunWithPriorityAsync(
                        ABI::Windows::System::Threading::IWorkItemHandler* handler,
                        ABI::Windows::System::Threading::WorkItemPriority priority,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RunWithPriorityAndOptionsAsync(
                        ABI::Windows::System::Threading::IWorkItemHandler* handler,
                        ABI::Windows::System::Threading::WorkItemPriority priority,
                        ABI::Windows::System::Threading::WorkItemOptions options,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IThreadPoolStatics = __uuidof(IThreadPoolStatics);
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.IThreadPoolTimer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.ThreadPoolTimer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_IThreadPoolTimer[] = L"Windows.System.Threading.IThreadPoolTimer";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                MIDL_INTERFACE("594ebe78-55ea-4a88-a50d-3402ae1f9cf2")
                IThreadPoolTimer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Period(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Delay(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Cancel(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IThreadPoolTimer = __uuidof(IThreadPoolTimer);
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.IThreadPoolTimerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.ThreadPoolTimer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_IThreadPoolTimerStatics[] = L"Windows.System.Threading.IThreadPoolTimerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Threading {
                MIDL_INTERFACE("1a8a9d02-e482-461b-b8c7-8efad1cce590")
                IThreadPoolTimerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePeriodicTimer(
                        ABI::Windows::System::Threading::ITimerElapsedHandler* handler,
                        ABI::Windows::Foundation::TimeSpan period,
                        ABI::Windows::System::Threading::IThreadPoolTimer** timer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTimer(
                        ABI::Windows::System::Threading::ITimerElapsedHandler* handler,
                        ABI::Windows::Foundation::TimeSpan delay,
                        ABI::Windows::System::Threading::IThreadPoolTimer** timer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePeriodicTimerWithCompletion(
                        ABI::Windows::System::Threading::ITimerElapsedHandler* handler,
                        ABI::Windows::Foundation::TimeSpan period,
                        ABI::Windows::System::Threading::ITimerDestroyedHandler* destroyed,
                        ABI::Windows::System::Threading::IThreadPoolTimer** timer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTimerWithCompletion(
                        ABI::Windows::System::Threading::ITimerElapsedHandler* handler,
                        ABI::Windows::Foundation::TimeSpan delay,
                        ABI::Windows::System::Threading::ITimerDestroyedHandler* destroyed,
                        ABI::Windows::System::Threading::IThreadPoolTimer** timer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IThreadPoolTimerStatics = __uuidof(IThreadPoolTimerStatics);
            } /* Threading */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.ThreadPool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Threading.IThreadPoolStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_ThreadPool_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_ThreadPool_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_ThreadPool[] = L"Windows.System.Threading.ThreadPool";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.ThreadPoolTimer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Threading.IThreadPoolTimerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Threading.IThreadPoolTimer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_ThreadPoolTimer_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_ThreadPoolTimer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_ThreadPoolTimer[] = L"Windows.System.Threading.ThreadPoolTimer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics;

#endif // ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions;

typedef enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority;

/*
 *
 * Struct Windows.System.Threading.WorkItemOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions
{
    WorkItemOptions_None = 0,
    WorkItemOptions_TimeSliced = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Threading.WorkItemPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority
{
    WorkItemPriority_Low = -1,
    WorkItemPriority_Normal = 0,
    WorkItemPriority_High = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.System.Threading.TimerDestroyedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler* This,
        __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* timer);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandlerVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_Invoke(This, timer) \
    ((This)->lpVtbl->Invoke(This, timer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.System.Threading.TimerElapsedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* This,
        __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* timer);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandlerVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_Invoke(This, timer) \
    ((This)->lpVtbl->Invoke(This, timer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.System.Threading.WorkItemHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction* operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandlerVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_Invoke(This, operation) \
    ((This)->lpVtbl->Invoke(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.IThreadPoolStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.ThreadPool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_IThreadPoolStatics[] = L"Windows.System.Threading.IThreadPoolStatics";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RunAsync)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* handler,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RunWithPriorityAsync)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* handler,
        enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority priority,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RunWithPriorityAndOptionsAsync)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CIWorkItemHandler* handler,
        enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemPriority priority,
        enum __x_ABI_CWindows_CSystem_CThreading_CWorkItemOptions options,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_RunAsync(This, handler, operation) \
    ((This)->lpVtbl->RunAsync(This, handler, operation))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_RunWithPriorityAsync(This, handler, priority, operation) \
    ((This)->lpVtbl->RunWithPriorityAsync(This, handler, priority, operation))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_RunWithPriorityAndOptionsAsync(This, handler, priority, options, operation) \
    ((This)->lpVtbl->RunWithPriorityAndOptionsAsync(This, handler, priority, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.IThreadPoolTimer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.ThreadPoolTimer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_IThreadPoolTimer[] = L"Windows.System.Threading.IThreadPoolTimer";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Period)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Delay)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* Cancel)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_get_Period(This, value) \
    ((This)->lpVtbl->get_Period(This, value))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_get_Delay(This, value) \
    ((This)->lpVtbl->get_Delay(This, value))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_Cancel(This) \
    ((This)->lpVtbl->Cancel(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Threading.IThreadPoolTimerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Threading.ThreadPoolTimer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Threading_IThreadPoolTimerStatics[] = L"Windows.System.Threading.IThreadPoolTimerStatics";
typedef struct __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePeriodicTimer)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* handler,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan period,
        __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer** timer);
    HRESULT (STDMETHODCALLTYPE* CreateTimer)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* handler,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan delay,
        __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer** timer);
    HRESULT (STDMETHODCALLTYPE* CreatePeriodicTimerWithCompletion)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* handler,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan period,
        __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler* destroyed,
        __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer** timer);
    HRESULT (STDMETHODCALLTYPE* CreateTimerWithCompletion)(__x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics* This,
        __x_ABI_CWindows_CSystem_CThreading_CITimerElapsedHandler* handler,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan delay,
        __x_ABI_CWindows_CSystem_CThreading_CITimerDestroyedHandler* destroyed,
        __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimer** timer);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_CreatePeriodicTimer(This, handler, period, timer) \
    ((This)->lpVtbl->CreatePeriodicTimer(This, handler, period, timer))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_CreateTimer(This, handler, delay, timer) \
    ((This)->lpVtbl->CreateTimer(This, handler, delay, timer))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_CreatePeriodicTimerWithCompletion(This, handler, period, destroyed, timer) \
    ((This)->lpVtbl->CreatePeriodicTimerWithCompletion(This, handler, period, destroyed, timer))

#define __x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_CreateTimerWithCompletion(This, handler, delay, destroyed, timer) \
    ((This)->lpVtbl->CreateTimerWithCompletion(This, handler, delay, destroyed, timer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CThreading_CIThreadPoolTimerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.ThreadPool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Threading.IThreadPoolStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_ThreadPool_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_ThreadPool_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_ThreadPool[] = L"Windows.System.Threading.ThreadPool";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Threading.ThreadPoolTimer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Threading.IThreadPoolTimerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Threading.IThreadPoolTimer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Threading_ThreadPoolTimer_DEFINED
#define RUNTIMECLASS_Windows_System_Threading_ThreadPoolTimer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Threading_ThreadPoolTimer[] = L"Windows.System.Threading.ThreadPoolTimer";
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
#endif // __windows2Esystem2Ethreading_p_h__

#endif // __windows2Esystem2Ethreading_h__
