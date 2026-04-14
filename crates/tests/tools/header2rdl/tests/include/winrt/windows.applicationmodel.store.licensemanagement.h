
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
#ifndef __windows2Eapplicationmodel2Estore2Elicensemanagement_h__
#define __windows2Eapplicationmodel2Estore2Elicensemanagement_h__
#ifndef __windows2Eapplicationmodel2Estore2Elicensemanagement_p_h__
#define __windows2Eapplicationmodel2Estore2Elicensemanagement_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    interface ILicenseManagerStatics;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    interface ILicenseManagerStatics2;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2 ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    interface ILicenseSatisfactionInfo;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseSatisfactionInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    interface ILicenseSatisfactionResult;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseSatisfactionResult

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    class LicenseSatisfactionResult;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8447bc9-a9f8-5867-8b30-cd34720edc31"))
IAsyncOperation<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionResult*, ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseSatisfactionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("936e8471-252f-5339-89c3-9428412086ca"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionResult*, ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseSatisfactionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000


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


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    class LicenseSatisfactionInfo;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("69426b8f-88d8-5546-92ee-53f75307845b"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionInfo*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionInfo*, ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseSatisfactionInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionInfo*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("168d4306-e80a-5e37-ae46-2ceecdd7fc9a"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac935021-e04b-5226-8119-5b73d0b8be5b"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("19df5e64-c2b2-5bfd-a259-f02c23574cd3"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionInfo*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionInfo*, ABI::Windows::ApplicationModel::Store::LicenseManagement::ILicenseSatisfactionInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseSatisfactionInfo*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    typedef enum LicenseRefreshOption : int LicenseRefreshOption;
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    enum LicenseRefreshOption : int
                    {
                        LicenseRefreshOption_RunningLicenses = 0,
                        LicenseRefreshOption_AllLicenses = 1,
                    };
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseManagerStatics[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    MIDL_INTERFACE("b5ac3ae0-da47-4f20-9a23-09182c9476ff")
                    ILicenseManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddLicenseAsync(
                            ABI::Windows::Storage::Streams::IBuffer* license,
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSatisfactionInfosAsync(
                            __FIIterable_1_HSTRING* contentIds,
                            __FIIterable_1_HSTRING* keyIds,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILicenseManagerStatics = __uuidof(ILicenseManagerStatics);
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseManagerStatics2[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    MIDL_INTERFACE("ab2ec47b-1f79-4480-b87e-2c499e601ba3")
                    ILicenseManagerStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RefreshLicensesAsync(
                            ABI::Windows::ApplicationModel::Store::LicenseManagement::LicenseRefreshOption refreshOption,
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILicenseManagerStatics2 = __uuidof(ILicenseManagerStatics2);
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseSatisfactionInfo[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    MIDL_INTERFACE("3ccbb08f-db31-48d5-8384-fa17c81474e2")
                    ILicenseSatisfactionInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SatisfiedByDevice(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SatisfiedByOpenLicense(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SatisfiedByTrial(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SatisfiedByPass(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SatisfiedByInstallMedia(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SatisfiedBySignedInUser(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSatisfied(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILicenseSatisfactionInfo = __uuidof(ILicenseSatisfactionInfo);
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseSatisfactionResult[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace LicenseManagement {
                    MIDL_INTERFACE("3c674f73-3c87-4ee1-8201-f428359bd3af")
                    ILicenseSatisfactionResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LicenseSatisfactionInfos(
                            __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILicenseSatisfactionResult = __uuidof(ILicenseSatisfactionResult);
                } /* LicenseManagement */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseManagement.LicenseManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseManagement_LicenseManager[] = L"Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionInfo[] = L"Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionResult[] = L"Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2 __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CLicenseRefreshOption __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CLicenseRefreshOption;

/*
 *
 * Struct Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CLicenseRefreshOption
{
    LicenseRefreshOption_RunningLicenses = 0,
    LicenseRefreshOption_AllLicenses = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseManagerStatics[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddLicenseAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* license,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* GetSatisfactionInfosAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics* This,
        __FIIterable_1_HSTRING* contentIds,
        __FIIterable_1_HSTRING* keyIds,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_AddLicenseAsync(This, license, action) \
    ((This)->lpVtbl->AddLicenseAsync(This, license, action))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_GetSatisfactionInfosAsync(This, contentIds, keyIds, operation) \
    ((This)->lpVtbl->GetSatisfactionInfosAsync(This, contentIds, keyIds, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseManagerStatics2[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RefreshLicensesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CLicenseRefreshOption refreshOption,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_RefreshLicensesAsync(This, refreshOption, action) \
    ((This)->lpVtbl->RefreshLicensesAsync(This, refreshOption, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseSatisfactionInfo[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SatisfiedByDevice)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SatisfiedByOpenLicense)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SatisfiedByTrial)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SatisfiedByPass)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SatisfiedByInstallMedia)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SatisfiedBySignedInUser)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSatisfied)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_SatisfiedByDevice(This, value) \
    ((This)->lpVtbl->get_SatisfiedByDevice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_SatisfiedByOpenLicense(This, value) \
    ((This)->lpVtbl->get_SatisfiedByOpenLicense(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_SatisfiedByTrial(This, value) \
    ((This)->lpVtbl->get_SatisfiedByTrial(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_SatisfiedByPass(This, value) \
    ((This)->lpVtbl->get_SatisfiedByPass(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_SatisfiedByInstallMedia(This, value) \
    ((This)->lpVtbl->get_SatisfiedByInstallMedia(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_SatisfiedBySignedInUser(This, value) \
    ((This)->lpVtbl->get_SatisfiedBySignedInUser(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_get_IsSatisfied(This, value) \
    ((This)->lpVtbl->get_IsSatisfied(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_LicenseManagement_ILicenseSatisfactionResult[] = L"Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LicenseSatisfactionInfos)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CStore__CLicenseManagement__CLicenseSatisfactionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_get_LicenseSatisfactionInfos(This, value) \
    ((This)->lpVtbl->get_LicenseSatisfactionInfos(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CLicenseManagement_CILicenseSatisfactionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseManagement.LicenseManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Store.LicenseManagement.ILicenseManagerStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseManagement_LicenseManager[] = L"Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionInfo[] = L"Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.LicenseManagement.ILicenseSatisfactionResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_LicenseManagement_LicenseSatisfactionResult[] = L"Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Estore2Elicensemanagement_p_h__

#endif // __windows2Eapplicationmodel2Estore2Elicensemanagement_h__
