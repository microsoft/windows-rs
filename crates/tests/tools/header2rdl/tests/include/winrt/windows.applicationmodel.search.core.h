
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
#ifndef __windows2Eapplicationmodel2Esearch2Ecore_h__
#define __windows2Eapplicationmodel2Esearch2Ecore_h__
#ifndef __windows2Eapplicationmodel2Esearch2Ecore_p_h__
#define __windows2Eapplicationmodel2Esearch2Ecore_p_h__


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

#if !defined(WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Search.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    interface IRequestingFocusOnKeyboardInputEventArgs;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs ABI::Windows::ApplicationModel::Search::Core::IRequestingFocusOnKeyboardInputEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    interface ISearchSuggestion;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    interface ISearchSuggestionManager;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestionManager

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    interface ISearchSuggestionsRequestedEventArgs;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestionsRequestedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    class SearchSuggestion;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c3157582-f72f-513a-b089-6208188ec2b6"))
IIterator<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Search.Core.SearchSuggestion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t;
#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("df956160-f155-5816-b38b-a2ff0629ba0e"))
IIterable<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Search.Core.SearchSuggestion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t;
#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05ac2ec0-f0c8-5cad-b4f9-f985e0f79fe9"))
IVectorView<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Search.Core.SearchSuggestion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t;
#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1cc36c46-19be-5d6b-a56d-047413252c69"))
IVector<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.Search.Core.SearchSuggestion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t;
#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#define DEF___FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb7b988b-468a-5eae-afe4-df8b005f80af"))
VectorChangedEventHandler<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> : VectorChangedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.VectorChangedEventHandler`1<Windows.ApplicationModel.Search.Core.SearchSuggestion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef VectorChangedEventHandler<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t;
#define __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion ABI::Windows::Foundation::Collections::__FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#define DEF___FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e885956c-72c1-514f-ad9a-2943b880ae13"))
IObservableVector<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> : IObservableVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableVector`1<Windows.ApplicationModel.Search.Core.SearchSuggestion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableVector<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestion*> __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t;
#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion ABI::Windows::Foundation::Collections::__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    class SearchSuggestionManager;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    class RequestingFocusOnKeyboardInputEventArgs;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bb198760-4739-5559-80e6-f0e1af5355fd"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionManager*, ABI::Windows::ApplicationModel::Search::Core::RequestingFocusOnKeyboardInputEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionManager*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestionManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::RequestingFocusOnKeyboardInputEventArgs*, ABI::Windows::ApplicationModel::Search::Core::IRequestingFocusOnKeyboardInputEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.Core.SearchSuggestionManager, Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionManager*, ABI::Windows::ApplicationModel::Search::Core::RequestingFocusOnKeyboardInputEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    class SearchSuggestionsRequestedEventArgs;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7e7f3cdf-80d9-5646-8743-ec1e38645d68"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionManager*, ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionManager*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestionManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionsRequestedEventArgs*, ABI::Windows::ApplicationModel::Search::Core::ISearchSuggestionsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Search.Core.SearchSuggestionManager, Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionManager*, ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class LocalContentSuggestionSettings;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchSuggestionsRequest;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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
                namespace Core {
                    typedef enum SearchSuggestionKind : int SearchSuggestionKind;
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Search.Core.SearchSuggestionKind
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    enum SearchSuggestionKind : int
                    {
                        SearchSuggestionKind_Query = 0,
                        SearchSuggestionKind_Result = 1,
                        SearchSuggestionKind_Separator = 2,
                    };
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_IRequestingFocusOnKeyboardInputEventArgs[] = L"Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    MIDL_INTERFACE("a1195f27-b1a7-41a2-879d-6a68687e5985")
                    IRequestingFocusOnKeyboardInputEventArgs : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IRequestingFocusOnKeyboardInputEventArgs = __uuidof(IRequestingFocusOnKeyboardInputEventArgs);
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.ISearchSuggestion
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.SearchSuggestion
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_ISearchSuggestion[] = L"Windows.ApplicationModel.Search.Core.ISearchSuggestion";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    MIDL_INTERFACE("5b5554b0-1527-437b-95c5-8d18d2b8af55")
                    ISearchSuggestion : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::ApplicationModel::Search::Core::SearchSuggestionKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Text(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Tag(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DetailText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Image(
                            ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ImageAlternateText(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISearchSuggestion = __uuidof(ISearchSuggestion);
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.ISearchSuggestionManager
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.SearchSuggestionManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_ISearchSuggestionManager[] = L"Windows.ApplicationModel.Search.Core.ISearchSuggestionManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    MIDL_INTERFACE("3f0c50a1-cb9d-497b-b500-3c04ac959ad2")
                    ISearchSuggestionManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SearchHistoryEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SearchHistoryEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SearchHistoryContext(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SearchHistoryContext(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLocalContentSuggestionSettings(
                            ABI::Windows::ApplicationModel::Search::ILocalContentSuggestionSettings* settings
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetQuery(
                            HSTRING queryText
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetQueryWithLanguage(
                            HSTRING queryText,
                            HSTRING language
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetQueryWithSearchQueryLinguisticDetails(
                            HSTRING queryText,
                            HSTRING language,
                            ABI::Windows::ApplicationModel::Search::ISearchQueryLinguisticDetails* linguisticDetails
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Suggestions(
                            __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddToHistory(
                            HSTRING queryText
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddToHistoryWithLanguage(
                            HSTRING queryText,
                            HSTRING language
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ClearHistory(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SuggestionsRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SuggestionsRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_RequestingFocusOnKeyboardInput(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_RequestingFocusOnKeyboardInput(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISearchSuggestionManager = __uuidof(ISearchSuggestionManager);
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_ISearchSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                namespace Core {
                    MIDL_INTERFACE("6fd519e5-9e7e-4ab4-8be3-c76b1bd4344a")
                    ISearchSuggestionsRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_QueryText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Language(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LinguisticDetails(
                            ABI::Windows::ApplicationModel::Search::ISearchQueryLinguisticDetails** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Search::ISearchSuggestionsRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISearchSuggestionsRequestedEventArgs = __uuidof(ISearchSuggestionsRequestedEventArgs);
                } /* Core */
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_RequestingFocusOnKeyboardInputEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_RequestingFocusOnKeyboardInputEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_RequestingFocusOnKeyboardInputEventArgs[] = L"Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.SearchSuggestion
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.ISearchSuggestion ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestion_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_SearchSuggestion[] = L"Windows.ApplicationModel.Search.Core.SearchSuggestion";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.SearchSuggestionManager
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.Search.Core.SearchCoreContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.ISearchSuggestionManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_SearchSuggestionManager[] = L"Windows.ApplicationModel.Search.Core.SearchSuggestionManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_SearchSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

typedef struct __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

typedef struct __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __FIIterator_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

typedef struct __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __FIVectorView_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl;

interface __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

typedef interface __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__)
#define ____FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__

typedef interface __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

typedef struct __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* sender,
        __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs* event);

    END_INTERFACE
} __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl;

interface __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion
{
    CONST_VTBL struct __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__)
#define ____FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__

typedef interface __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion;

typedef struct __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_VectorChanged)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        __FVectorChangedEventHandler_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_VectorChanged)(__FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl;

interface __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion
{
    CONST_VTBL struct __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_add_VectorChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_VectorChanged(This, vhnd, result))

#define __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_remove_VectorChanged(This, token) \
    ((This)->lpVtbl->remove_VectorChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* sender,
        __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CSearchSuggestionKind __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CSearchSuggestionKind;

/*
 *
 * Struct Windows.ApplicationModel.Search.Core.SearchSuggestionKind
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CSearchSuggestionKind
{
    SearchSuggestionKind_Query = 0,
    SearchSuggestionKind_Result = 1,
    SearchSuggestionKind_Separator = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_IRequestingFocusOnKeyboardInputEventArgs[] = L"Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CIRequestingFocusOnKeyboardInputEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.ISearchSuggestion
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.SearchSuggestion
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_ISearchSuggestion[] = L"Windows.ApplicationModel.Search.Core.ISearchSuggestion";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        enum __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CSearchSuggestionKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DetailText)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Image)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_ImageAlternateText)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_get_DetailText(This, value) \
    ((This)->lpVtbl->get_DetailText(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_get_Image(This, value) \
    ((This)->lpVtbl->get_Image(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_get_ImageAlternateText(This, value) \
    ((This)->lpVtbl->get_ImageAlternateText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestion_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.ISearchSuggestionManager
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.SearchSuggestionManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_ISearchSuggestionManager[] = L"Windows.ApplicationModel.Search.Core.ISearchSuggestionManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SearchHistoryEnabled)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SearchHistoryEnabled)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SearchHistoryContext)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SearchHistoryContext)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* SetLocalContentSuggestionSettings)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CILocalContentSuggestionSettings* settings);
    HRESULT (STDMETHODCALLTYPE* SetQuery)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING queryText);
    HRESULT (STDMETHODCALLTYPE* SetQueryWithLanguage)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING queryText,
        HSTRING language);
    HRESULT (STDMETHODCALLTYPE* SetQueryWithSearchQueryLinguisticDetails)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING queryText,
        HSTRING language,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails* linguisticDetails);
    HRESULT (STDMETHODCALLTYPE* get_Suggestions)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        __FIObservableVector_1_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestion** value);
    HRESULT (STDMETHODCALLTYPE* AddToHistory)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING queryText);
    HRESULT (STDMETHODCALLTYPE* AddToHistoryWithLanguage)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        HSTRING queryText,
        HSTRING language);
    HRESULT (STDMETHODCALLTYPE* ClearHistory)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This);
    HRESULT (STDMETHODCALLTYPE* add_SuggestionsRequested)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionsRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SuggestionsRequested)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RequestingFocusOnKeyboardInput)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CSearch__CCore__CSearchSuggestionManager_Windows__CApplicationModel__CSearch__CCore__CRequestingFocusOnKeyboardInputEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RequestingFocusOnKeyboardInput)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_get_SearchHistoryEnabled(This, value) \
    ((This)->lpVtbl->get_SearchHistoryEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_put_SearchHistoryEnabled(This, value) \
    ((This)->lpVtbl->put_SearchHistoryEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_get_SearchHistoryContext(This, value) \
    ((This)->lpVtbl->get_SearchHistoryContext(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_put_SearchHistoryContext(This, value) \
    ((This)->lpVtbl->put_SearchHistoryContext(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_SetLocalContentSuggestionSettings(This, settings) \
    ((This)->lpVtbl->SetLocalContentSuggestionSettings(This, settings))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_SetQuery(This, queryText) \
    ((This)->lpVtbl->SetQuery(This, queryText))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_SetQueryWithLanguage(This, queryText, language) \
    ((This)->lpVtbl->SetQueryWithLanguage(This, queryText, language))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_SetQueryWithSearchQueryLinguisticDetails(This, queryText, language, linguisticDetails) \
    ((This)->lpVtbl->SetQueryWithSearchQueryLinguisticDetails(This, queryText, language, linguisticDetails))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_get_Suggestions(This, value) \
    ((This)->lpVtbl->get_Suggestions(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_AddToHistory(This, queryText) \
    ((This)->lpVtbl->AddToHistory(This, queryText))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_AddToHistoryWithLanguage(This, queryText, language) \
    ((This)->lpVtbl->AddToHistoryWithLanguage(This, queryText, language))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_ClearHistory(This) \
    ((This)->lpVtbl->ClearHistory(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_add_SuggestionsRequested(This, handler, token) \
    ((This)->lpVtbl->add_SuggestionsRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_remove_SuggestionsRequested(This, token) \
    ((This)->lpVtbl->remove_SuggestionsRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_add_RequestingFocusOnKeyboardInput(This, handler, token) \
    ((This)->lpVtbl->add_RequestingFocusOnKeyboardInput(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_remove_RequestingFocusOnKeyboardInput(This, token) \
    ((This)->lpVtbl->remove_RequestingFocusOnKeyboardInput(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Search_Core_ISearchSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QueryText)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LinguisticDetails)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchQueryLinguisticDetails** value);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchSuggestionsRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_get_QueryText(This, value) \
    ((This)->lpVtbl->get_QueryText(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_get_LinguisticDetails(This, value) \
    ((This)->lpVtbl->get_LinguisticDetails(This, value))

#define __x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSearch_CCore_CISearchSuggestionsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_RequestingFocusOnKeyboardInputEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_RequestingFocusOnKeyboardInputEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_RequestingFocusOnKeyboardInputEventArgs[] = L"Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.SearchSuggestion
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.ISearchSuggestion ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestion_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_SearchSuggestion[] = L"Windows.ApplicationModel.Search.Core.SearchSuggestion";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.SearchSuggestionManager
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.Search.Core.SearchCoreContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.ISearchSuggestionManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_SearchSuggestionManager[] = L"Windows.ApplicationModel.Search.Core.SearchSuggestionManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Search.Core.SearchCoreContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Search_Core_SearchSuggestionsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Search_Core_SearchSuggestionsRequestedEventArgs[] = L"Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SEARCH_CORE_SEARCHCORECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Esearch2Ecore_p_h__

#endif // __windows2Eapplicationmodel2Esearch2Ecore_h__
