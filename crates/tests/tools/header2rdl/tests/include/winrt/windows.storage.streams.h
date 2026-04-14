
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
#ifndef __windows2Estorage2Estreams_h__
#define __windows2Estorage2Estreams_h__
#ifndef __windows2Estorage2Estreams_p_h__
#define __windows2Estorage2Estreams_p_h__


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
#include "Windows.Storage.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBufferFactory;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory ABI::Windows::Storage::Streams::IBufferFactory

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBufferStatics;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics ABI::Windows::Storage::Streams::IBufferStatics

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IContentTypeProvider;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider ABI::Windows::Storage::Streams::IContentTypeProvider

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IDataReader;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader ABI::Windows::Storage::Streams::IDataReader

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IDataReaderFactory;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory ABI::Windows::Storage::Streams::IDataReaderFactory

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IDataReaderStatics;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics ABI::Windows::Storage::Streams::IDataReaderStatics

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IDataWriter;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter ABI::Windows::Storage::Streams::IDataWriter

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IDataWriterFactory;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory ABI::Windows::Storage::Streams::IDataWriterFactory

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IFileRandomAccessStreamStatics;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics ABI::Windows::Storage::Streams::IFileRandomAccessStreamStatics

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference ABI::Windows::Storage::Streams::IInputStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IPropertySetSerializer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer ABI::Windows::Storage::Streams::IPropertySetSerializer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReferenceStatics;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics ABI::Windows::Storage::Streams::IRandomAccessStreamReferenceStatics

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamStatics;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics ABI::Windows::Storage::Streams::IRandomAccessStreamStatics

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

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



#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageStreamTransaction;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageStreamTransaction;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction ABI::Windows::Storage::IStorageStreamTransaction

