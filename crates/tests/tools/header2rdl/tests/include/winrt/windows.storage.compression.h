
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
#ifndef __windows2Estorage2Ecompression_h__
#define __windows2Estorage2Ecompression_h__
#ifndef __windows2Estorage2Ecompression_p_h__
#define __windows2Estorage2Ecompression_p_h__


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
#ifndef ____x_ABI_CWindows_CStorage_CCompression_CICompressor_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                interface ICompressor;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CCompression_CICompressor ABI::Windows::Storage::Compression::ICompressor

#endif // ____x_ABI_CWindows_CStorage_CCompression_CICompressor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                interface ICompressorFactory;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory ABI::Windows::Storage::Compression::ICompressorFactory

#endif // ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                interface IDecompressor;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor ABI::Windows::Storage::Compression::IDecompressor

#endif // ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                interface IDecompressorFactory;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory ABI::Windows::Storage::Compression::IDecompressorFactory

#endif // ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_FWD_DEFINED__

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
        namespace Storage {
            namespace Compression {
                typedef enum CompressAlgorithm : int CompressAlgorithm;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                class Compressor;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                class Decompressor;
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Storage.Compression.CompressAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                enum CompressAlgorithm : int
                {
                    CompressAlgorithm_InvalidAlgorithm = 0,
                    CompressAlgorithm_NullAlgorithm = 1,
                    CompressAlgorithm_Mszip = 2,
                    CompressAlgorithm_Xpress = 3,
                    CompressAlgorithm_XpressHuff = 4,
                    CompressAlgorithm_Lzms = 5,
                };
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.ICompressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Compressor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IOutputStream
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_ICompressor[] = L"Windows.Storage.Compression.ICompressor";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                MIDL_INTERFACE("0ac3645a-57ac-4ee1-b702-84d39d5424e0")
                ICompressor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FinishAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DetachStream(
                        ABI::Windows::Storage::Streams::IOutputStream** stream
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICompressor = __uuidof(ICompressor);
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CICompressor;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.ICompressorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Compressor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_ICompressorFactory[] = L"Windows.Storage.Compression.ICompressorFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                MIDL_INTERFACE("5f3d96a4-2cfb-442c-a8ba-d7d11b039da0")
                ICompressorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCompressor(
                        ABI::Windows::Storage::Streams::IOutputStream* underlyingStream,
                        ABI::Windows::Storage::Compression::ICompressor** createdCompressor
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCompressorEx(
                        ABI::Windows::Storage::Streams::IOutputStream* underlyingStream,
                        ABI::Windows::Storage::Compression::CompressAlgorithm algorithm,
                        UINT32 blockSize,
                        ABI::Windows::Storage::Compression::ICompressor** createdCompressor
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICompressorFactory = __uuidof(ICompressorFactory);
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CICompressorFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.IDecompressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Decompressor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_IDecompressor[] = L"Windows.Storage.Compression.IDecompressor";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                MIDL_INTERFACE("b883fe46-d68a-4c8b-ada0-4ee813fc5283")
                IDecompressor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DetachStream(
                        ABI::Windows::Storage::Streams::IInputStream** stream
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDecompressor = __uuidof(IDecompressor);
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CIDecompressor;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.IDecompressorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Decompressor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_IDecompressorFactory[] = L"Windows.Storage.Compression.IDecompressorFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Compression {
                MIDL_INTERFACE("5337e252-1da2-42e1-8834-0379d28d742f")
                IDecompressorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDecompressor(
                        ABI::Windows::Storage::Streams::IInputStream* underlyingStream,
                        ABI::Windows::Storage::Compression::IDecompressor** createdDecompressor
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDecompressorFactory = __uuidof(IDecompressorFactory);
            } /* Compression */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Compression.Compressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Compression.ICompressorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Compression.ICompressor ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Compression_Compressor_DEFINED
#define RUNTIMECLASS_Windows_Storage_Compression_Compressor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Compression_Compressor[] = L"Windows.Storage.Compression.Compressor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Compression.Decompressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Compression.IDecompressorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Compression.IDecompressor ** Default Interface **
 *    Windows.Storage.Streams.IInputStream
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Compression_Decompressor_DEFINED
#define RUNTIMECLASS_Windows_Storage_Compression_Decompressor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Compression_Decompressor[] = L"Windows.Storage.Compression.Decompressor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CCompression_CICompressor_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CCompression_CICompressor __x_ABI_CWindows_CStorage_CCompression_CICompressor;

#endif // ____x_ABI_CWindows_CStorage_CCompression_CICompressor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory;

#endif // ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CCompression_CIDecompressor __x_ABI_CWindows_CStorage_CCompression_CIDecompressor;

#endif // ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory;

#endif // ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CCompression_CCompressAlgorithm __x_ABI_CWindows_CStorage_CCompression_CCompressAlgorithm;

/*
 *
 * Struct Windows.Storage.Compression.CompressAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CCompression_CCompressAlgorithm
{
    CompressAlgorithm_InvalidAlgorithm = 0,
    CompressAlgorithm_NullAlgorithm = 1,
    CompressAlgorithm_Mszip = 2,
    CompressAlgorithm_Xpress = 3,
    CompressAlgorithm_XpressHuff = 4,
    CompressAlgorithm_Lzms = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.ICompressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Compressor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IOutputStream
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_ICompressor[] = L"Windows.Storage.Compression.ICompressor";
typedef struct __x_ABI_CWindows_CStorage_CCompression_CICompressorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FinishAsync)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* DetachStream)(__x_ABI_CWindows_CStorage_CCompression_CICompressor* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** stream);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CCompression_CICompressorVtbl;

interface __x_ABI_CWindows_CStorage_CCompression_CICompressor
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CCompression_CICompressorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_FinishAsync(This, operation) \
    ((This)->lpVtbl->FinishAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressor_DetachStream(This, stream) \
    ((This)->lpVtbl->DetachStream(This, stream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CICompressor;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.ICompressorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Compressor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_ICompressorFactory[] = L"Windows.Storage.Compression.ICompressorFactory";
typedef struct __x_ABI_CWindows_CStorage_CCompression_CICompressorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCompressor)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* underlyingStream,
        __x_ABI_CWindows_CStorage_CCompression_CICompressor** createdCompressor);
    HRESULT (STDMETHODCALLTYPE* CreateCompressorEx)(__x_ABI_CWindows_CStorage_CCompression_CICompressorFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* underlyingStream,
        enum __x_ABI_CWindows_CStorage_CCompression_CCompressAlgorithm algorithm,
        UINT32 blockSize,
        __x_ABI_CWindows_CStorage_CCompression_CICompressor** createdCompressor);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CCompression_CICompressorFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CCompression_CICompressorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_CreateCompressor(This, underlyingStream, createdCompressor) \
    ((This)->lpVtbl->CreateCompressor(This, underlyingStream, createdCompressor))

#define __x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_CreateCompressorEx(This, underlyingStream, algorithm, blockSize, createdCompressor) \
    ((This)->lpVtbl->CreateCompressorEx(This, underlyingStream, algorithm, blockSize, createdCompressor))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CICompressorFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CICompressorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.IDecompressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Decompressor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_IDecompressor[] = L"Windows.Storage.Compression.IDecompressor";
typedef struct __x_ABI_CWindows_CStorage_CCompression_CIDecompressorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DetachStream)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressor* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** stream);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CCompression_CIDecompressorVtbl;

interface __x_ABI_CWindows_CStorage_CCompression_CIDecompressor
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CCompression_CIDecompressorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressor_DetachStream(This, stream) \
    ((This)->lpVtbl->DetachStream(This, stream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CIDecompressor;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Compression.IDecompressorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Compression.Decompressor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Compression_IDecompressorFactory[] = L"Windows.Storage.Compression.IDecompressorFactory";
typedef struct __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDecompressor)(__x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* underlyingStream,
        __x_ABI_CWindows_CStorage_CCompression_CIDecompressor** createdDecompressor);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_CreateDecompressor(This, underlyingStream, createdDecompressor) \
    ((This)->lpVtbl->CreateDecompressor(This, underlyingStream, createdDecompressor))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CCompression_CIDecompressorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Compression.Compressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Compression.ICompressorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Compression.ICompressor ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Compression_Compressor_DEFINED
#define RUNTIMECLASS_Windows_Storage_Compression_Compressor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Compression_Compressor[] = L"Windows.Storage.Compression.Compressor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Compression.Decompressor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Compression.IDecompressorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Compression.IDecompressor ** Default Interface **
 *    Windows.Storage.Streams.IInputStream
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Compression_Decompressor_DEFINED
#define RUNTIMECLASS_Windows_Storage_Compression_Decompressor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Compression_Decompressor[] = L"Windows.Storage.Compression.Decompressor";
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
#endif // __windows2Estorage2Ecompression_p_h__

#endif // __windows2Estorage2Ecompression_h__
