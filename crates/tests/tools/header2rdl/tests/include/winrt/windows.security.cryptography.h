
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
#ifndef __windows2Esecurity2Ecryptography_h__
#define __windows2Esecurity2Ecryptography_h__
#ifndef __windows2Esecurity2Ecryptography_p_h__
#define __windows2Esecurity2Ecryptography_p_h__


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
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                interface ICryptographicBufferStatics;
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics ABI::Windows::Security::Cryptography::ICryptographicBufferStatics

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                typedef enum BinaryStringEncoding : int BinaryStringEncoding;
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Cryptography.BinaryStringEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                enum BinaryStringEncoding : int
                {
                    BinaryStringEncoding_Utf8 = 0,
                    BinaryStringEncoding_Utf16LE = 1,
                    BinaryStringEncoding_Utf16BE = 2,
                };
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.ICryptographicBufferStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.CryptographicBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_ICryptographicBufferStatics[] = L"Windows.Security.Cryptography.ICryptographicBufferStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                MIDL_INTERFACE("320b7e22-3cb0-4cdf-8663-1d28910065eb")
                ICryptographicBufferStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Compare(
                        ABI::Windows::Storage::Streams::IBuffer* object1,
                        ABI::Windows::Storage::Streams::IBuffer* object2,
                        boolean* isEqual
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GenerateRandom(
                        UINT32 length,
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GenerateRandomNumber(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromByteArray(
                        UINT32 valueLength,
                        BYTE* value,
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CopyToByteArray(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        UINT32* valueLength,
                        BYTE** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DecodeFromHexString(
                        HSTRING value,
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EncodeToHexString(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DecodeFromBase64String(
                        HSTRING value,
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EncodeToBase64String(
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConvertStringToBinary(
                        HSTRING value,
                        ABI::Windows::Security::Cryptography::BinaryStringEncoding encoding,
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConvertBinaryToString(
                        ABI::Windows::Security::Cryptography::BinaryStringEncoding encoding,
                        ABI::Windows::Storage::Streams::IBuffer* buffer,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICryptographicBufferStatics = __uuidof(ICryptographicBufferStatics);
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.CryptographicBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.ICryptographicBufferStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_CryptographicBuffer_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_CryptographicBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_CryptographicBuffer[] = L"Windows.Security.Cryptography.CryptographicBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CBinaryStringEncoding __x_ABI_CWindows_CSecurity_CCryptography_CBinaryStringEncoding;

/*
 *
 * Struct Windows.Security.Cryptography.BinaryStringEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCryptography_CBinaryStringEncoding
{
    BinaryStringEncoding_Utf8 = 0,
    BinaryStringEncoding_Utf16LE = 1,
    BinaryStringEncoding_Utf16BE = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Cryptography.ICryptographicBufferStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Cryptography.CryptographicBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Cryptography_ICryptographicBufferStatics[] = L"Windows.Security.Cryptography.ICryptographicBufferStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Compare)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* object1,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* object2,
        boolean* isEqual);
    HRESULT (STDMETHODCALLTYPE* GenerateRandom)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        UINT32 length,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* GenerateRandomNumber)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* CreateFromByteArray)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        UINT32 valueLength,
        BYTE* value,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* CopyToByteArray)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* DecodeFromHexString)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        HSTRING value,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* EncodeToHexString)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* DecodeFromBase64String)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        HSTRING value,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* EncodeToBase64String)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ConvertStringToBinary)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        HSTRING value,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CBinaryStringEncoding encoding,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* ConvertBinaryToString)(__x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CBinaryStringEncoding encoding,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_Compare(This, object1, object2, isEqual) \
    ((This)->lpVtbl->Compare(This, object1, object2, isEqual))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_GenerateRandom(This, length, buffer) \
    ((This)->lpVtbl->GenerateRandom(This, length, buffer))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_GenerateRandomNumber(This, value) \
    ((This)->lpVtbl->GenerateRandomNumber(This, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_CreateFromByteArray(This, valueLength, value, buffer) \
    ((This)->lpVtbl->CreateFromByteArray(This, valueLength, value, buffer))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_CopyToByteArray(This, buffer, valueLength, value) \
    ((This)->lpVtbl->CopyToByteArray(This, buffer, valueLength, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_DecodeFromHexString(This, value, buffer) \
    ((This)->lpVtbl->DecodeFromHexString(This, value, buffer))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_EncodeToHexString(This, buffer, value) \
    ((This)->lpVtbl->EncodeToHexString(This, buffer, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_DecodeFromBase64String(This, value, buffer) \
    ((This)->lpVtbl->DecodeFromBase64String(This, value, buffer))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_EncodeToBase64String(This, buffer, value) \
    ((This)->lpVtbl->EncodeToBase64String(This, buffer, value))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_ConvertStringToBinary(This, value, encoding, buffer) \
    ((This)->lpVtbl->ConvertStringToBinary(This, value, encoding, buffer))

#define __x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_ConvertBinaryToString(This, encoding, buffer, value) \
    ((This)->lpVtbl->ConvertBinaryToString(This, encoding, buffer, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCryptography_CICryptographicBufferStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Cryptography.CryptographicBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Cryptography.ICryptographicBufferStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Cryptography_CryptographicBuffer_DEFINED
#define RUNTIMECLASS_Windows_Security_Cryptography_CryptographicBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Cryptography_CryptographicBuffer[] = L"Windows.Security.Cryptography.CryptographicBuffer";
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
#endif // __windows2Esecurity2Ecryptography_p_h__

#endif // __windows2Esecurity2Ecryptography_h__
