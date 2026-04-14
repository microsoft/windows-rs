
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
#ifndef __windows2Eapplicationmodel2Ecommunicationblocking_h__
#define __windows2Eapplicationmodel2Ecommunicationblocking_h__
#ifndef __windows2Eapplicationmodel2Ecommunicationblocking_p_h__
#define __windows2Eapplicationmodel2Ecommunicationblocking_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION)

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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace CommunicationBlocking {
                interface ICommunicationBlockingAccessManagerStatics;
            } /* CommunicationBlocking */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics ABI::Windows::ApplicationModel::CommunicationBlocking::ICommunicationBlockingAccessManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace CommunicationBlocking {
                interface ICommunicationBlockingAppManagerStatics;
            } /* CommunicationBlocking */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics ABI::Windows::ApplicationModel::CommunicationBlocking::ICommunicationBlockingAppManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace CommunicationBlocking {
                interface ICommunicationBlockingAppManagerStatics2;
            } /* CommunicationBlocking */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2 ABI::Windows::ApplicationModel::CommunicationBlocking::ICommunicationBlockingAppManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_FWD_DEFINED__

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



#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


/*
 *
 * Interface Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_CommunicationBlocking_ICommunicationBlockingAccessManagerStatics[] = L"Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace CommunicationBlocking {
                MIDL_INTERFACE("1c969998-9d2a-5db7-edd5-0ce407fc2595")
                ICommunicationBlockingAccessManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsBlockingActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsBlockedNumberAsync(
                        HSTRING number,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowBlockNumbersUI(
                        __FIIterable_1_HSTRING* phoneNumbers,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowUnblockNumbersUI(
                        __FIIterable_1_HSTRING* phoneNumbers,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowBlockedCallsUI(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowBlockedMessagesUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ICommunicationBlockingAccessManagerStatics = __uuidof(ICommunicationBlockingAccessManagerStatics);
            } /* CommunicationBlocking */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_CommunicationBlocking_ICommunicationBlockingAppManagerStatics[] = L"Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace CommunicationBlocking {
                MIDL_INTERFACE("77db58ec-14a6-4baa-942a-6a673d999bf2")
                ICommunicationBlockingAppManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsCurrentAppActiveBlockingApp(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowCommunicationBlockingSettingsUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ICommunicationBlockingAppManagerStatics = __uuidof(ICommunicationBlockingAppManagerStatics);
            } /* CommunicationBlocking */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_CommunicationBlocking_ICommunicationBlockingAppManagerStatics2[] = L"Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace CommunicationBlocking {
                MIDL_INTERFACE("14a68edd-ed88-457a-a364-a3634d6f166d")
                ICommunicationBlockingAppManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestSetAsActiveBlockingAppAsync(
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICommunicationBlockingAppManagerStatics2 = __uuidof(ICommunicationBlockingAppManagerStatics2);
            } /* CommunicationBlocking */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAccessManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAccessManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAccessManager[] = L"Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2 interface starting with version 2.0 of the Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAppManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAppManager[] = L"Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2 __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_FWD_DEFINED__

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

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_CommunicationBlocking_ICommunicationBlockingAccessManagerStatics[] = L"Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsBlockingActive)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsBlockedNumberAsync)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        HSTRING number,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* ShowBlockNumbersUI)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        __FIIterable_1_HSTRING* phoneNumbers,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ShowUnblockNumbersUI)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This,
        __FIIterable_1_HSTRING* phoneNumbers,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ShowBlockedCallsUI)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* ShowBlockedMessagesUI)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_get_IsBlockingActive(This, value) \
    ((This)->lpVtbl->get_IsBlockingActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_IsBlockedNumberAsync(This, number, result) \
    ((This)->lpVtbl->IsBlockedNumberAsync(This, number, result))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_ShowBlockNumbersUI(This, phoneNumbers, value) \
    ((This)->lpVtbl->ShowBlockNumbersUI(This, phoneNumbers, value))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_ShowUnblockNumbersUI(This, phoneNumbers, value) \
    ((This)->lpVtbl->ShowUnblockNumbersUI(This, phoneNumbers, value))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_ShowBlockedCallsUI(This) \
    ((This)->lpVtbl->ShowBlockedCallsUI(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_ShowBlockedMessagesUI(This) \
    ((This)->lpVtbl->ShowBlockedMessagesUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAccessManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_CommunicationBlocking_ICommunicationBlockingAppManagerStatics[] = L"Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCurrentAppActiveBlockingApp)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ShowCommunicationBlockingSettingsUI)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_get_IsCurrentAppActiveBlockingApp(This, value) \
    ((This)->lpVtbl->get_IsCurrentAppActiveBlockingApp(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_ShowCommunicationBlockingSettingsUI(This) \
    ((This)->lpVtbl->ShowCommunicationBlockingSettingsUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_CommunicationBlocking_ICommunicationBlockingAppManagerStatics2[] = L"Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestSetAsActiveBlockingAppAsync)(__x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2* This,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_RequestSetAsActiveBlockingAppAsync(This, result) \
    ((This)->lpVtbl->RequestSetAsActiveBlockingAppAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCommunicationBlocking_CICommunicationBlockingAppManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAccessManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAccessManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAccessManager[] = L"Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager
 *
 * Introduced to Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2 interface starting with version 2.0 of the Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAppManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_CommunicationBlocking_CommunicationBlockingAppManager[] = L"Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Ecommunicationblocking_p_h__

#endif // __windows2Eapplicationmodel2Ecommunicationblocking_h__