#endif // ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d81405a-9bd3-5e87-82f4-9b4128a887eb"))
IAsyncOperation<ABI::Windows::Storage::StorageStreamTransaction*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageStreamTransaction*, ABI::Windows::Storage::IStorageStreamTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageStreamTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageStreamTransaction*> __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d11739e6-2995-5d33-bfff-51b6041f68c1"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageStreamTransaction*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageStreamTransaction*, ABI::Windows::Storage::IStorageStreamTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageStreamTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageStreamTransaction*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8fe0732-556d-5841-b7ee-b3450fb52666"))
IAsyncOperation<ABI::Windows::Storage::Streams::IInputStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IInputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IInputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IInputStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d0bd0125-9049-57a3-bd66-e2525d98c814"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IInputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IInputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("430ecece-1418-5d19-81b2-5ddb381603cc"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("398c4183-793d-5b00-819b-4aef92485e94"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4a57c5e-32b0-55b3-ad13-ce1c23041ed6"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStreamWithContentType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3dddecf4-1d39-58e8-83b1-dbed541c7f35"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStreamWithContentType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1e466dc5-840f-54f9-b877-5e3a9f4b6c74"))
IAsyncOperationWithProgressCompletedHandler<UINT32, UINT32> : IAsyncOperationWithProgressCompletedHandler_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<UINT32, UINT32> __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eccb574a-c684-5572-a679-6b0842cfb57f"))
IAsyncOperationWithProgress<UINT32, UINT32> : IAsyncOperationWithProgress_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<UINT32, UINT32> __FIAsyncOperationWithProgress_2_UINT32_UINT32_t;
#define __FIAsyncOperationWithProgress_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ea0fe405-d432-5ac7-9ef8-5a65e1f97d7e"))
IAsyncOperationProgressHandler<UINT32, UINT32> : IAsyncOperationProgressHandler_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<UINT32, UINT32> __FIAsyncOperationProgressHandler_2_UINT32_UINT32_t;
#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d2024e41-5500-5b5a-ba46-cb7009596a2f"))
IAsyncOperationWithProgressCompletedHandler<UINT64, UINT64> : IAsyncOperationWithProgressCompletedHandler_impl<UINT64, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<UInt64, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<UINT64, UINT64> __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_UINT64_UINT64_USE
#define DEF___FIAsyncOperationWithProgress_2_UINT64_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8f1db6e3-6556-5516-825c-1021ee27cd0c"))
IAsyncOperationWithProgress<UINT64, UINT64> : IAsyncOperationWithProgress_impl<UINT64, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<UInt64, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<UINT64, UINT64> __FIAsyncOperationWithProgress_2_UINT64_UINT64_t;
#define __FIAsyncOperationWithProgress_2_UINT64_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_UINT64_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_UINT64_UINT64_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_UINT64_UINT64_USE
#define DEF___FIAsyncOperationProgressHandler_2_UINT64_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ffb2b65d-4120-5d13-826d-107851e6bb1c"))
IAsyncOperationProgressHandler<UINT64, UINT64> : IAsyncOperationProgressHandler_impl<UINT64, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<UInt64, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<UINT64, UINT64> __FIAsyncOperationProgressHandler_2_UINT64_UINT64_t;
#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_UINT64_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_UINT64_UINT64_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("06386a7a-e009-5b0b-ab68-a8e48b516647"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT32> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Storage.Streams.IBuffer, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT32> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d26b2819-897f-5c7d-84d6-56d796561431"))
IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IBuffer*, UINT32> : IAsyncOperationWithProgress_impl<ABI::Windows::Storage::Streams::IBuffer*, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Storage.Streams.IBuffer, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IBuffer*, UINT32> __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_t;
#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bf666554-7605-5d9a-b14e-18d8c8472afe"))
IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT32> : IAsyncOperationProgressHandler_impl<ABI::Windows::Storage::Streams::IBuffer*, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Storage.Streams.IBuffer, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT32> __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class MemoryBuffer;
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
        namespace Foundation {
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum FileAccessMode : int FileAccessMode;
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
        namespace Storage {
            typedef enum StorageOpenOptions : unsigned int StorageOpenOptions;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                typedef enum ByteOrder : int ByteOrder;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                typedef enum FileOpenDisposition : int FileOpenDisposition;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                typedef enum InputStreamOptions : unsigned int InputStreamOptions;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                typedef enum UnicodeEncoding : int UnicodeEncoding;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class Buffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class DataReader;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class DataReaderLoadOperation;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class DataWriter;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class DataWriterStoreOperation;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class RandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Storage.Streams.ByteOrder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                enum ByteOrder : int
                {
                    ByteOrder_LittleEndian = 0,
                    ByteOrder_BigEndian = 1,
                };
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Streams.FileOpenDisposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                enum FileOpenDisposition : int
                {
                    FileOpenDisposition_OpenExisting = 0,
                    FileOpenDisposition_OpenAlways = 1,
                    FileOpenDisposition_CreateNew = 2,
                    FileOpenDisposition_CreateAlways = 3,
                    FileOpenDisposition_TruncateExisting = 4,
                };
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Storage.Streams.InputStreamOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                enum InputStreamOptions : unsigned int
                {
                    InputStreamOptions_None = 0,
                    InputStreamOptions_Partial = 0x1,
                    InputStreamOptions_ReadAhead = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(InputStreamOptions)
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Streams.UnicodeEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                enum UnicodeEncoding : int
                {
                    UnicodeEncoding_Utf8 = 0,
                    UnicodeEncoding_Utf16LE = 1,
                    UnicodeEncoding_Utf16BE = 2,
                };
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IBuffer[] = L"Windows.Storage.Streams.IBuffer";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("905a0fe0-bc53-11df-8c49-001e4fc686da")
                IBuffer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Capacity(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Length(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Length(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBuffer = __uuidof(IBuffer);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIBuffer;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IBufferFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.Buffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IBufferFactory[] = L"Windows.Storage.Streams.IBufferFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("71af914d-c10f-484b-bc50-14bc623b3a27")
                IBufferFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        UINT32 capacity,
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBufferFactory = __uuidof(IBufferFactory);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIBufferFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IBufferStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.Buffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IBufferStatics[] = L"Windows.Storage.Streams.IBufferStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("e901e65b-d716-475a-a90a-af7229b1e741")
                IBufferStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCopyFromMemoryBuffer(
                        ABI::Windows::Foundation::IMemoryBuffer* input,
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMemoryBufferOverIBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* input,
                        ABI::Windows::Foundation::IMemoryBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBufferStatics = __uuidof(IBufferStatics);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIBufferStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IContentTypeProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IContentTypeProvider[] = L"Windows.Storage.Streams.IContentTypeProvider";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("97d098a5-3b99-4de9-88a5-e11d2f50c795")
                IContentTypeProvider : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContentTypeProvider = __uuidof(IContentTypeProvider);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataReader[] = L"Windows.Storage.Streams.IDataReader";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("e2b50029-b4c1-4314-a4b8-fb813a2f275e")
                IDataReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UnconsumedBufferLength(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UnicodeEncoding(
                        ABI::Windows::Storage::Streams::UnicodeEncoding* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UnicodeEncoding(
                        ABI::Windows::Storage::Streams::UnicodeEncoding value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ByteOrder(
                        ABI::Windows::Storage::Streams::ByteOrder* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ByteOrder(
                        ABI::Windows::Storage::Streams::ByteOrder value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputStreamOptions(
                        ABI::Windows::Storage::Streams::InputStreamOptions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InputStreamOptions(
                        ABI::Windows::Storage::Streams::InputStreamOptions value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadByte(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadBytes(
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadBuffer(
                        UINT32 length,
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadBoolean(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadGuid(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadInt16(
                        INT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadInt32(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadInt64(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadUInt16(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadUInt32(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadUInt64(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadSingle(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadDouble(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadString(
                        UINT32 codeUnitCount,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadDateTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadTimeSpan(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadAsync(
                        UINT32 count,
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DetachBuffer(
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DetachStream(
                        ABI::Windows::Storage::Streams::IInputStream** stream
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataReader = __uuidof(IDataReader);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataReader;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataReaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.DataReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataReaderFactory[] = L"Windows.Storage.Streams.IDataReaderFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("d7527847-57da-4e15-914c-06806699a098")
                IDataReaderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDataReader(
                        ABI::Windows::Storage::Streams::IInputStream* inputStream,
                        ABI::Windows::Storage::Streams::IDataReader** dataReader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataReaderFactory = __uuidof(IDataReaderFactory);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataReaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.DataReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataReaderStatics[] = L"Windows.Storage.Streams.IDataReaderStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("11fcbfc8-f93a-471b-b121-f379e349313c")
                IDataReaderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        ABI::Windows::Storage::Streams::IDataReader** dataReader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataReaderStatics = __uuidof(IDataReaderStatics);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataWriter[] = L"Windows.Storage.Streams.IDataWriter";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("64b89265-d341-4922-b38a-dd4af8808c4e")
                IDataWriter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UnstoredBufferLength(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UnicodeEncoding(
                        ABI::Windows::Storage::Streams::UnicodeEncoding* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UnicodeEncoding(
                        ABI::Windows::Storage::Streams::UnicodeEncoding value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ByteOrder(
                        ABI::Windows::Storage::Streams::ByteOrder* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ByteOrder(
                        ABI::Windows::Storage::Streams::ByteOrder value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteByte(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteBytes(
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteBufferRange(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        UINT32 start,
                        UINT32 count
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteBoolean(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteGuid(
                        GUID value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteInt16(
                        INT16 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteInt32(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteInt64(
                        INT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteUInt16(
                        UINT16 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteUInt32(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteUInt64(
                        UINT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteSingle(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteDouble(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteDateTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteTimeSpan(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteString(
                        HSTRING value,
                        UINT32* codeUnitCount
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MeasureString(
                        HSTRING value,
                        UINT32* codeUnitCount
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StoreAsync(
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DetachBuffer(
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DetachStream(
                        ABI::Windows::Storage::Streams::IOutputStream** outputStream
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataWriter = __uuidof(IDataWriter);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataWriter;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataWriterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.DataWriter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataWriterFactory[] = L"Windows.Storage.Streams.IDataWriterFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("338c67c2-8b84-4c2b-9c50-7b8767847a1f")
                IDataWriterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDataWriter(
                        ABI::Windows::Storage::Streams::IOutputStream* outputStream,
                        ABI::Windows::Storage::Streams::IDataWriter** dataWriter
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataWriterFactory = __uuidof(IDataWriterFactory);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IFileRandomAccessStreamStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.FileRandomAccessStream
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IFileRandomAccessStreamStatics[] = L"Windows.Storage.Streams.IFileRandomAccessStreamStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("73550107-3b57-4b5d-8345-554d2fc621f0")
                IFileRandomAccessStreamStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE OpenAsync(
                        HSTRING filePath,
                        ABI::Windows::Storage::FileAccessMode accessMode,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenWithOptionsAsync(
                        HSTRING filePath,
                        ABI::Windows::Storage::FileAccessMode accessMode,
                        ABI::Windows::Storage::StorageOpenOptions sharingOptions,
                        ABI::Windows::Storage::Streams::FileOpenDisposition openDisposition,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenTransactedWriteAsync(
                        HSTRING filePath,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenTransactedWriteWithOptionsAsync(
                        HSTRING filePath,
                        ABI::Windows::Storage::StorageOpenOptions openOptions,
                        ABI::Windows::Storage::Streams::FileOpenDisposition openDisposition,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenForUserAsync(
                        ABI::Windows::System::IUser* user,
                        HSTRING filePath,
                        ABI::Windows::Storage::FileAccessMode accessMode,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenForUserWithOptionsAsync(
                        ABI::Windows::System::IUser* user,
                        HSTRING filePath,
                        ABI::Windows::Storage::FileAccessMode accessMode,
                        ABI::Windows::Storage::StorageOpenOptions sharingOptions,
                        ABI::Windows::Storage::Streams::FileOpenDisposition openDisposition,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenTransactedWriteForUserAsync(
                        ABI::Windows::System::IUser* user,
                        HSTRING filePath,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenTransactedWriteForUserWithOptionsAsync(
                        ABI::Windows::System::IUser* user,
                        HSTRING filePath,
                        ABI::Windows::Storage::StorageOpenOptions openOptions,
                        ABI::Windows::Storage::Streams::FileOpenDisposition openDisposition,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileRandomAccessStreamStatics = __uuidof(IFileRandomAccessStreamStatics);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.Streams.IInputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IInputStream[] = L"Windows.Storage.Streams.IInputStream";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("905a0fe2-bc53-11df-8c49-001e4fc686da")
                IInputStream : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadAsync(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        UINT32 count,
                        ABI::Windows::Storage::Streams::InputStreamOptions options,
                        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputStream = __uuidof(IInputStream);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIInputStream;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IInputStreamReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IInputStreamReference[] = L"Windows.Storage.Streams.IInputStreamReference";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("43929d18-5ec9-4b5a-919c-4205b0c804b6")
                IInputStreamReference : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE OpenSequentialReadAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputStreamReference = __uuidof(IInputStreamReference);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IOutputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IOutputStream[] = L"Windows.Storage.Streams.IOutputStream";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("905a0fe6-bc53-11df-8c49-001e4fc686da")
                IOutputStream : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE WriteAsync(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        __FIAsyncOperationWithProgress_2_UINT32_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOutputStream = __uuidof(IOutputStream);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIOutputStream;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IPropertySetSerializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IPropertySetSerializer[] = L"Windows.Storage.Streams.IPropertySetSerializer";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("6e8ebf1c-ef3d-4376-b20e-5be638aeac77")
                IPropertySetSerializer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Serialize(
                        ABI::Windows::Foundation::Collections::IPropertySet* propertySet,
                        ABI::Windows::Storage::Streams::IBuffer** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Deserialize(
                        ABI::Windows::Foundation::Collections::IPropertySet* propertySet,
                        ABI::Windows::Storage::Streams::IBuffer* buffer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPropertySetSerializer = __uuidof(IPropertySetSerializer);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Storage.Streams.IOutputStream
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStream[] = L"Windows.Storage.Streams.IRandomAccessStream";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("905a0fe1-bc53-11df-8c49-001e4fc686da")
                IRandomAccessStream : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Size(
                        UINT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInputStreamAt(
                        UINT64 position,
                        ABI::Windows::Storage::Streams::IInputStream** stream
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOutputStreamAt(
                        UINT64 position,
                        ABI::Windows::Storage::Streams::IOutputStream** stream
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Seek(
                        UINT64 position
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloneStream(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** stream
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanRead(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanWrite(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRandomAccessStream = __uuidof(IRandomAccessStream);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamReference[] = L"Windows.Storage.Streams.IRandomAccessStreamReference";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("33ee3134-1dd6-4e3a-8067-d1c162e8642b")
                IRandomAccessStreamReference : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE OpenReadAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRandomAccessStreamReference = __uuidof(IRandomAccessStreamReference);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamReferenceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.RandomAccessStreamReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamReferenceStatics[] = L"Windows.Storage.Streams.IRandomAccessStreamReferenceStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("857309dc-3fbf-4e7d-986f-ef3b1a07a964")
                IRandomAccessStreamReferenceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFile(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** streamReference
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** streamReference
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromStream(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** streamReference
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRandomAccessStreamReferenceStatics = __uuidof(IRandomAccessStreamReferenceStatics);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.RandomAccessStream
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamStatics[] = L"Windows.Storage.Streams.IRandomAccessStreamStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("524cedcf-6e29-4ce5-9573-6b753db66c3a")
                IRandomAccessStreamStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CopyAsync(
                        ABI::Windows::Storage::Streams::IInputStream* source,
                        ABI::Windows::Storage::Streams::IOutputStream* destination,
                        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CopySizeAsync(
                        ABI::Windows::Storage::Streams::IInputStream* source,
                        ABI::Windows::Storage::Streams::IOutputStream* destination,
                        UINT64 bytesToCopy,
                        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CopyAndCloseAsync(
                        ABI::Windows::Storage::Streams::IInputStream* source,
                        ABI::Windows::Storage::Streams::IOutputStream* destination,
                        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRandomAccessStreamStatics = __uuidof(IRandomAccessStreamStatics);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamWithContentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IRandomAccessStream
 *     Windows.Foundation.IClosable
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Storage.Streams.IOutputStream
 *     Windows.Storage.Streams.IContentTypeProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamWithContentType[] = L"Windows.Storage.Streams.IRandomAccessStreamWithContentType";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                MIDL_INTERFACE("cc254827-4b3d-438f-9232-10c76bc7e038")
                IRandomAccessStreamWithContentType : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IRandomAccessStreamWithContentType = __uuidof(IRandomAccessStreamWithContentType);
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.Buffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Streams.IBufferFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IBufferStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IBuffer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_Buffer_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_Buffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_Buffer[] = L"Windows.Storage.Streams.Buffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Streams.IDataReaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IDataReaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IDataReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataReader_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataReader[] = L"Windows.Storage.Streams.DataReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataReaderLoadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<UInt32> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataReaderLoadOperation_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataReaderLoadOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataReaderLoadOperation[] = L"Windows.Storage.Streams.DataReaderLoadOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Streams.IDataWriterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IDataWriter ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataWriter_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataWriter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataWriter[] = L"Windows.Storage.Streams.DataWriter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataWriterStoreOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<UInt32> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataWriterStoreOperation_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataWriterStoreOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataWriterStoreOperation[] = L"Windows.Storage.Streams.DataWriterStoreOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.FileInputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IInputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_FileInputStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_FileInputStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_FileInputStream[] = L"Windows.Storage.Streams.FileInputStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.FileOutputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IOutputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_FileOutputStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_FileOutputStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_FileOutputStream[] = L"Windows.Storage.Streams.FileOutputStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.FileRandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IFileRandomAccessStreamStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStream ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_FileRandomAccessStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_FileRandomAccessStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_FileRandomAccessStream[] = L"Windows.Storage.Streams.FileRandomAccessStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.InMemoryRandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStream ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_InMemoryRandomAccessStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_InMemoryRandomAccessStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_InMemoryRandomAccessStream[] = L"Windows.Storage.Streams.InMemoryRandomAccessStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.InputStreamOverStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IInputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_InputStreamOverStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_InputStreamOverStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_InputStreamOverStream[] = L"Windows.Storage.Streams.InputStreamOverStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.OutputStreamOverStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IOutputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_OutputStreamOverStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_OutputStreamOverStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_OutputStreamOverStream[] = L"Windows.Storage.Streams.OutputStreamOverStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.RandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IRandomAccessStreamStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_RandomAccessStream[] = L"Windows.Storage.Streams.RandomAccessStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.RandomAccessStreamOverStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStream ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamOverStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamOverStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_RandomAccessStreamOverStream[] = L"Windows.Storage.Streams.RandomAccessStreamOverStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.RandomAccessStreamReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IRandomAccessStreamReferenceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamReference ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamReference_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_RandomAccessStreamReference[] = L"Windows.Storage.Streams.RandomAccessStreamReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIDataReader __x_ABI_CWindows_CStorage_CStreams_CIDataReader;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIDataWriter __x_ABI_CWindows_CStorage_CStreams_CIDataWriter;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageStreamTransaction __x_ABI_CWindows_CStorage_CIStorageStreamTransaction;

#endif // ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        __x_ABI_CWindows_CStorage_CIStorageStreamTransaction** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransactionVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransactionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32 __FIAsyncOperationProgressHandler_2_UINT32_UINT32;

typedef interface __FIAsyncOperationWithProgress_2_UINT32_UINT32 __FIAsyncOperationWithProgress_2_UINT32_UINT32;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32 __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_UINT32_UINT32 __FIAsyncOperationWithProgress_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_UINT32_UINT32;

typedef struct __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationProgressHandler_2_UINT32_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationProgressHandler_2_UINT32_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationWithProgress_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32 __FIAsyncOperationProgressHandler_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_UINT32_UINT32;

typedef struct __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32* asyncInfo,
        UINT32 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_UINT64_UINT64 __FIAsyncOperationProgressHandler_2_UINT64_UINT64;

typedef interface __FIAsyncOperationWithProgress_2_UINT64_UINT64 __FIAsyncOperationWithProgress_2_UINT64_UINT64;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64 __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_UINT64_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_UINT64_UINT64 __FIAsyncOperationWithProgress_2_UINT64_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_UINT64_UINT64;

typedef struct __FIAsyncOperationWithProgress_2_UINT64_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationProgressHandler_2_UINT64_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationProgressHandler_2_UINT64_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        UINT64* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_UINT64_UINT64Vtbl;

interface __FIAsyncOperationWithProgress_2_UINT64_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_UINT64_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_UINT64_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_UINT64_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_UINT64_UINT64 __FIAsyncOperationProgressHandler_2_UINT64_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_UINT64_UINT64;

typedef struct __FIAsyncOperationProgressHandler_2_UINT64_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64* asyncInfo,
        UINT64 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_UINT64_UINT64Vtbl;

interface __FIAsyncOperationProgressHandler_2_UINT64_UINT64
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_UINT64_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32 __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32 __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32 __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32 __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32 __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32* asyncInfo,
        UINT32 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CFileAccessMode __x_ABI_CWindows_CStorage_CFileAccessMode;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CStorageOpenOptions __x_ABI_CWindows_CStorage_CStorageOpenOptions;

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CStreams_CByteOrder __x_ABI_CWindows_CStorage_CStreams_CByteOrder;

typedef enum __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition;

typedef enum __x_ABI_CWindows_CStorage_CStreams_CInputStreamOptions __x_ABI_CWindows_CStorage_CStreams_CInputStreamOptions;

typedef enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding;

/*
 *
 * Struct Windows.Storage.Streams.ByteOrder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStreams_CByteOrder
{
    ByteOrder_LittleEndian = 0,
    ByteOrder_BigEndian = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Streams.FileOpenDisposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition
{
    FileOpenDisposition_OpenExisting = 0,
    FileOpenDisposition_OpenAlways = 1,
    FileOpenDisposition_CreateNew = 2,
    FileOpenDisposition_CreateAlways = 3,
    FileOpenDisposition_TruncateExisting = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Storage.Streams.InputStreamOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStreams_CInputStreamOptions
{
    InputStreamOptions_None = 0,
    InputStreamOptions_Partial = 0x1,
    InputStreamOptions_ReadAhead = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Streams.UnicodeEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding
{
    UnicodeEncoding_Utf8 = 0,
    UnicodeEncoding_Utf16LE = 1,
    UnicodeEncoding_Utf16BE = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IBuffer[] = L"Windows.Storage.Streams.IBuffer";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capacity)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Length)(__x_ABI_CWindows_CStorage_CStreams_CIBuffer* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIBufferVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_get_Capacity(This, value) \
    ((This)->lpVtbl->get_Capacity(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer_put_Length(This, value) \
    ((This)->lpVtbl->put_Length(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIBuffer;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IBufferFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.Buffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IBufferFactory[] = L"Windows.Storage.Streams.IBufferFactory";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIBufferFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CStorage_CStreams_CIBufferFactory* This,
        UINT32 capacity,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIBufferFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIBufferFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_Create(This, capacity, value) \
    ((This)->lpVtbl->Create(This, capacity, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIBufferFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IBufferStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.Buffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IBufferStatics[] = L"Windows.Storage.Streams.IBufferStatics";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIBufferStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCopyFromMemoryBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This,
        __x_ABI_CWindows_CFoundation_CIMemoryBuffer* input,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* CreateMemoryBufferOverIBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIBufferStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* input,
        __x_ABI_CWindows_CFoundation_CIMemoryBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIBufferStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIBufferStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_CreateCopyFromMemoryBuffer(This, input, value) \
    ((This)->lpVtbl->CreateCopyFromMemoryBuffer(This, input, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_CreateMemoryBufferOverIBuffer(This, input, value) \
    ((This)->lpVtbl->CreateMemoryBufferOverIBuffer(This, input, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIBufferStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIBufferStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IContentTypeProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IContentTypeProvider[] = L"Windows.Storage.Streams.IContentTypeProvider";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentType)(__x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProviderVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_get_ContentType(This, value) \
    ((This)->lpVtbl->get_ContentType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataReader[] = L"Windows.Storage.Streams.IDataReader";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIDataReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnconsumedBufferLength)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UnicodeEncoding)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding* value);
    HRESULT (STDMETHODCALLTYPE* put_UnicodeEncoding)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding value);
    HRESULT (STDMETHODCALLTYPE* get_ByteOrder)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CByteOrder* value);
    HRESULT (STDMETHODCALLTYPE* put_ByteOrder)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CByteOrder value);
    HRESULT (STDMETHODCALLTYPE* get_InputStreamOptions)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CInputStreamOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_InputStreamOptions)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CInputStreamOptions value);
    HRESULT (STDMETHODCALLTYPE* ReadByte)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* ReadBytes)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* ReadBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT32 length,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* ReadBoolean)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ReadGuid)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* ReadInt16)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        INT16* value);
    HRESULT (STDMETHODCALLTYPE* ReadInt32)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* ReadInt64)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* ReadUInt16)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* ReadUInt32)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* ReadUInt64)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* ReadSingle)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* ReadDouble)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* ReadString)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT32 codeUnitCount,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReadDateTime)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* ReadTimeSpan)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* LoadAsync)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        UINT32 count,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* DetachBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* DetachStream)(__x_ABI_CWindows_CStorage_CStreams_CIDataReader* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** stream);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIDataReaderVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIDataReader
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIDataReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_get_UnconsumedBufferLength(This, value) \
    ((This)->lpVtbl->get_UnconsumedBufferLength(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_get_UnicodeEncoding(This, value) \
    ((This)->lpVtbl->get_UnicodeEncoding(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_put_UnicodeEncoding(This, value) \
    ((This)->lpVtbl->put_UnicodeEncoding(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_get_ByteOrder(This, value) \
    ((This)->lpVtbl->get_ByteOrder(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_put_ByteOrder(This, value) \
    ((This)->lpVtbl->put_ByteOrder(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_get_InputStreamOptions(This, value) \
    ((This)->lpVtbl->get_InputStreamOptions(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_put_InputStreamOptions(This, value) \
    ((This)->lpVtbl->put_InputStreamOptions(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadByte(This, value) \
    ((This)->lpVtbl->ReadByte(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadBytes(This, valueLength, value) \
    ((This)->lpVtbl->ReadBytes(This, valueLength, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadBuffer(This, length, buffer) \
    ((This)->lpVtbl->ReadBuffer(This, length, buffer))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadBoolean(This, value) \
    ((This)->lpVtbl->ReadBoolean(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadGuid(This, value) \
    ((This)->lpVtbl->ReadGuid(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadInt16(This, value) \
    ((This)->lpVtbl->ReadInt16(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadInt32(This, value) \
    ((This)->lpVtbl->ReadInt32(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadInt64(This, value) \
    ((This)->lpVtbl->ReadInt64(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadUInt16(This, value) \
    ((This)->lpVtbl->ReadUInt16(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadUInt32(This, value) \
    ((This)->lpVtbl->ReadUInt32(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadUInt64(This, value) \
    ((This)->lpVtbl->ReadUInt64(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadSingle(This, value) \
    ((This)->lpVtbl->ReadSingle(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadDouble(This, value) \
    ((This)->lpVtbl->ReadDouble(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadString(This, codeUnitCount, value) \
    ((This)->lpVtbl->ReadString(This, codeUnitCount, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadDateTime(This, value) \
    ((This)->lpVtbl->ReadDateTime(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_ReadTimeSpan(This, value) \
    ((This)->lpVtbl->ReadTimeSpan(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_LoadAsync(This, count, operation) \
    ((This)->lpVtbl->LoadAsync(This, count, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_DetachBuffer(This, buffer) \
    ((This)->lpVtbl->DetachBuffer(This, buffer))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReader_DetachStream(This, stream) \
    ((This)->lpVtbl->DetachStream(This, stream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataReader;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataReaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.DataReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataReaderFactory[] = L"Windows.Storage.Streams.IDataReaderFactory";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDataReader)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* inputStream,
        __x_ABI_CWindows_CStorage_CStreams_CIDataReader** dataReader);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_CreateDataReader(This, inputStream, dataReader) \
    ((This)->lpVtbl->CreateDataReader(This, inputStream, dataReader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataReaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.DataReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataReaderStatics[] = L"Windows.Storage.Streams.IDataReaderStatics";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CStorage_CStreams_CIDataReader** dataReader);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_FromBuffer(This, buffer, dataReader) \
    ((This)->lpVtbl->FromBuffer(This, buffer, dataReader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataReaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataWriter[] = L"Windows.Storage.Streams.IDataWriter";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIDataWriterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnstoredBufferLength)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UnicodeEncoding)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding* value);
    HRESULT (STDMETHODCALLTYPE* put_UnicodeEncoding)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding value);
    HRESULT (STDMETHODCALLTYPE* get_ByteOrder)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CByteOrder* value);
    HRESULT (STDMETHODCALLTYPE* put_ByteOrder)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        enum __x_ABI_CWindows_CStorage_CStreams_CByteOrder value);
    HRESULT (STDMETHODCALLTYPE* WriteByte)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* WriteBytes)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* WriteBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer);
    HRESULT (STDMETHODCALLTYPE* WriteBufferRange)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        UINT32 start,
        UINT32 count);
    HRESULT (STDMETHODCALLTYPE* WriteBoolean)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* WriteGuid)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* WriteInt16)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        INT16 value);
    HRESULT (STDMETHODCALLTYPE* WriteInt32)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* WriteInt64)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        INT64 value);
    HRESULT (STDMETHODCALLTYPE* WriteUInt16)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* WriteUInt32)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* WriteUInt64)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* WriteSingle)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* WriteDouble)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* WriteDateTime)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* WriteTimeSpan)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* WriteString)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        HSTRING value,
        UINT32* codeUnitCount);
    HRESULT (STDMETHODCALLTYPE* MeasureString)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        HSTRING value,
        UINT32* codeUnitCount);
    HRESULT (STDMETHODCALLTYPE* StoreAsync)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* FlushAsync)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* DetachBuffer)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* DetachStream)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriter* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** outputStream);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIDataWriterVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIDataWriter
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIDataWriterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_get_UnstoredBufferLength(This, value) \
    ((This)->lpVtbl->get_UnstoredBufferLength(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_get_UnicodeEncoding(This, value) \
    ((This)->lpVtbl->get_UnicodeEncoding(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_put_UnicodeEncoding(This, value) \
    ((This)->lpVtbl->put_UnicodeEncoding(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_get_ByteOrder(This, value) \
    ((This)->lpVtbl->get_ByteOrder(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_put_ByteOrder(This, value) \
    ((This)->lpVtbl->put_ByteOrder(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteByte(This, value) \
    ((This)->lpVtbl->WriteByte(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteBytes(This, valueLength, value) \
    ((This)->lpVtbl->WriteBytes(This, valueLength, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteBuffer(This, buffer) \
    ((This)->lpVtbl->WriteBuffer(This, buffer))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteBufferRange(This, buffer, start, count) \
    ((This)->lpVtbl->WriteBufferRange(This, buffer, start, count))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteBoolean(This, value) \
    ((This)->lpVtbl->WriteBoolean(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteGuid(This, value) \
    ((This)->lpVtbl->WriteGuid(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteInt16(This, value) \
    ((This)->lpVtbl->WriteInt16(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteInt32(This, value) \
    ((This)->lpVtbl->WriteInt32(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteInt64(This, value) \
    ((This)->lpVtbl->WriteInt64(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteUInt16(This, value) \
    ((This)->lpVtbl->WriteUInt16(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteUInt32(This, value) \
    ((This)->lpVtbl->WriteUInt32(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteUInt64(This, value) \
    ((This)->lpVtbl->WriteUInt64(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteSingle(This, value) \
    ((This)->lpVtbl->WriteSingle(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteDouble(This, value) \
    ((This)->lpVtbl->WriteDouble(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteDateTime(This, value) \
    ((This)->lpVtbl->WriteDateTime(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteTimeSpan(This, value) \
    ((This)->lpVtbl->WriteTimeSpan(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_WriteString(This, value, codeUnitCount) \
    ((This)->lpVtbl->WriteString(This, value, codeUnitCount))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_MeasureString(This, value, codeUnitCount) \
    ((This)->lpVtbl->MeasureString(This, value, codeUnitCount))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_StoreAsync(This, operation) \
    ((This)->lpVtbl->StoreAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_FlushAsync(This, operation) \
    ((This)->lpVtbl->FlushAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_DetachBuffer(This, buffer) \
    ((This)->lpVtbl->DetachBuffer(This, buffer))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriter_DetachStream(This, outputStream) \
    ((This)->lpVtbl->DetachStream(This, outputStream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataWriter;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IDataWriterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.DataWriter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IDataWriterFactory[] = L"Windows.Storage.Streams.IDataWriterFactory";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDataWriter)(__x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* outputStream,
        __x_ABI_CWindows_CStorage_CStreams_CIDataWriter** dataWriter);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_CreateDataWriter(This, outputStream, dataWriter) \
    ((This)->lpVtbl->CreateDataWriter(This, outputStream, dataWriter))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIDataWriterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IFileRandomAccessStreamStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.FileRandomAccessStream
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IFileRandomAccessStreamStatics[] = L"Windows.Storage.Streams.IFileRandomAccessStreamStatics";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        HSTRING filePath,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* OpenWithOptionsAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        HSTRING filePath,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        enum __x_ABI_CWindows_CStorage_CStorageOpenOptions sharingOptions,
        enum __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition openDisposition,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* OpenTransactedWriteAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        HSTRING filePath,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation);
    HRESULT (STDMETHODCALLTYPE* OpenTransactedWriteWithOptionsAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        HSTRING filePath,
        enum __x_ABI_CWindows_CStorage_CStorageOpenOptions openOptions,
        enum __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition openDisposition,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation);
    HRESULT (STDMETHODCALLTYPE* OpenForUserAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING filePath,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* OpenForUserWithOptionsAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING filePath,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        enum __x_ABI_CWindows_CStorage_CStorageOpenOptions sharingOptions,
        enum __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition openDisposition,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* OpenTransactedWriteForUserAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING filePath,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation);
    HRESULT (STDMETHODCALLTYPE* OpenTransactedWriteForUserWithOptionsAsync)(__x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING filePath,
        enum __x_ABI_CWindows_CStorage_CStorageOpenOptions openOptions,
        enum __x_ABI_CWindows_CStorage_CStreams_CFileOpenDisposition openDisposition,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenAsync(This, filePath, accessMode, operation) \
    ((This)->lpVtbl->OpenAsync(This, filePath, accessMode, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenWithOptionsAsync(This, filePath, accessMode, sharingOptions, openDisposition, operation) \
    ((This)->lpVtbl->OpenWithOptionsAsync(This, filePath, accessMode, sharingOptions, openDisposition, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenTransactedWriteAsync(This, filePath, operation) \
    ((This)->lpVtbl->OpenTransactedWriteAsync(This, filePath, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenTransactedWriteWithOptionsAsync(This, filePath, openOptions, openDisposition, operation) \
    ((This)->lpVtbl->OpenTransactedWriteWithOptionsAsync(This, filePath, openOptions, openDisposition, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenForUserAsync(This, user, filePath, accessMode, operation) \
    ((This)->lpVtbl->OpenForUserAsync(This, user, filePath, accessMode, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenForUserWithOptionsAsync(This, user, filePath, accessMode, sharingOptions, openDisposition, operation) \
    ((This)->lpVtbl->OpenForUserWithOptionsAsync(This, user, filePath, accessMode, sharingOptions, openDisposition, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenTransactedWriteForUserAsync(This, user, filePath, operation) \
    ((This)->lpVtbl->OpenTransactedWriteForUserAsync(This, user, filePath, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_OpenTransactedWriteForUserWithOptionsAsync(This, user, filePath, openOptions, openDisposition, operation) \
    ((This)->lpVtbl->OpenTransactedWriteForUserWithOptionsAsync(This, user, filePath, openOptions, openDisposition, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIFileRandomAccessStreamStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.Streams.IInputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IInputStream[] = L"Windows.Storage.Streams.IInputStream";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadAsync)(__x_ABI_CWindows_CStorage_CStreams_CIInputStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        UINT32 count,
        enum __x_ABI_CWindows_CStorage_CStreams_CInputStreamOptions options,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT32** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIInputStreamVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream_ReadAsync(This, buffer, count, options, operation) \
    ((This)->lpVtbl->ReadAsync(This, buffer, count, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIInputStream;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IInputStreamReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IInputStreamReference[] = L"Windows.Storage.Streams.IInputStreamReference";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenSequentialReadAsync)(__x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReferenceVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_OpenSequentialReadAsync(This, operation) \
    ((This)->lpVtbl->OpenSequentialReadAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IOutputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IOutputStream[] = L"Windows.Storage.Streams.IOutputStream";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIOutputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* WriteAsync)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* FlushAsync)(__x_ABI_CWindows_CStorage_CStreams_CIOutputStream* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIOutputStreamVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIOutputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_WriteAsync(This, buffer, operation) \
    ((This)->lpVtbl->WriteAsync(This, buffer, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FlushAsync(This, operation) \
    ((This)->lpVtbl->FlushAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIOutputStream;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IPropertySetSerializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IPropertySetSerializer[] = L"Windows.Storage.Streams.IPropertySetSerializer";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Serialize)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* propertySet,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* Deserialize)(__x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* propertySet,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializerVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_Serialize(This, propertySet, result) \
    ((This)->lpVtbl->Serialize(This, propertySet, result))

#define __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_Deserialize(This, propertySet, buffer) \
    ((This)->lpVtbl->Deserialize(This, propertySet, buffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Storage.Streams.IOutputStream
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStream[] = L"Windows.Storage.Streams.IRandomAccessStream";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* put_Size)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* GetInputStreamAt)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        UINT64 position,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** stream);
    HRESULT (STDMETHODCALLTYPE* GetOutputStreamAt)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        UINT64 position,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** stream);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* Seek)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        UINT64 position);
    HRESULT (STDMETHODCALLTYPE* CloneStream)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** stream);
    HRESULT (STDMETHODCALLTYPE* get_CanRead)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CanWrite)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_put_Size(This, value) \
    ((This)->lpVtbl->put_Size(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_GetInputStreamAt(This, position, stream) \
    ((This)->lpVtbl->GetInputStreamAt(This, position, stream))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_GetOutputStreamAt(This, position, stream) \
    ((This)->lpVtbl->GetOutputStreamAt(This, position, stream))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_Seek(This, position) \
    ((This)->lpVtbl->Seek(This, position))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_CloneStream(This, stream) \
    ((This)->lpVtbl->CloneStream(This, stream))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_get_CanRead(This, value) \
    ((This)->lpVtbl->get_CanRead(This, value))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_get_CanWrite(This, value) \
    ((This)->lpVtbl->get_CanWrite(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamReference[] = L"Windows.Storage.Streams.IRandomAccessStreamReference";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenReadAsync)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_OpenReadAsync(This, operation) \
    ((This)->lpVtbl->OpenReadAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamReferenceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.RandomAccessStreamReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamReferenceStatics[] = L"Windows.Storage.Streams.IRandomAccessStreamReferenceStatics";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromFile)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** streamReference);
    HRESULT (STDMETHODCALLTYPE* CreateFromUri)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** streamReference);
    HRESULT (STDMETHODCALLTYPE* CreateFromStream)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** streamReference);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_CreateFromFile(This, file, streamReference) \
    ((This)->lpVtbl->CreateFromFile(This, file, streamReference))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_CreateFromUri(This, uri, streamReference) \
    ((This)->lpVtbl->CreateFromUri(This, uri, streamReference))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_CreateFromStream(This, stream, streamReference) \
    ((This)->lpVtbl->CreateFromStream(This, stream, streamReference))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReferenceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Streams.RandomAccessStream
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamStatics[] = L"Windows.Storage.Streams.IRandomAccessStreamStatics";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CopyAsync)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* source,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* destination,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation);
    HRESULT (STDMETHODCALLTYPE* CopySizeAsync)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* source,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* destination,
        UINT64 bytesToCopy,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation);
    HRESULT (STDMETHODCALLTYPE* CopyAndCloseAsync)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* source,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* destination,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_CopyAsync(This, source, destination, operation) \
    ((This)->lpVtbl->CopyAsync(This, source, destination, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_CopySizeAsync(This, source, destination, bytesToCopy, operation) \
    ((This)->lpVtbl->CopySizeAsync(This, source, destination, bytesToCopy, operation))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_CopyAndCloseAsync(This, source, destination, operation) \
    ((This)->lpVtbl->CopyAndCloseAsync(This, source, destination, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Streams.IRandomAccessStreamWithContentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IRandomAccessStream
 *     Windows.Foundation.IClosable
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Storage.Streams.IOutputStream
 *     Windows.Storage.Streams.IContentTypeProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Streams_IRandomAccessStreamWithContentType[] = L"Windows.Storage.Streams.IRandomAccessStreamWithContentType";
typedef struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentTypeVtbl;

interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;
#endif /* !defined(____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.Buffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Streams.IBufferFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IBufferStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IBuffer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_Buffer_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_Buffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_Buffer[] = L"Windows.Storage.Streams.Buffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Streams.IDataReaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IDataReaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IDataReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataReader_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataReader[] = L"Windows.Storage.Streams.DataReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataReaderLoadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<UInt32> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataReaderLoadOperation_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataReaderLoadOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataReaderLoadOperation[] = L"Windows.Storage.Streams.DataReaderLoadOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.Streams.IDataWriterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IDataWriter ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataWriter_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataWriter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataWriter[] = L"Windows.Storage.Streams.DataWriter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.DataWriterStoreOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<UInt32> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_DataWriterStoreOperation_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_DataWriterStoreOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_DataWriterStoreOperation[] = L"Windows.Storage.Streams.DataWriterStoreOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.FileInputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IInputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_FileInputStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_FileInputStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_FileInputStream[] = L"Windows.Storage.Streams.FileInputStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.FileOutputStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IOutputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_FileOutputStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_FileOutputStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_FileOutputStream[] = L"Windows.Storage.Streams.FileOutputStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.FileRandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IFileRandomAccessStreamStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStream ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_FileRandomAccessStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_FileRandomAccessStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_FileRandomAccessStream[] = L"Windows.Storage.Streams.FileRandomAccessStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.InMemoryRandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStream ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_InMemoryRandomAccessStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_InMemoryRandomAccessStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_InMemoryRandomAccessStream[] = L"Windows.Storage.Streams.InMemoryRandomAccessStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.InputStreamOverStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IInputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_InputStreamOverStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_InputStreamOverStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_InputStreamOverStream[] = L"Windows.Storage.Streams.InputStreamOverStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.OutputStreamOverStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IOutputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_OutputStreamOverStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_OutputStreamOverStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_OutputStreamOverStream[] = L"Windows.Storage.Streams.OutputStreamOverStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.RandomAccessStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IRandomAccessStreamStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_RandomAccessStream[] = L"Windows.Storage.Streams.RandomAccessStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.RandomAccessStreamOverStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStream ** Default Interface **
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamOverStream_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamOverStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_RandomAccessStreamOverStream[] = L"Windows.Storage.Streams.RandomAccessStreamOverStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Streams.RandomAccessStreamReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Streams.IRandomAccessStreamReferenceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamReference ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamReference_DEFINED
#define RUNTIMECLASS_Windows_Storage_Streams_RandomAccessStreamReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Streams_RandomAccessStreamReference[] = L"Windows.Storage.Streams.RandomAccessStreamReference";
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
#endif // __windows2Estorage2Estreams_p_h__

#endif // __windows2Estorage2Estreams_h__
