
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
#ifndef __windows2Esecurity2Ecryptography2Edataprotection_h__
#define __windows2Esecurity2Ecryptography2Edataprotection_h__
#ifndef __windows2Esecurity2Ecryptography2Edataprotection_p_h__
#define __windows2Esecurity2Ecryptography2Edataprotection_p_h__


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
#include "Windows.Storage.Streams.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace DataProtection {
                    interface IDataProtectionProvider;
                } /* DataProtection */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider ABI::Windows::Security::Cryptography::DataProtection::IDataProtectionProvider

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace DataProtection {
                    interface IDataProtectionProviderFactory;
                } /* DataProtection */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory ABI::Windows::Security::Cryptography::DataProtection::IDataProtectionProviderFactory

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace DataProtection {
                    class DataProtectionProvider;
                } /* DataProtection */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Security.Cryptography.DataProtection.IDataProtectionProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.DataProtection.DataProtectionProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_DataProtection_IDataProtectionProvider[] = L"Windows.Security.Cryptography.DataProtection.IDataProtectionProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace DataProtection {
                    MIDL_INTERFACE("09639948-ed22-4270-bd1c-6d72c00f8787")
                    IDataProtectionProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ProtectAsync(
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnprotectAsync(
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProtectStreamAsync(
                            ABI::Windows::Storage::Streams::IInputStream* src,
                            ABI::Windows::Storage::Streams::IOutputStream* dest,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnprotectStreamAsync(
                            ABI::Windows::Storage::Streams::IInputStream* src,
                            ABI::Windows::Storage::Streams::IOutputStream* dest,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDataProtectionProvider = __uuidof(IDataProtectionProvider);
                } /* DataProtection */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.DataProtection.DataProtectionProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_DataProtection_IDataProtectionProviderFactory[] = L"Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace DataProtection {
                    MIDL_INTERFACE("adf33dac-4932-4cdf-ac41-7214333514ca")
                    IDataProtectionProviderFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateOverloadExplicit(
                            HSTRING protectionDescriptor,
                            ABI::Windows::Security::Cryptography::DataProtection::IDataProtectionProvider** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDataProtectionProviderFactory = __uuidof(IDataProtectionProviderFactory);
                } /* DataProtection */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.DataProtection.DataProtectionProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.DataProtection.IDataProtectionProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_DataProtection_DataProtectionProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_DataProtection_DataProtectionProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_DataProtection_DataProtectionProvider[] = L"Windows.Security.Cryptography.DataProtection.DataProtectionProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

/*
 *
 * Interface Windows.Security.Cryptography.DataProtection.IDataProtectionProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.DataProtection.DataProtectionProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_DataProtection_IDataProtectionProvider[] = L"Windows.Security.Cryptography.DataProtection.IDataProtectionProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ProtectAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* UnprotectAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* ProtectStreamAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* src,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* dest,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* UnprotectStreamAsync)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* src,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* dest,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_ProtectAsync(This, data, value) \
    ((This)->lpVtbl->ProtectAsync(This, data, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_UnprotectAsync(This, data, value) \
    ((This)->lpVtbl->UnprotectAsync(This, data, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_ProtectStreamAsync(This, src, dest, value) \
    ((This)->lpVtbl->ProtectStreamAsync(This, src, dest, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_UnprotectStreamAsync(This, src, dest, value) \
    ((This)->lpVtbl->UnprotectStreamAsync(This, src, dest, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.DataProtection.DataProtectionProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_DataProtection_IDataProtectionProviderFactory[] = L"Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateOverloadExplicit)(__x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory* This,
        HSTRING protectionDescriptor,
        __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_CreateOverloadExplicit(This, protectionDescriptor, value) \
    ((This)->lpVtbl->CreateOverloadExplicit(This, protectionDescriptor, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CDataProtection_CIDataProtectionProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.DataProtection.DataProtectionProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Cryptography.DataProtection.IDataProtectionProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_DataProtection_DataProtectionProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_DataProtection_DataProtectionProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_DataProtection_DataProtectionProvider[] = L"Windows.Security.Cryptography.DataProtection.DataProtectionProvider";
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
#endif // __windows2Esecurity2Ecryptography2Edataprotection_p_h__

#endif // __windows2Esecurity2Ecryptography2Edataprotection_h__
