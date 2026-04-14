
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
#ifndef __windows2Eapplicationmodel2Esearch_h__
#define __windows2Eapplicationmodel2Esearch_h__
#ifndef __windows2Eapplicationmodel2Esearch_p_h__
#define __windows2Eapplicationmodel2Esearch_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION)

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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ILocalContentSuggestionSettings;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings ABI::Windows::ApplicationModel::Search::ILocalContentSuggestionSettings

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPane;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane ABI::Windows::ApplicationModel::Search::ISearchPane

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneQueryChangedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs ABI::Windows::ApplicationModel::Search::ISearchPaneQueryChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails ABI::Windows::ApplicationModel::Search::ISearchPaneQueryLinguisticDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneQuerySubmittedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs ABI::Windows::ApplicationModel::Search::ISearchPaneQuerySubmittedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails ABI::Windows::ApplicationModel::Search::ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneResultSuggestionChosenEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs ABI::Windows::ApplicationModel::Search::ISearchPaneResultSuggestionChosenEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneStatics;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics ABI::Windows::ApplicationModel::Search::ISearchPaneStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneStaticsWithHideThisApplication;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication ABI::Windows::ApplicationModel::Search::ISearchPaneStaticsWithHideThisApplication

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneSuggestionsRequest;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest ABI::Windows::ApplicationModel::Search::ISearchPaneSuggestionsRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneSuggestionsRequestDeferral;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral ABI::Windows::ApplicationModel::Search::ISearchPaneSuggestionsRequestDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneSuggestionsRequestedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs ABI::Windows::ApplicationModel::Search::ISearchPaneSuggestionsRequestedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneVisibilityChangedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs ABI::Windows::ApplicationModel::Search::ISearchPaneVisibilityChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails ABI::Windows::ApplicationModel::Search::ISearchQueryLinguisticDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchQueryLinguisticDetailsFactory;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory ABI::Windows::ApplicationModel::Search::ISearchQueryLinguisticDetailsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchSuggestionCollection;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection ABI::Windows::ApplicationModel::Search::ISearchSuggestionCollection

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchSuggestionsRequest;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest ABI::Windows::ApplicationModel::Search::ISearchSuggestionsRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchSuggestionsRequestDeferral;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral ABI::Windows::ApplicationModel::Search::ISearchSuggestionsRequestDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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
        namespace Storage {
            class StorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIIterator_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5aac96fb-b3b9-5a7f-a920-4b5a8df81168"))
IIterator<ABI::Windows::Storage::StorageFolder*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::StorageFolder*> __FIIterator_1_Windows__CStorage__CStorageFolder_t;
#define __FIIterator_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIIterable_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4669befc-ae5c-52b1-8a97-5466ce61e94e"))
IIterable<ABI::Windows::Storage::StorageFolder*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::StorageFolder*> __FIIterable_1_Windows__CStorage__CStorageFolder_t;
#define __FIIterable_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e20debc6-dc4e-542e-a2e7-a24d19c8dd62"))
IVectorView<ABI::Windows::Storage::StorageFolder*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::StorageFolder*> __FIVectorView_1_Windows__CStorage__CStorageFolder_t;
#define __FIVectorView_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIVector_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6c26b7be-5f01-5a60-9dd7-fd17be3a9dd6"))
IVector<ABI::Windows::Storage::StorageFolder*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Storage::StorageFolder*> __FIVector_1_Windows__CStorage__CStorageFolder_t;
#define __FIVector_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPane;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneQueryChangedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7ccd7da1-8767-5eef-972d-31d09f1bf308"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneQueryChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::ISearchPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPaneQueryChangedEventArgs*, ABI::Windows::ApplicationModel::Search::ISearchPaneQueryChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.SearchPane, Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneQueryChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneQuerySubmittedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5f05121b-61a6-5f6b-b007-20816dfe7009"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneQuerySubmittedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::ISearchPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPaneQuerySubmittedEventArgs*, ABI::Windows::ApplicationModel::Search::ISearchPaneQuerySubmittedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.SearchPane, Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneQuerySubmittedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneResultSuggestionChosenEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ec31e5d4-3b8c-5757-96bf-14096be946cd"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneResultSuggestionChosenEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::ISearchPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPaneResultSuggestionChosenEventArgs*, ABI::Windows::ApplicationModel::Search::ISearchPaneResultSuggestionChosenEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.SearchPane, Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneResultSuggestionChosenEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneSuggestionsRequestedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("95088dc0-6c14-55b8-8a8f-9df1ca44d1d4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneSuggestionsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::ISearchPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPaneSuggestionsRequestedEventArgs*, ABI::Windows::ApplicationModel::Search::ISearchPaneSuggestionsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.SearchPane, Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneSuggestionsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneVisibilityChangedEventArgs;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3975ff72-971b-54b6-9b5f-cc442e2a87f0"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneVisibilityChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::ISearchPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::SearchPaneVisibilityChangedEventArgs*, ABI::Windows::ApplicationModel::Search::ISearchPaneVisibilityChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.SearchPane, Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::SearchPane*, ABI::Windows::ApplicationModel::Search::SearchPaneVisibilityChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class LocalContentSuggestionSettings;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneSuggestionsRequest;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneSuggestionsRequestDeferral;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchSuggestionCollection;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchSuggestionsRequestDeferral;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Search.ILocalContentSuggestionSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.LocalContentSuggestionSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ILocalContentSuggestionSettings[] = L"Windows.ApplicationModel.Search.ILocalContentSuggestionSettings";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("eeaeb062-743d-456e-84a3-23f06f2d15d7")
                ILocalContentSuggestionSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Enabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Locations(
                        __FIVector_1_Windows__CStorage__CStorageFolder** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AqsFilter(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AqsFilter(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PropertiesToMatch(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILocalContentSuggestionSettings = __uuidof(ILocalContentSuggestionSettings);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPane
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPane
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPane[] = L"Windows.ApplicationModel.Search.ISearchPane";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("fdacec38-3700-4d73-91a1-2f998674238a")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPane : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_SearchHistoryEnabled(
                        boolean value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SearchHistoryEnabled(
                        boolean* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_SearchHistoryContext(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SearchHistoryContext(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_PlaceholderText(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PlaceholderText(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_QueryText(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Visible(
                        boolean* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_VisibilityChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_VisibilityChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_QueryChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_QueryChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_SuggestionsRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_SuggestionsRequested(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_QuerySubmitted(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_QuerySubmitted(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_ResultSuggestionChosen(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_ResultSuggestionChosen(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SetLocalContentSuggestionSettings(
                        ABI::Windows::ApplicationModel::Search::ILocalContentSuggestionSettings* settings
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE ShowOverloadDefault(void) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE ShowOverloadWithQuery(
                        HSTRING query
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_ShowOnKeyboardInput(
                        boolean value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_ShowOnKeyboardInput(
                        boolean* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE TrySetQueryText(
                        HSTRING query,
                        boolean* succeeded
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPane = __uuidof(ISearchPane);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQueryChangedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("3c064fe9-2351-4248-a529-7110f464a785")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneQueryChangedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_QueryText(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_LinguisticDetails(
                        ABI::Windows::ApplicationModel::Search::ISearchPaneQueryLinguisticDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneQueryChangedEventArgs = __uuidof(ISearchPaneQueryChangedEventArgs);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("82fb460e-0940-4b6d-b8d0-642b30989e15")
                ISearchPaneQueryLinguisticDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_QueryTextAlternatives(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QueryTextCompositionStart(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QueryTextCompositionLength(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneQueryLinguisticDetails = __uuidof(ISearchPaneQueryLinguisticDetails);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQuerySubmittedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("143ba4fc-e9c5-4736-91b2-e8eb9cb88356")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneQuerySubmittedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_QueryText(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneQuerySubmittedEventArgs = __uuidof(ISearchPaneQuerySubmittedEventArgs);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails[] = L"Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("460c92e5-4c32-4538-a4d4-b6b4400d140f")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_LinguisticDetails(
                        ABI::Windows::ApplicationModel::Search::ISearchPaneQueryLinguisticDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails = __uuidof(ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneResultSuggestionChosenEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("c8316cc0-aed2-41e0-bce0-c26ca74f85ec")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneResultSuggestionChosenEventArgs : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneResultSuggestionChosenEventArgs = __uuidof(ISearchPaneResultSuggestionChosenEventArgs);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneStatics
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPane
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneStatics[] = L"Windows.ApplicationModel.Search.ISearchPaneStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("9572adf1-8f1d-481f-a15b-c61655f16a0e")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneStatics : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::ApplicationModel::Search::ISearchPane** searchPane
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneStatics = __uuidof(ISearchPaneStatics);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPane
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneStaticsWithHideThisApplication[] = L"Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("00732830-50f1-4d03-99ac-c6644c8ed8b5")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneStaticsWithHideThisApplication may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneStaticsWithHideThisApplication : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneStaticsWithHideThisApplication may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE HideThisApplication(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneStaticsWithHideThisApplication = __uuidof(ISearchPaneStaticsWithHideThisApplication);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneSuggestionsRequest[] = L"Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("81b10b1c-e561-4093-9b4d-2ad482794a53")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneSuggestionsRequest : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                        boolean* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SearchSuggestionCollection(
                        ABI::Windows::ApplicationModel::Search::ISearchSuggestionCollection** collection
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::Search::ISearchPaneSuggestionsRequestDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneSuggestionsRequest = __uuidof(ISearchPaneSuggestionsRequest);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("a0d009f7-8748-4ee2-ad44-afa6be997c51")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneSuggestionsRequestDeferral : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneSuggestionsRequestDeferral = __uuidof(ISearchPaneSuggestionsRequestDeferral);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("c89b8a2f-ac56-4460-8d2f-80023bec4fc5")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneSuggestionsRequestedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::ApplicationModel::Search::ISearchPaneSuggestionsRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneSuggestionsRequestedEventArgs = __uuidof(ISearchPaneSuggestionsRequestedEventArgs);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneVisibilityChangedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("3c4d3046-ac4b-49f2-97d6-020e6182cb9c")
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                DEPRECATED("ISearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                ISearchPaneVisibilityChangedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Visible(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchPaneVisibilityChangedEventArgs = __uuidof(ISearchPaneVisibilityChangedEventArgs);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchQueryLinguisticDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("46a1205b-69c9-4745-b72f-a8a4fc8f24ae")
                ISearchQueryLinguisticDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_QueryTextAlternatives(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QueryTextCompositionStart(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QueryTextCompositionLength(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchQueryLinguisticDetails = __uuidof(ISearchQueryLinguisticDetails);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchQueryLinguisticDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchQueryLinguisticDetailsFactory[] = L"Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("cac6c3b8-3c64-4dfd-ad9f-479e4d4065a4")
                ISearchQueryLinguisticDetailsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        __FIIterable_1_HSTRING* queryTextAlternatives,
                        UINT32 queryTextCompositionStart,
                        UINT32 queryTextCompositionLength,
                        ABI::Windows::ApplicationModel::Search::ISearchQueryLinguisticDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchQueryLinguisticDetailsFactory = __uuidof(ISearchQueryLinguisticDetailsFactory);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchSuggestionCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchSuggestionCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchSuggestionCollection[] = L"Windows.ApplicationModel.Search.ISearchSuggestionCollection";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("323a8a4b-fbea-4446-abbc-3da7915fdd3a")
                ISearchSuggestionCollection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AppendQuerySuggestion(
                        HSTRING text
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AppendQuerySuggestions(
                        __FIIterable_1_HSTRING* suggestions
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AppendResultSuggestion(
                        HSTRING text,
                        HSTRING detailText,
                        HSTRING tag,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* image,
                        HSTRING imageAlternateText
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AppendSearchSeparator(
                        HSTRING label
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchSuggestionCollection = __uuidof(ISearchSuggestionCollection);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchSuggestionsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchSuggestionsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchSuggestionsRequest[] = L"Windows.ApplicationModel.Search.ISearchSuggestionsRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("4e4e26a7-44e5-4039-9099-6000ead1f0c6")
                ISearchSuggestionsRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SearchSuggestionCollection(
                        ABI::Windows::ApplicationModel::Search::ISearchSuggestionCollection** collection
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::Search::ISearchSuggestionsRequestDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchSuggestionsRequest = __uuidof(ISearchSuggestionsRequest);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                MIDL_INTERFACE("b71598a9-c065-456d-a845-1eccec5dc28b")
                ISearchSuggestionsRequestDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchSuggestionsRequestDeferral = __uuidof(ISearchSuggestionsRequestDeferral);
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.LocalContentSuggestionSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ILocalContentSuggestionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_LocalContentSuggestionSettings_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_LocalContentSuggestionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_LocalContentSuggestionSettings[] = L"Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPane
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication interface starting with version 1.0 of the Windows.ApplicationModel.Search.SearchContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Search.ISearchPaneStatics interface starting with version 1.0 of the Windows.ApplicationModel.Search.SearchContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPane ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPane_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPane_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPane[] = L"Windows.ApplicationModel.Search.SearchPane";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryChangedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneQueryChangedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryLinguisticDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryLinguisticDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQuerySubmittedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQuerySubmittedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneQuerySubmittedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneResultSuggestionChosenEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneResultSuggestionChosenEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneResultSuggestionChosenEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequest_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequest[] = L"Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestDeferral_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneVisibilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneVisibilityChangedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneVisibilityChangedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchQueryLinguisticDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchQueryLinguisticDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchSuggestionCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchSuggestionCollection ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionCollection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchSuggestionCollection[] = L"Windows.ApplicationModel.Search.SearchSuggestionCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchSuggestionsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchSuggestionsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchSuggestionsRequest[] = L"Windows.ApplicationModel.Search.SearchSuggestionsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStorageFolder __FIIterator_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStorageFolder;

typedef struct __FIIterator_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIIterator_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStorageFolder __FIIterable_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStorageFolder;

typedef struct __FIIterable_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        __FIIterator_1_Windows__CStorage__CStorageFolder** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIIterable_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStorageFolder __FIVectorView_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStorageFolder;

typedef struct __FIVectorView_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIVectorView_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CStorage__CStorageFolder __FIVector_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CStorage__CStorageFolder;

typedef struct __FIVector_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        __FIVectorView_1_Windows__CStorage__CStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items);

    END_INTERFACE
} __FIVector_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIVector_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIVector_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CStorage__CStorageFolder_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CStorage__CStorageFolder_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CStorage__CStorageFolder_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CStorage__CStorageFolder_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Search.ILocalContentSuggestionSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.LocalContentSuggestionSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ILocalContentSuggestionSettings[] = L"Windows.ApplicationModel.Search.ILocalContentSuggestionSettings";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Enabled)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Locations)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        __FIVector_1_Windows__CStorage__CStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* put_AqsFilter)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AqsFilter)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PropertiesToMatch)(__x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettingsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_put_Enabled(This, value) \
    ((This)->lpVtbl->put_Enabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_get_Locations(This, value) \
    ((This)->lpVtbl->get_Locations(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_put_AqsFilter(This, value) \
    ((This)->lpVtbl->put_AqsFilter(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_get_AqsFilter(This, value) \
    ((This)->lpVtbl->get_AqsFilter(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_get_PropertiesToMatch(This, value) \
    ((This)->lpVtbl->get_PropertiesToMatch(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPane
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPane
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPane[] = L"Windows.ApplicationModel.Search.ISearchPane";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_SearchHistoryEnabled)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        boolean value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SearchHistoryEnabled)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        boolean* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_SearchHistoryContext)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SearchHistoryContext)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_PlaceholderText)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PlaceholderText)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_QueryText)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Visible)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        boolean* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_VisibilityChanged)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneVisibilityChangedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_VisibilityChanged)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        EventRegistrationToken token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_QueryChanged)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQueryChangedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_QueryChanged)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        EventRegistrationToken token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_SuggestionsRequested)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneSuggestionsRequestedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_SuggestionsRequested)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        EventRegistrationToken token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_QuerySubmitted)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneQuerySubmittedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_QuerySubmitted)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        EventRegistrationToken token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_ResultSuggestionChosen)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CSearchPane_Windows__CApplicationModel__CSearch__CSearchPaneResultSuggestionChosenEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_ResultSuggestionChosen)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        EventRegistrationToken token);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SetLocalContentSuggestionSettings)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* settings);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* ShowOverloadDefault)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* ShowOverloadWithQuery)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING query);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_ShowOnKeyboardInput)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        boolean value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ShowOnKeyboardInput)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        boolean* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* TrySetQueryText)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane* This,
        HSTRING query,
        boolean* succeeded);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_put_SearchHistoryEnabled(This, value) \
    ((This)->lpVtbl->put_SearchHistoryEnabled(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_SearchHistoryEnabled(This, value) \
    ((This)->lpVtbl->get_SearchHistoryEnabled(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_put_SearchHistoryContext(This, value) \
    ((This)->lpVtbl->put_SearchHistoryContext(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_SearchHistoryContext(This, value) \
    ((This)->lpVtbl->get_SearchHistoryContext(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_put_PlaceholderText(This, value) \
    ((This)->lpVtbl->put_PlaceholderText(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_PlaceholderText(This, value) \
    ((This)->lpVtbl->get_PlaceholderText(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_QueryText(This, value) \
    ((This)->lpVtbl->get_QueryText(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_Visible(This, value) \
    ((This)->lpVtbl->get_Visible(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_add_VisibilityChanged(This, handler, token) \
    ((This)->lpVtbl->add_VisibilityChanged(This, handler, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_remove_VisibilityChanged(This, token) \
    ((This)->lpVtbl->remove_VisibilityChanged(This, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_add_QueryChanged(This, handler, token) \
    ((This)->lpVtbl->add_QueryChanged(This, handler, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_remove_QueryChanged(This, token) \
    ((This)->lpVtbl->remove_QueryChanged(This, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_add_SuggestionsRequested(This, handler, token) \
    ((This)->lpVtbl->add_SuggestionsRequested(This, handler, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_remove_SuggestionsRequested(This, token) \
    ((This)->lpVtbl->remove_SuggestionsRequested(This, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_add_QuerySubmitted(This, handler, token) \
    ((This)->lpVtbl->add_QuerySubmitted(This, handler, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_remove_QuerySubmitted(This, token) \
    ((This)->lpVtbl->remove_QuerySubmitted(This, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_add_ResultSuggestionChosen(This, handler, token) \
    ((This)->lpVtbl->add_ResultSuggestionChosen(This, handler, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_remove_ResultSuggestionChosen(This, token) \
    ((This)->lpVtbl->remove_ResultSuggestionChosen(This, token))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_SetLocalContentSuggestionSettings(This, settings) \
    ((This)->lpVtbl->SetLocalContentSuggestionSettings(This, settings))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_ShowOverloadDefault(This) \
    ((This)->lpVtbl->ShowOverloadDefault(This))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_ShowOverloadWithQuery(This, query) \
    ((This)->lpVtbl->ShowOverloadWithQuery(This, query))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_put_ShowOnKeyboardInput(This, value) \
    ((This)->lpVtbl->put_ShowOnKeyboardInput(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_get_ShowOnKeyboardInput(This, value) \
    ((This)->lpVtbl->get_ShowOnKeyboardInput(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_TrySetQueryText(This, query, succeeded) \
    ((This)->lpVtbl->TrySetQueryText(This, query, succeeded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQueryChangedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_QueryText)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_LinguisticDetails)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_get_QueryText(This, value) \
    ((This)->lpVtbl->get_QueryText(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_get_LinguisticDetails(This, value) \
    ((This)->lpVtbl->get_LinguisticDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QueryTextAlternatives)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_QueryTextCompositionStart)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_QueryTextCompositionLength)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_get_QueryTextAlternatives(This, value) \
    ((This)->lpVtbl->get_QueryTextAlternatives(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_get_QueryTextCompositionStart(This, value) \
    ((This)->lpVtbl->get_QueryTextCompositionStart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_get_QueryTextCompositionLength(This, value) \
    ((This)->lpVtbl->get_QueryTextCompositionLength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQuerySubmittedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_QueryText)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_get_QueryText(This, value) \
    ((This)->lpVtbl->get_QueryText(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails[] = L"Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_LinguisticDetails)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_get_LinguisticDetails(This, value) \
    ((This)->lpVtbl->get_LinguisticDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneResultSuggestionChosenEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneResultSuggestionChosenEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneStatics
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPane
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneStatics[] = L"Windows.ApplicationModel.Search.ISearchPaneStatics";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPane** searchPane);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_GetForCurrentView(This, searchPane) \
    ((This)->lpVtbl->GetForCurrentView(This, searchPane))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPane
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneStaticsWithHideThisApplication[] = L"Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneStaticsWithHideThisApplication may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplicationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneStaticsWithHideThisApplication may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* HideThisApplication)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplicationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplicationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneStaticsWithHideThisApplication may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_HideThisApplication(This) \
    ((This)->lpVtbl->HideThisApplication(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneStaticsWithHideThisApplication_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneSuggestionsRequest[] = L"Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        boolean* value);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SearchSuggestionCollection)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection** collection);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_get_SearchSuggestionCollection(This, collection) \
    ((This)->lpVtbl->get_SearchSuggestionCollection(This, collection))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneSuggestionsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchPaneVisibilityChangedEventArgs[] = L"Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("ISearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Visible)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
    DEPRECATED("ISearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_get_Visible(This, value) \
    ((This)->lpVtbl->get_Visible(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneVisibilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchQueryLinguisticDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QueryTextAlternatives)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_QueryTextCompositionStart)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_QueryTextCompositionLength)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_get_QueryTextAlternatives(This, value) \
    ((This)->lpVtbl->get_QueryTextAlternatives(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_get_QueryTextCompositionStart(This, value) \
    ((This)->lpVtbl->get_QueryTextCompositionStart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_get_QueryTextCompositionLength(This, value) \
    ((This)->lpVtbl->get_QueryTextCompositionLength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchQueryLinguisticDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchQueryLinguisticDetailsFactory[] = L"Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory* This,
        __FIIterable_1_HSTRING* queryTextAlternatives,
        UINT32 queryTextCompositionStart,
        UINT32 queryTextCompositionLength,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_CreateInstance(This, queryTextAlternatives, queryTextCompositionStart, queryTextCompositionLength, value) \
    ((This)->lpVtbl->CreateInstance(This, queryTextAlternatives, queryTextCompositionStart, queryTextCompositionLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetailsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchSuggestionCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchSuggestionCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchSuggestionCollection[] = L"Windows.ApplicationModel.Search.ISearchSuggestionCollection";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* AppendQuerySuggestion)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        HSTRING text);
    HRESULT (STDMETHODCALLTYPE* AppendQuerySuggestions)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        __FIIterable_1_HSTRING* suggestions);
    HRESULT (STDMETHODCALLTYPE* AppendResultSuggestion)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        HSTRING text,
        HSTRING detailText,
        HSTRING tag,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* image,
        HSTRING imageAlternateText);
    HRESULT (STDMETHODCALLTYPE* AppendSearchSeparator)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection* This,
        HSTRING label);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollectionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_AppendQuerySuggestion(This, text) \
    ((This)->lpVtbl->AppendQuerySuggestion(This, text))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_AppendQuerySuggestions(This, suggestions) \
    ((This)->lpVtbl->AppendQuerySuggestions(This, suggestions))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_AppendResultSuggestion(This, text, detailText, tag, image, imageAlternateText) \
    ((This)->lpVtbl->AppendResultSuggestion(This, text, detailText, tag, image, imageAlternateText))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_AppendSearchSeparator(This, label) \
    ((This)->lpVtbl->AppendSearchSeparator(This, label))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchSuggestionsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchSuggestionsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchSuggestionsRequest[] = L"Windows.ApplicationModel.Search.ISearchSuggestionsRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SearchSuggestionCollection)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionCollection** collection);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_get_SearchSuggestionCollection(This, collection) \
    ((This)->lpVtbl->get_SearchSuggestionCollection(This, collection))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_ISearchSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.LocalContentSuggestionSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ILocalContentSuggestionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_LocalContentSuggestionSettings_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_LocalContentSuggestionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_LocalContentSuggestionSettings[] = L"Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPane
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication interface starting with version 1.0 of the Windows.ApplicationModel.Search.SearchContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Search.ISearchPaneStatics interface starting with version 1.0 of the Windows.ApplicationModel.Search.SearchContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPane ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPane_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPane_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPane may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPane[] = L"Windows.ApplicationModel.Search.SearchPane";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryChangedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneQueryChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneQueryChangedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryLinguisticDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQueryLinguisticDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQuerySubmittedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneQuerySubmittedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneQuerySubmittedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneQuerySubmittedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneResultSuggestionChosenEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneResultSuggestionChosenEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneResultSuggestionChosenEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneResultSuggestionChosenEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequest_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneSuggestionsRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequest[] = L"Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestDeferral_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneSuggestionsRequestDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneSuggestionsRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.SearchContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneVisibilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchPaneVisibilityChangedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
DEPRECATED("SearchPaneVisibilityChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchPaneVisibilityChangedEventArgs[] = L"Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchQueryLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchQueryLinguisticDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchQueryLinguisticDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchQueryLinguisticDetails[] = L"Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchSuggestionCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchSuggestionCollection ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionCollection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchSuggestionCollection[] = L"Windows.ApplicationModel.Search.SearchSuggestionCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchSuggestionsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchSuggestionsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchSuggestionsRequest[] = L"Windows.ApplicationModel.Search.SearchSuggestionsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_SearchSuggestionsRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_SearchSuggestionsRequestDeferral[] = L"Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
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
#endif // __windows2Eapplicationmodel2Esearch_p_h__

#endif // __windows2Eapplicationmodel2Esearch_h__
