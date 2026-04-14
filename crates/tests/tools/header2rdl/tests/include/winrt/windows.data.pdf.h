
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
#ifndef __windows2Edata2Epdf_h__
#define __windows2Edata2Epdf_h__
#ifndef __windows2Edata2Epdf_p_h__
#define __windows2Edata2Epdf_p_h__


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
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                interface IPdfDocument;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument ABI::Windows::Data::Pdf::IPdfDocument

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                interface IPdfDocumentStatics;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics ABI::Windows::Data::Pdf::IPdfDocumentStatics

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfPage_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                interface IPdfPage;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CPdf_CIPdfPage ABI::Windows::Data::Pdf::IPdfPage

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfPage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                interface IPdfPageDimensions;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions ABI::Windows::Data::Pdf::IPdfPageDimensions

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                interface IPdfPageRenderOptions;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions ABI::Windows::Data::Pdf::IPdfPageRenderOptions

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                class PdfDocument;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_USE
#define DEF___FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d6b166ec-099a-5ee2-ad2e-f4c88614aabb"))
IAsyncOperation<ABI::Windows::Data::Pdf::PdfDocument*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Pdf::PdfDocument*, ABI::Windows::Data::Pdf::IPdfDocument*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Data.Pdf.PdfDocument>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Data::Pdf::PdfDocument*> __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_t;
#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8d4950b3-629d-5d7d-84cc-04c0dcf7942b"))
IAsyncOperationCompletedHandler<ABI::Windows::Data::Pdf::PdfDocument*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Pdf::PdfDocument*, ABI::Windows::Data::Pdf::IPdfDocument*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Data.Pdf.PdfDocument>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Data::Pdf::PdfDocument*> __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_USE */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
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

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                typedef enum PdfPageRotation : int PdfPageRotation;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                class PdfPage;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                class PdfPageDimensions;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                class PdfPageRenderOptions;
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Data.Pdf.PdfPageRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                enum PdfPageRotation : int
                {
                    PdfPageRotation_Normal = 0,
                    PdfPageRotation_Rotate90 = 1,
                    PdfPageRotation_Rotate180 = 2,
                    PdfPageRotation_Rotate270 = 3,
                };
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocument_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfDocument[] = L"Windows.Data.Pdf.IPdfDocument";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                MIDL_INTERFACE("ac7ebedd-80fa-4089-846e-81b77ff5a86c")
                IPdfDocument : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPage(
                        UINT32 pageIndex,
                        ABI::Windows::Data::Pdf::IPdfPage** pdfPage
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PageCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPasswordProtected(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPdfDocument = __uuidof(IPdfDocument);
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfDocument;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocument_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfDocumentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfDocumentStatics[] = L"Windows.Data.Pdf.IPdfDocumentStatics";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                MIDL_INTERFACE("433a0b5f-c007-4788-90f2-08143d922599")
                IPdfDocumentStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadFromFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromFileWithPasswordAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        HSTRING password,
                        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* inputStream,
                        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromStreamWithPasswordAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* inputStream,
                        HSTRING password,
                        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPdfDocumentStatics = __uuidof(IPdfDocumentStatics);
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfPage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfPage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfPage[] = L"Windows.Data.Pdf.IPdfPage";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                MIDL_INTERFACE("9db4b0c8-5320-4cfc-ad76-493fdad0e594")
                IPdfPage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RenderToStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* outputStream,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RenderWithOptionsToStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* outputStream,
                        ABI::Windows::Data::Pdf::IPdfPageRenderOptions* options,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PreparePageAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Index(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Dimensions(
                        ABI::Windows::Data::Pdf::IPdfPageDimensions** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rotation(
                        ABI::Windows::Data::Pdf::PdfPageRotation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredZoom(
                        FLOAT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPdfPage = __uuidof(IPdfPage);
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfPage;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfPageDimensions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfPageDimensions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfPageDimensions[] = L"Windows.Data.Pdf.IPdfPageDimensions";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                MIDL_INTERFACE("22170471-313e-44e8-835d-63a3e7624a10")
                IPdfPageDimensions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MediaBox(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CropBox(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BleedBox(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimBox(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ArtBox(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPdfPageDimensions = __uuidof(IPdfPageDimensions);
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfPageRenderOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfPageRenderOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfPageRenderOptions[] = L"Windows.Data.Pdf.IPdfPageRenderOptions";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Pdf {
                MIDL_INTERFACE("3c98056f-b7cf-4c29-9a04-52d90267f425")
                IPdfPageRenderOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SourceRect(
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DestinationWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DestinationWidth(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DestinationHeight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DestinationHeight(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIgnoringHighContrast(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsIgnoringHighContrast(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapEncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BitmapEncoderId(
                        GUID value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPdfPageRenderOptions = __uuidof(IPdfPageRenderOptions);
            } /* Pdf */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Data.Pdf.IPdfDocumentStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfDocument ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfDocument_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfDocument_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfDocument[] = L"Windows.Data.Pdf.PdfDocument";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfPage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfPage ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfPage_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfPage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfPage[] = L"Windows.Data.Pdf.PdfPage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfPageDimensions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfPageDimensions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfPageDimensions_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfPageDimensions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfPageDimensions[] = L"Windows.Data.Pdf.PdfPageDimensions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfPageRenderOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfPageRenderOptions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfPageRenderOptions_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfPageRenderOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfPageRenderOptions[] = L"Windows.Data.Pdf.PdfPageRenderOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CPdf_CIPdfDocument __x_ABI_CWindows_CData_CPdf_CIPdfDocument;

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics;

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfPage_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CPdf_CIPdfPage __x_ABI_CWindows_CData_CPdf_CIPdfPage;

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfPage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions;

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions;

#endif // ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument;

typedef struct __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* This,
        __x_ABI_CWindows_CData_CPdf_CIPdfDocument** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocumentVtbl;

interface __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument* This,
        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocumentVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CData__CPdf__CPdfDocument_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CData_CPdf_CPdfPageRotation __x_ABI_CWindows_CData_CPdf_CPdfPageRotation;

/*
 *
 * Struct Windows.Data.Pdf.PdfPageRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CData_CPdf_CPdfPageRotation
{
    PdfPageRotation_Normal = 0,
    PdfPageRotation_Rotate90 = 1,
    PdfPageRotation_Rotate180 = 2,
    PdfPageRotation_Rotate270 = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocument_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocument_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfDocument[] = L"Windows.Data.Pdf.IPdfDocument";
typedef struct __x_ABI_CWindows_CData_CPdf_CIPdfDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPage)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        UINT32 pageIndex,
        __x_ABI_CWindows_CData_CPdf_CIPdfPage** pdfPage);
    HRESULT (STDMETHODCALLTYPE* get_PageCount)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPasswordProtected)(__x_ABI_CWindows_CData_CPdf_CIPdfDocument* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CPdf_CIPdfDocumentVtbl;

interface __x_ABI_CWindows_CData_CPdf_CIPdfDocument
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CPdf_CIPdfDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_GetPage(This, pageIndex, pdfPage) \
    ((This)->lpVtbl->GetPage(This, pageIndex, pdfPage))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_get_PageCount(This, value) \
    ((This)->lpVtbl->get_PageCount(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocument_get_IsPasswordProtected(This, value) \
    ((This)->lpVtbl->get_IsPasswordProtected(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfDocument;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocument_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfDocumentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfDocumentStatics[] = L"Windows.Data.Pdf.IPdfDocumentStatics";
typedef struct __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadFromFileAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LoadFromFileWithPasswordAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING password,
        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LoadFromStreamAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* inputStream,
        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LoadFromStreamWithPasswordAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* inputStream,
        HSTRING password,
        __FIAsyncOperation_1_Windows__CData__CPdf__CPdfDocument** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStaticsVtbl;

interface __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_LoadFromFileAsync(This, file, asyncInfo) \
    ((This)->lpVtbl->LoadFromFileAsync(This, file, asyncInfo))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_LoadFromFileWithPasswordAsync(This, file, password, asyncInfo) \
    ((This)->lpVtbl->LoadFromFileWithPasswordAsync(This, file, password, asyncInfo))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_LoadFromStreamAsync(This, inputStream, asyncInfo) \
    ((This)->lpVtbl->LoadFromStreamAsync(This, inputStream, asyncInfo))

#define __x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_LoadFromStreamWithPasswordAsync(This, inputStream, password, asyncInfo) \
    ((This)->lpVtbl->LoadFromStreamWithPasswordAsync(This, inputStream, password, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfDocumentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfPage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfPage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfPage[] = L"Windows.Data.Pdf.IPdfPage";
typedef struct __x_ABI_CWindows_CData_CPdf_CIPdfPageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RenderToStreamAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* outputStream,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* RenderWithOptionsToStreamAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* outputStream,
        __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* options,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* PreparePageAsync)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_Index)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_Dimensions)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions** value);
    HRESULT (STDMETHODCALLTYPE* get_Rotation)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        enum __x_ABI_CWindows_CData_CPdf_CPdfPageRotation* value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredZoom)(__x_ABI_CWindows_CData_CPdf_CIPdfPage* This,
        FLOAT* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CPdf_CIPdfPageVtbl;

interface __x_ABI_CWindows_CData_CPdf_CIPdfPage
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CPdf_CIPdfPageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_RenderToStreamAsync(This, outputStream, asyncInfo) \
    ((This)->lpVtbl->RenderToStreamAsync(This, outputStream, asyncInfo))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_RenderWithOptionsToStreamAsync(This, outputStream, options, asyncInfo) \
    ((This)->lpVtbl->RenderWithOptionsToStreamAsync(This, outputStream, options, asyncInfo))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_PreparePageAsync(This, asyncInfo) \
    ((This)->lpVtbl->PreparePageAsync(This, asyncInfo))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_get_Index(This, value) \
    ((This)->lpVtbl->get_Index(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_get_Dimensions(This, value) \
    ((This)->lpVtbl->get_Dimensions(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_get_Rotation(This, value) \
    ((This)->lpVtbl->get_Rotation(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPage_get_PreferredZoom(This, value) \
    ((This)->lpVtbl->get_PreferredZoom(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfPage;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfPageDimensions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfPageDimensions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfPageDimensions[] = L"Windows.Data.Pdf.IPdfPageDimensions";
typedef struct __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MediaBox)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_CropBox)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_BleedBox)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_TrimBox)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_ArtBox)(__x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensionsVtbl;

interface __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_get_MediaBox(This, value) \
    ((This)->lpVtbl->get_MediaBox(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_get_CropBox(This, value) \
    ((This)->lpVtbl->get_CropBox(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_get_BleedBox(This, value) \
    ((This)->lpVtbl->get_BleedBox(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_get_TrimBox(This, value) \
    ((This)->lpVtbl->get_TrimBox(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_get_ArtBox(This, value) \
    ((This)->lpVtbl->get_ArtBox(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageDimensions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Pdf.IPdfPageRenderOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Pdf.PdfPageRenderOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Pdf_IPdfPageRenderOptions[] = L"Windows.Data.Pdf.IPdfPageRenderOptions";
typedef struct __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceRect)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_SourceRect)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* get_DestinationWidth)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DestinationWidth)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DestinationHeight)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DestinationHeight)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_IsIgnoringHighContrast)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsIgnoringHighContrast)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapEncoderId)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_BitmapEncoderId)(__x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptionsVtbl;

interface __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_get_SourceRect(This, value) \
    ((This)->lpVtbl->get_SourceRect(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_put_SourceRect(This, value) \
    ((This)->lpVtbl->put_SourceRect(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_get_DestinationWidth(This, value) \
    ((This)->lpVtbl->get_DestinationWidth(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_put_DestinationWidth(This, value) \
    ((This)->lpVtbl->put_DestinationWidth(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_get_DestinationHeight(This, value) \
    ((This)->lpVtbl->get_DestinationHeight(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_put_DestinationHeight(This, value) \
    ((This)->lpVtbl->put_DestinationHeight(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_get_IsIgnoringHighContrast(This, value) \
    ((This)->lpVtbl->get_IsIgnoringHighContrast(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_put_IsIgnoringHighContrast(This, value) \
    ((This)->lpVtbl->put_IsIgnoringHighContrast(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_get_BitmapEncoderId(This, value) \
    ((This)->lpVtbl->get_BitmapEncoderId(This, value))

#define __x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_put_BitmapEncoderId(This, value) \
    ((This)->lpVtbl->put_BitmapEncoderId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions;
#endif /* !defined(____x_ABI_CWindows_CData_CPdf_CIPdfPageRenderOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Data.Pdf.IPdfDocumentStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfDocument ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfDocument_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfDocument_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfDocument[] = L"Windows.Data.Pdf.PdfDocument";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfPage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfPage ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfPage_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfPage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfPage[] = L"Windows.Data.Pdf.PdfPage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfPageDimensions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfPageDimensions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfPageDimensions_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfPageDimensions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfPageDimensions[] = L"Windows.Data.Pdf.PdfPageDimensions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Pdf.PdfPageRenderOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Pdf.IPdfPageRenderOptions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Pdf_PdfPageRenderOptions_DEFINED
#define RUNTIMECLASS_Windows_Data_Pdf_PdfPageRenderOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Pdf_PdfPageRenderOptions[] = L"Windows.Data.Pdf.PdfPageRenderOptions";
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
#endif // __windows2Edata2Epdf_p_h__

#endif // __windows2Edata2Epdf_h__
