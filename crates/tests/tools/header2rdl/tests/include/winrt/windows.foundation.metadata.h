
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
#ifndef __windows2Efoundation2Emetadata_h__
#define __windows2Efoundation2Emetadata_h__
#ifndef __windows2Efoundation2Emetadata_p_h__
#define __windows2Efoundation2Emetadata_p_h__


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
#ifndef ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Metadata {
                interface IApiInformationStatics;
            } /* Metadata */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics ABI::Windows::Foundation::Metadata::IApiInformationStatics

#endif // ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
/*
 *
 * Struct Windows.Foundation.Metadata.GCPressureAmount
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Metadata {
                enum GCPressureAmount : int
                {
                    GCPressureAmount_Low = 0,
                    GCPressureAmount_Medium = 1,
                    GCPressureAmount_High = 2,
                };
            } /* Metadata */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Metadata.IApiInformationStatics
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Metadata.ApiInformation
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Metadata_IApiInformationStatics[] = L"Windows.Foundation.Metadata.IApiInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Metadata {
                MIDL_INTERFACE("997439fe-f681-4a11-b416-c13a47e8ba36")
                IApiInformationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsTypePresent(
                        HSTRING typeName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsMethodPresent(
                        HSTRING typeName,
                        HSTRING methodName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsMethodPresentWithArity(
                        HSTRING typeName,
                        HSTRING methodName,
                        UINT32 inputParameterCount,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsEventPresent(
                        HSTRING typeName,
                        HSTRING eventName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsPropertyPresent(
                        HSTRING typeName,
                        HSTRING propertyName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsReadOnlyPropertyPresent(
                        HSTRING typeName,
                        HSTRING propertyName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsWriteablePropertyPresent(
                        HSTRING typeName,
                        HSTRING propertyName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsEnumNamedValuePresent(
                        HSTRING enumTypeName,
                        HSTRING valueName,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsApiContractPresentByMajor(
                        HSTRING contractName,
                        UINT16 majorVersion,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsApiContractPresentByMajorAndMinor(
                        HSTRING contractName,
                        UINT16 majorVersion,
                        UINT16 minorVersion,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApiInformationStatics = __uuidof(IApiInformationStatics);
            } /* Metadata */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Metadata.ApiInformation
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.Metadata.IApiInformationStatics interface starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Metadata_ApiInformation_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Metadata_ApiInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Metadata_ApiInformation[] = L"Windows.Foundation.Metadata.ApiInformation";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics;

#endif // ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Struct Windows.Foundation.Metadata.GCPressureAmount
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CMetadata_CGCPressureAmount
{
    GCPressureAmount_Low = 0,
    GCPressureAmount_Medium = 1,
    GCPressureAmount_High = 2,
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Metadata.IApiInformationStatics
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Metadata.ApiInformation
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Metadata_IApiInformationStatics[] = L"Windows.Foundation.Metadata.IApiInformationStatics";
typedef struct __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsTypePresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsMethodPresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        HSTRING methodName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsMethodPresentWithArity)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        HSTRING methodName,
        UINT32 inputParameterCount,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsEventPresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        HSTRING eventName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsPropertyPresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        HSTRING propertyName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsReadOnlyPropertyPresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        HSTRING propertyName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsWriteablePropertyPresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING typeName,
        HSTRING propertyName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsEnumNamedValuePresent)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING enumTypeName,
        HSTRING valueName,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsApiContractPresentByMajor)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING contractName,
        UINT16 majorVersion,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsApiContractPresentByMajorAndMinor)(__x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics* This,
        HSTRING contractName,
        UINT16 majorVersion,
        UINT16 minorVersion,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStaticsVtbl;

interface __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsTypePresent(This, typeName, result) \
    ((This)->lpVtbl->IsTypePresent(This, typeName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsMethodPresent(This, typeName, methodName, result) \
    ((This)->lpVtbl->IsMethodPresent(This, typeName, methodName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsMethodPresentWithArity(This, typeName, methodName, inputParameterCount, result) \
    ((This)->lpVtbl->IsMethodPresentWithArity(This, typeName, methodName, inputParameterCount, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsEventPresent(This, typeName, eventName, result) \
    ((This)->lpVtbl->IsEventPresent(This, typeName, eventName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsPropertyPresent(This, typeName, propertyName, result) \
    ((This)->lpVtbl->IsPropertyPresent(This, typeName, propertyName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsReadOnlyPropertyPresent(This, typeName, propertyName, result) \
    ((This)->lpVtbl->IsReadOnlyPropertyPresent(This, typeName, propertyName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsWriteablePropertyPresent(This, typeName, propertyName, result) \
    ((This)->lpVtbl->IsWriteablePropertyPresent(This, typeName, propertyName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsEnumNamedValuePresent(This, enumTypeName, valueName, result) \
    ((This)->lpVtbl->IsEnumNamedValuePresent(This, enumTypeName, valueName, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsApiContractPresentByMajor(This, contractName, majorVersion, result) \
    ((This)->lpVtbl->IsApiContractPresentByMajor(This, contractName, majorVersion, result))

#define __x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_IsApiContractPresentByMajorAndMinor(This, contractName, majorVersion, minorVersion, result) \
    ((This)->lpVtbl->IsApiContractPresentByMajorAndMinor(This, contractName, majorVersion, minorVersion, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CMetadata_CIApiInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Metadata.ApiInformation
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.Metadata.IApiInformationStatics interface starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Metadata_ApiInformation_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Metadata_ApiInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Metadata_ApiInformation[] = L"Windows.Foundation.Metadata.ApiInformation";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Efoundation2Emetadata_p_h__

#endif // __windows2Efoundation2Emetadata_h__
