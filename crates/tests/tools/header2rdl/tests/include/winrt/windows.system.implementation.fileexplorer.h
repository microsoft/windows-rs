
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
#ifndef __windows2Esystem2Eimplementation2Efileexplorer_h__
#define __windows2Esystem2Eimplementation2Efileexplorer_h__
#ifndef __windows2Esystem2Eimplementation2Efileexplorer_p_h__
#define __windows2Esystem2Eimplementation2Efileexplorer_p_h__


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

#if !defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)
#define WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.Provider.h"
#include "Windows.Web.Http.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    interface ISysStorageProviderEventReceivedEventArgs;
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventReceivedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    interface ISysStorageProviderEventReceivedEventArgsFactory;
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventReceivedEventArgsFactory

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    interface ISysStorageProviderEventSource;
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventSource

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    interface ISysStorageProviderHandlerFactory;
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderHandlerFactory

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    interface ISysStorageProviderHttpRequestProvider;
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderHttpRequestProvider

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage ABI::Windows::Web::Http::IHttpResponseMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_USE
#define DEF___FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c3fdb0bb-ef3e-57d5-8135-fb9d30e48543"))
IAsyncOperation<ABI::Windows::Web::Http::HttpResponseMessage*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpResponseMessage*, ABI::Windows::Web::Http::IHttpResponseMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Web.Http.HttpResponseMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Web::Http::HttpResponseMessage*> __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_t;
#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cfe26c2b-ec28-5d16-b2b7-67fba5666944"))
IAsyncOperationCompletedHandler<ABI::Windows::Web::Http::HttpResponseMessage*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpResponseMessage*, ABI::Windows::Web::Http::IHttpResponseMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Web.Http.HttpResponseMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Web::Http::HttpResponseMessage*> __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    class SysStorageProviderEventReceivedEventArgs;
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("da34b79f-979c-5e94-82fb-d6c964e97ef6"))
ITypedEventHandler<ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventSource*, ABI::Windows::System::Implementation::FileExplorer::SysStorageProviderEventReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventSource*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Implementation::FileExplorer::SysStorageProviderEventReceivedEventArgs*, ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource, Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventSource*, ABI::Windows::System::Implementation::FileExplorer::SysStorageProviderEventReceivedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_USE */

#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage ABI::Windows::Web::Http::IHttpRequestMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderEventReceivedEventArgs[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    MIDL_INTERFACE("e132d1b9-7b9d-5820-9728-4262b5289142")
                    ISysStorageProviderEventReceivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Json(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISysStorageProviderEventReceivedEventArgs = __uuidof(ISysStorageProviderEventReceivedEventArgs);
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderEventReceivedEventArgsFactory[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    MIDL_INTERFACE("de1a780e-e975-5f68-bcc6-fb46281c6a61")
                    ISysStorageProviderEventReceivedEventArgsFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            HSTRING json,
                            ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventReceivedEventArgs** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISysStorageProviderEventReceivedEventArgsFactory = __uuidof(ISysStorageProviderEventReceivedEventArgsFactory);
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderEventSource[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    MIDL_INTERFACE("1f36c476-9546-536a-8381-2f9a2c08cedd")
                    ISysStorageProviderEventSource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_EventReceived(
                            __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_EventReceived(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISysStorageProviderEventSource = __uuidof(ISysStorageProviderEventSource);
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderHandlerFactory[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    MIDL_INTERFACE("ee798431-8213-5e89-a623-14d8c72b8a61")
                    ISysStorageProviderHandlerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetHttpRequestProvider(
                            HSTRING syncRootId,
                            ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderHttpRequestProvider** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetEventSource(
                            HSTRING syncRootId,
                            HSTRING eventName,
                            ABI::Windows::System::Implementation::FileExplorer::ISysStorageProviderEventSource** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISysStorageProviderHandlerFactory = __uuidof(ISysStorageProviderHandlerFactory);
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderHttpRequestProvider[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Implementation {
                namespace FileExplorer {
                    MIDL_INTERFACE("cb6fefb6-e76a-5c25-a33e-3e78a6e0e0ce")
                    ISysStorageProviderHttpRequestProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SendRequestAsync(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISysStorageProviderHttpRequestProvider = __uuidof(ISysStorageProviderHttpRequestProvider);
                } /* FileExplorer */
            } /* Implementation */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory interface starting with version 5.0 of the Windows.Storage.Provider.CloudFilesContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_System_Implementation_FileExplorer_SysStorageProviderEventReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Implementation_FileExplorer_SysStorageProviderEventReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Implementation_FileExplorer_SysStorageProviderEventReceivedEventArgs[] = L"Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
#endif
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory;

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource;

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory;

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider;

#endif // ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage;

typedef struct __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessageVtbl;

interface __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage* This,
        __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessageVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CWeb__CHttp__CHttpResponseMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs* This,
        __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* sender,
        __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderEventReceivedEventArgs[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Json)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_get_Json(This, value) \
    ((This)->lpVtbl->get_Json(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderEventReceivedEventArgsFactory[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory";
typedef struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory* This,
        HSTRING json,
        __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgs** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_CreateInstance(This, json, value) \
    ((This)->lpVtbl->CreateInstance(This, json, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventReceivedEventArgsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderEventSource[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
typedef struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_EventReceived)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This,
        __FITypedEventHandler_2_Windows__CSystem__CImplementation__CFileExplorer__CISysStorageProviderEventSource_Windows__CSystem__CImplementation__CFileExplorer__CSysStorageProviderEventReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EventReceived)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSourceVtbl;

interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_add_EventReceived(This, handler, token) \
    ((This)->lpVtbl->add_EventReceived(This, handler, token))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_remove_EventReceived(This, token) \
    ((This)->lpVtbl->remove_EventReceived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderHandlerFactory[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
typedef struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetHttpRequestProvider)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This,
        HSTRING syncRootId,
        __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider** result);
    HRESULT (STDMETHODCALLTYPE* GetEventSource)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory* This,
        HSTRING syncRootId,
        HSTRING eventName,
        __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderEventSource** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_GetHttpRequestProvider(This, syncRootId, result) \
    ((This)->lpVtbl->GetHttpRequestProvider(This, syncRootId, result))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_GetEventSource(This, syncRootId, eventName, result) \
    ((This)->lpVtbl->GetEventSource(This, syncRootId, eventName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHandlerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Implementation_FileExplorer_ISysStorageProviderHttpRequestProvider[] = L"Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
typedef struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SendRequestAsync)(__x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        __FIAsyncOperation_1_Windows__CWeb__CHttp__CHttpResponseMessage** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProviderVtbl;

interface __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_SendRequestAsync(This, request, operation) \
    ((This)->lpVtbl->SendRequestAsync(This, request, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider;
#endif /* !defined(____x_ABI_CWindows_CSystem_CImplementation_CFileExplorer_CISysStorageProviderHttpRequestProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs
 *
 * Introduced to Windows.Storage.Provider.CloudFilesContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory interface starting with version 5.0 of the Windows.Storage.Provider.CloudFilesContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_System_Implementation_FileExplorer_SysStorageProviderEventReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Implementation_FileExplorer_SysStorageProviderEventReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Implementation_FileExplorer_SysStorageProviderEventReceivedEventArgs[] = L"Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
#endif
#endif // WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION >= 0x50000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Eimplementation2Efileexplorer_p_h__

#endif // __windows2Esystem2Eimplementation2Efileexplorer_h__
