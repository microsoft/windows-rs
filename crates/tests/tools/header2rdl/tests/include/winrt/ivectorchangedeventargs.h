/* Header file automatically generated from ivectorchangedeventargs.idl */
/*
 * File built with Microsoft(R) MIDLRT Compiler Engine Version 10.00.0231 
 */

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
#ifndef __ivectorchangedeventargs_h__
#define __ivectorchangedeventargs_h__
#ifndef __ivectorchangedeventargs_p_h__
#define __ivectorchangedeventargs_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)
#define WINDOWS_PHONE_PHONECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)
#define WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)

#if !defined(WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION)
#define WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "oaidl.h"
#include "inspectable.h"
#include "asyncinfo.h"
#include "eventtoken.h"
#include "windowscontracts.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IVectorChangedEventArgs;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs ABI::Windows::Foundation::Collections::IVectorChangedEventArgs

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__



#pragma warning (push)
#pragma warning (disable:4668) 
#pragma warning (disable:4001) 
#pragma once 
#pragma warning (pop)

/*
 *
 * Typedef of Windows.Foundation.Collections.CollectionChange
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                /* [contract, version] */
                typedef /* [v1_enum] */
                enum CollectionChange : int
                {
                    CollectionChange_Reset,
                    CollectionChange_ItemInserted,
                    CollectionChange_ItemRemoved,
                    CollectionChange_ItemChanged,
                } CollectionChange;
                
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.Collections.IVectorChangedEventArgs
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Collections_IVectorChangedEventArgs[] = L"Windows.Foundation.Collections.IVectorChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                /* [contract, version, pointer_default(unique), uuid("575933df-34fe-4480-af15-07691f3d5d9b"), object] */
                MIDL_INTERFACE("575933df-34fe-4480-af15-07691f3d5d9b")
                IVectorChangedEventArgs : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_CollectionChange(
                        /* [retval, out] */__RPC__out ABI::Windows::Foundation::Collections::CollectionChange * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Index(
                        /* [retval, out] */__RPC__out unsigned int * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IVectorChangedEventArgs=__uuidof(IVectorChangedEventArgs);
                
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__


#pragma warning (push)
#pragma warning (disable:4668) 
#pragma warning (disable:4001) 
#pragma once 
#pragma warning (pop)

/*
 *
 * Typedef of Windows.Foundation.Collections.CollectionChange
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange
{
    CollectionChange_Reset,
    CollectionChange_ItemInserted,
    CollectionChange_ItemRemoved,
    CollectionChange_ItemChanged,
} __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange;
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.Collections.IVectorChangedEventArgs
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Collections_IVectorChangedEventArgs[] = L"Windows.Foundation.Collections.IVectorChangedEventArgs";
/* [contract, version, pointer_default(unique), uuid("575933df-34fe-4480-af15-07691f3d5d9b"), object] */
typedef struct __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_CollectionChange )(
        __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Index )(
        __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs * This,
        /* [retval, out] */__RPC__out unsigned int * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgsVtbl;

interface __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_get_CollectionChange(This,value) \
    ( (This)->lpVtbl->get_CollectionChange(This,value) )

#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_get_Index(This,value) \
    ( (This)->lpVtbl->get_Index(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_INTERFACE_DEFINED__) */
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
#endif // __ivectorchangedeventargs_p_h__

#endif // __ivectorchangedeventargs_h__
