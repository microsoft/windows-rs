
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
#ifndef __windows2Eglobalization_h__
#define __windows2Eglobalization_h__
#ifndef __windows2Eglobalization_p_h__
#define __windows2Eglobalization_p_h__


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

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

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
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IApplicationLanguagesStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics ABI::Windows::Globalization::IApplicationLanguagesStatics

#endif // ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IApplicationLanguagesStatics2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2 ABI::Windows::Globalization::IApplicationLanguagesStatics2

#endif // ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICalendar;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICalendar ABI::Windows::Globalization::ICalendar

#endif // ____x_ABI_CWindows_CGlobalization_CICalendar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICalendarFactory;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICalendarFactory ABI::Windows::Globalization::ICalendarFactory

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICalendarFactory2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2 ABI::Windows::Globalization::ICalendarFactory2

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICalendarIdentifiersStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics ABI::Windows::Globalization::ICalendarIdentifiersStatics

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICalendarIdentifiersStatics2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2 ABI::Windows::Globalization::ICalendarIdentifiersStatics2

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICalendarIdentifiersStatics3;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3 ABI::Windows::Globalization::ICalendarIdentifiersStatics3

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IClockIdentifiersStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics ABI::Windows::Globalization::IClockIdentifiersStatics

#endif // ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICurrencyAmount;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount ABI::Windows::Globalization::ICurrencyAmount

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICurrencyAmountFactory;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory ABI::Windows::Globalization::ICurrencyAmountFactory

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICurrencyIdentifiersStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics ABI::Windows::Globalization::ICurrencyIdentifiersStatics

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICurrencyIdentifiersStatics2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2 ABI::Windows::Globalization::ICurrencyIdentifiersStatics2

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ICurrencyIdentifiersStatics3;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3 ABI::Windows::Globalization::ICurrencyIdentifiersStatics3

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IGeographicRegion;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion ABI::Windows::Globalization::IGeographicRegion

#endif // ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IGeographicRegionFactory;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory ABI::Windows::Globalization::IGeographicRegionFactory

#endif // ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IGeographicRegionStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics ABI::Windows::Globalization::IGeographicRegionStatics

#endif // ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IJapanesePhoneme;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme ABI::Windows::Globalization::IJapanesePhoneme

#endif // ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface IJapanesePhoneticAnalyzerStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics ABI::Windows::Globalization::IJapanesePhoneticAnalyzerStatics

#endif // ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguage;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguage ABI::Windows::Globalization::ILanguage

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguage2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguage2 ABI::Windows::Globalization::ILanguage2

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguage3;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguage3 ABI::Windows::Globalization::ILanguage3

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguageExtensionSubtags;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags ABI::Windows::Globalization::ILanguageExtensionSubtags

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguageFactory;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguageFactory ABI::Windows::Globalization::ILanguageFactory

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguageStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguageStatics ABI::Windows::Globalization::ILanguageStatics

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguageStatics2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2 ABI::Windows::Globalization::ILanguageStatics2

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguageStatics3;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3 ABI::Windows::Globalization::ILanguageStatics3

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface INumeralSystemIdentifiersStatics;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics ABI::Windows::Globalization::INumeralSystemIdentifiersStatics

#endif // ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface INumeralSystemIdentifiersStatics2;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2 ABI::Windows::Globalization::INumeralSystemIdentifiersStatics2

#endif // ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ITimeZoneOnCalendar;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar ABI::Windows::Globalization::ITimeZoneOnCalendar

#endif // ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_FWD_DEFINED__

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
        namespace Globalization {
            class JapanesePhoneme;
        } /* Globalization */
    } /* Windows */
} /* ABI */

#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_USE
#define DEF___FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f15ca7e7-69a8-564d-9c20-4da75a773432"))
IIterator<ABI::Windows::Globalization::JapanesePhoneme*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::JapanesePhoneme*, ABI::Windows::Globalization::IJapanesePhoneme*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Globalization.JapanesePhoneme>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Globalization::JapanesePhoneme*> __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_t;
#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_USE */

#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_USE
#define DEF___FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1aad17cb-1829-5236-8aef-0b75f8dfd7a6"))
IIterable<ABI::Windows::Globalization::JapanesePhoneme*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::JapanesePhoneme*, ABI::Windows::Globalization::IJapanesePhoneme*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Globalization.JapanesePhoneme>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Globalization::JapanesePhoneme*> __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_t;
#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_USE */

#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000


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


#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_USE
#define DEF___FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4cdc5bd0-d4aa-5b60-bf25-7144905050f9"))
IVectorView<ABI::Windows::Globalization::JapanesePhoneme*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::JapanesePhoneme*, ABI::Windows::Globalization::IJapanesePhoneme*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.JapanesePhoneme>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Globalization::JapanesePhoneme*> __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_t;
#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_USE */

#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000


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


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
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
        namespace Globalization {
            typedef enum DayOfWeek : int DayOfWeek;
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            typedef enum LanguageLayoutDirection : int LanguageLayoutDirection;
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class Calendar;
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class CurrencyAmount;
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class GeographicRegion;
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class Language;
        } /* Globalization */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Globalization.DayOfWeek
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            enum DayOfWeek : int
            {
                DayOfWeek_Sunday = 0,
                DayOfWeek_Monday = 1,
                DayOfWeek_Tuesday = 2,
                DayOfWeek_Wednesday = 3,
                DayOfWeek_Thursday = 4,
                DayOfWeek_Friday = 5,
                DayOfWeek_Saturday = 6,
            };
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.LanguageLayoutDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            enum LanguageLayoutDirection : int
            {
                LanguageLayoutDirection_Ltr = 0,
                LanguageLayoutDirection_Rtl = 1,
                LanguageLayoutDirection_TtbLtr = 2,
                LanguageLayoutDirection_TtbRtl = 3,
            };
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Globalization.IApplicationLanguagesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.ApplicationLanguages
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IApplicationLanguagesStatics[] = L"Windows.Globalization.IApplicationLanguagesStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("75b40847-0a4c-4a92-9565-fd63c95f7aed")
            IApplicationLanguagesStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_PrimaryLanguageOverride(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_PrimaryLanguageOverride(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Languages(
                    __FIVectorView_1_HSTRING** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ManifestLanguages(
                    __FIVectorView_1_HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationLanguagesStatics = __uuidof(IApplicationLanguagesStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IApplicationLanguagesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.ApplicationLanguages
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IApplicationLanguagesStatics2[] = L"Windows.Globalization.IApplicationLanguagesStatics2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("1df0de4f-072b-4d7b-8f06-cb2db40f2bb5")
            IApplicationLanguagesStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetLanguagesForUser(
                    ABI::Windows::System::IUser* user,
                    __FIVectorView_1_HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationLanguagesStatics2 = __uuidof(IApplicationLanguagesStatics2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Globalization.ICalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendar[] = L"Windows.Globalization.ICalendar";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("ca30221d-86d9-40fb-a26b-d44eb7cf08ea")
            ICalendar : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Clone(
                    ABI::Windows::Globalization::ICalendar** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SetToMin(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE SetToMax(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Languages(
                    __FIVectorView_1_HSTRING** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumeralSystem(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_NumeralSystem(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetCalendarSystem(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ChangeCalendarSystem(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetClock(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ChangeClock(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDateTime(
                    ABI::Windows::Foundation::DateTime* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SetDateTime(
                    ABI::Windows::Foundation::DateTime value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SetToNow(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstEra(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastEra(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfEras(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Era(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Era(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddEras(
                    INT32 eras
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE EraAsFullString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE EraAsString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstYearInThisEra(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastYearInThisEra(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfYearsInThisEra(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Year(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Year(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddYears(
                    INT32 years
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE YearAsString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE YearAsTruncatedString(
                    INT32 remainingDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE YearAsPaddedString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstMonthInThisYear(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastMonthInThisYear(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfMonthsInThisYear(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Month(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Month(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddMonths(
                    INT32 months
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MonthAsFullString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MonthAsString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MonthAsFullSoloString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MonthAsSoloString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MonthAsNumericString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MonthAsPaddedNumericString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddWeeks(
                    INT32 weeks
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstDayInThisMonth(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastDayInThisMonth(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfDaysInThisMonth(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Day(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Day(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddDays(
                    INT32 days
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DayAsString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DayAsPaddedString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DayOfWeek(
                    ABI::Windows::Globalization::DayOfWeek* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DayOfWeekAsFullString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DayOfWeekAsString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DayOfWeekAsFullSoloString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DayOfWeekAsSoloString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstPeriodInThisDay(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastPeriodInThisDay(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfPeriodsInThisDay(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Period(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Period(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddPeriods(
                    INT32 periods
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE PeriodAsFullString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE PeriodAsString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstHourInThisPeriod(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastHourInThisPeriod(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfHoursInThisPeriod(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Hour(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Hour(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddHours(
                    INT32 hours
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE HourAsString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE HourAsPaddedString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Minute(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Minute(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddMinutes(
                    INT32 minutes
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MinuteAsString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MinuteAsPaddedString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Second(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Second(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddSeconds(
                    INT32 seconds
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SecondAsString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SecondAsPaddedString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Nanosecond(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Nanosecond(
                    INT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddNanoseconds(
                    INT32 nanoseconds
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE NanosecondAsString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE NanosecondAsPaddedString(
                    INT32 minDigits,
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Compare(
                    ABI::Windows::Globalization::ICalendar* other,
                    INT32* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CompareDateTime(
                    ABI::Windows::Foundation::DateTime other,
                    INT32* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyTo(
                    ABI::Windows::Globalization::ICalendar* other
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstMinuteInThisHour(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastMinuteInThisHour(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfMinutesInThisHour(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FirstSecondInThisMinute(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastSecondInThisMinute(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NumberOfSecondsInThisMinute(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsDaylightSavingTime(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICalendar = __uuidof(ICalendar);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendar;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarFactory[] = L"Windows.Globalization.ICalendarFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("83f58412-e56b-4c75-a66e-0f63d57758a6")
            ICalendarFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateCalendarDefaultCalendarAndClock(
                    __FIIterable_1_HSTRING* languages,
                    ABI::Windows::Globalization::ICalendar** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateCalendar(
                    __FIIterable_1_HSTRING* languages,
                    HSTRING calendar,
                    HSTRING clock,
                    ABI::Windows::Globalization::ICalendar** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICalendarFactory = __uuidof(ICalendarFactory);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarFactory2[] = L"Windows.Globalization.ICalendarFactory2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("b44b378c-ca7e-4590-9e72-ea2bec1a5115")
            ICalendarFactory2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateCalendarWithTimeZone(
                    __FIIterable_1_HSTRING* languages,
                    HSTRING calendar,
                    HSTRING clock,
                    HSTRING timeZoneId,
                    ABI::Windows::Globalization::ICalendar** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICalendarFactory2 = __uuidof(ICalendarFactory2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarFactory2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CalendarIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarIdentifiersStatics[] = L"Windows.Globalization.ICalendarIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("80653f68-2cb2-4c1f-b590-f0f52bf4fd1a")
            ICalendarIdentifiersStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Gregorian(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Hebrew(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Hijri(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Japanese(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Julian(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Korean(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Taiwan(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Thai(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UmAlQura(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICalendarIdentifiersStatics = __uuidof(ICalendarIdentifiersStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CalendarIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarIdentifiersStatics2[] = L"Windows.Globalization.ICalendarIdentifiersStatics2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("7df4d488-5fd0-42a7-95b5-7d98d823075f")
            ICalendarIdentifiersStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Persian(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICalendarIdentifiersStatics2 = __uuidof(ICalendarIdentifiersStatics2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarIdentifiersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CalendarIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarIdentifiersStatics3[] = L"Windows.Globalization.ICalendarIdentifiersStatics3";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("2c225423-1fad-40c0-9334-a8eb90db04f5")
            ICalendarIdentifiersStatics3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ChineseLunar(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_JapaneseLunar(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KoreanLunar(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TaiwanLunar(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VietnameseLunar(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICalendarIdentifiersStatics3 = __uuidof(ICalendarIdentifiersStatics3);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Globalization.IClockIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.ClockIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IClockIdentifiersStatics[] = L"Windows.Globalization.IClockIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("523805bb-12ec-4f83-bc31-b1b4376b0808")
            IClockIdentifiersStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_TwelveHour(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TwentyFourHour(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IClockIdentifiersStatics = __uuidof(IClockIdentifiersStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyAmount[] = L"Windows.Globalization.ICurrencyAmount";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("74b49942-eb75-443a-95b3-7d723f56f93c")
            ICurrencyAmount : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Amount(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Currency(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICurrencyAmount = __uuidof(ICurrencyAmount);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyAmount;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Globalization.ICurrencyAmountFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyAmountFactory[] = L"Windows.Globalization.ICurrencyAmountFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("48d7168f-ef3b-4aee-a6a1-4b036fe03ff0")
            ICurrencyAmountFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Create(
                    HSTRING amount,
                    HSTRING currency,
                    ABI::Windows::Globalization::ICurrencyAmount** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICurrencyAmountFactory = __uuidof(ICurrencyAmountFactory);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Globalization.ICurrencyIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyIdentifiersStatics[] = L"Windows.Globalization.ICurrencyIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("9f1d091b-d586-4913-9b6a-a9bd2dc12874")
            ICurrencyIdentifiersStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AED(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AFN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ALL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AMD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ANG(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AOA(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ARS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AUD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AWG(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AZN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BAM(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BBD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BDT(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BGN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BHD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BIF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BMD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BND(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BOB(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BRL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BSD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BTN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BWP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BYR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BZD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CAD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CDF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CHF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CLP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CNY(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_COP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CRC(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CUP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CVE(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CZK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DJF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DKK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DOP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DZD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EGP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ERN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ETB(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EUR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FJD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FKP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GBP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GEL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GHS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GIP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GMD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GNF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GTQ(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GYD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HKD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HNL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HRK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HTG(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HUF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IDR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ILS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_INR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IQD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IRR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ISK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_JMD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_JOD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_JPY(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KES(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KGS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KHR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KMF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KPW(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KRW(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KWD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KYD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_KZT(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LAK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LBP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LKR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LRD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LSL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LTL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LVL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LYD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MAD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MDL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MGA(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MKD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MMK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MNT(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MOP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MRO(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MUR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MVR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MWK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MXN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MYR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MZN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NAD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NGN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NIO(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NOK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NPR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NZD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_OMR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PAB(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PEN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PGK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PHP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PKR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PLN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PYG(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_QAR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RON(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RSD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RUB(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RWF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SAR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SBD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SCR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SDG(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SEK(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SGD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SHP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SLL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SOS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SRD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_STD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SYP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SZL(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_THB(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TJS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TMT(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TND(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TOP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TRY(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TTD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TWD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TZS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UAH(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UGX(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_USD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UYU(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UZS(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VEF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VND(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VUV(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_WST(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_XAF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_XCD(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_XOF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_XPF(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_XXX(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_YER(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZAR(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZMW(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZWL(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICurrencyIdentifiersStatics = __uuidof(ICurrencyIdentifiersStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICurrencyIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyIdentifiersStatics2[] = L"Windows.Globalization.ICurrencyIdentifiersStatics2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("1814797f-c3b2-4c33-9591-980011950d37")
            ICurrencyIdentifiersStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_BYN(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICurrencyIdentifiersStatics2 = __uuidof(ICurrencyIdentifiersStatics2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Globalization.ICurrencyIdentifiersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyIdentifiersStatics3[] = L"Windows.Globalization.ICurrencyIdentifiersStatics3";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("4fb23bfa-ed25-4f4d-857f-237f1748c21c")
            ICurrencyIdentifiersStatics3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_MRU(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SSP(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_STN(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VES(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICurrencyIdentifiersStatics3 = __uuidof(ICurrencyIdentifiersStatics3);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Globalization.IGeographicRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.GeographicRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IGeographicRegion[] = L"Windows.Globalization.IGeographicRegion";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("01e9a621-4a64-4ed9-954f-9edeb07bd903")
            IGeographicRegion : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Code(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CodeTwoLetter(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CodeThreeLetter(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CodeThreeDigit(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NativeName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CurrenciesInUse(
                    __FIVectorView_1_HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IGeographicRegion = __uuidof(IGeographicRegion);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIGeographicRegion;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IGeographicRegionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.GeographicRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IGeographicRegionFactory[] = L"Windows.Globalization.IGeographicRegionFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("53425270-77b4-426b-859f-81e19d512546")
            IGeographicRegionFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateGeographicRegion(
                    HSTRING geographicRegionCode,
                    ABI::Windows::Globalization::IGeographicRegion** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IGeographicRegionFactory = __uuidof(IGeographicRegionFactory);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IGeographicRegionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.GeographicRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IGeographicRegionStatics[] = L"Windows.Globalization.IGeographicRegionStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("29e28974-7ad9-4ef4-8799-b3b44fadec08")
            IGeographicRegionStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE IsSupported(
                    HSTRING geographicRegionCode,
                    boolean* result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IGeographicRegionStatics = __uuidof(IGeographicRegionStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IJapanesePhoneme
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.JapanesePhoneme
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IJapanesePhoneme[] = L"Windows.Globalization.IJapanesePhoneme";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("2f6a9300-e85b-43e6-897d-5d82f862df21")
            IJapanesePhoneme : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_DisplayText(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_YomiText(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsPhraseStart(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IJapanesePhoneme = __uuidof(IJapanesePhoneme);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIJapanesePhoneme;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_INTERFACE_DEFINED__) */
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IJapanesePhoneticAnalyzerStatics
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.JapanesePhoneticAnalyzer
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IJapanesePhoneticAnalyzerStatics[] = L"Windows.Globalization.IJapanesePhoneticAnalyzerStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("88ab9e90-93de-41b2-b4d5-8edb227fd1c2")
            IJapanesePhoneticAnalyzerStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetWords(
                    HSTRING input,
                    __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetWordsWithMonoRubyOption(
                    HSTRING input,
                    boolean monoRuby,
                    __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IJapanesePhoneticAnalyzerStatics = __uuidof(IJapanesePhoneticAnalyzerStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguage[] = L"Windows.Globalization.ILanguage";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("ea79a752-f7c2-4265-b1bd-c4dec4e4f080")
            ILanguage : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_LanguageTag(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NativeName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Script(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguage = __uuidof(ILanguage);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguage;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguage2[] = L"Windows.Globalization.ILanguage2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("6a47e5b5-d94d-4886-a404-a5a5b9d5b494")
            ILanguage2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_LayoutDirection(
                    ABI::Windows::Globalization::LanguageLayoutDirection* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguage2 = __uuidof(ILanguage2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguage2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Globalization.ILanguage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguage3[] = L"Windows.Globalization.ILanguage3";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("c6af3d10-641a-5ba4-bb43-5e12aed75954")
            ILanguage3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AbbreviatedName(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguage3 = __uuidof(ILanguage3);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguage3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Globalization.ILanguageExtensionSubtags
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageExtensionSubtags[] = L"Windows.Globalization.ILanguageExtensionSubtags";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("7d7daf45-368d-4364-852b-dec927037b85")
            ILanguageExtensionSubtags : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetExtensionSubtags(
                    HSTRING singleton,
                    __FIVectorView_1_HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguageExtensionSubtags = __uuidof(ILanguageExtensionSubtags);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageFactory[] = L"Windows.Globalization.ILanguageFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("9b0252ac-0c27-44f8-b792-9793fb66c63e")
            ILanguageFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateLanguage(
                    HSTRING languageTag,
                    ABI::Windows::Globalization::ILanguage** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguageFactory = __uuidof(ILanguageFactory);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageStatics[] = L"Windows.Globalization.ILanguageStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("b23cd557-0865-46d4-89b8-d59be8990f0d")
            ILanguageStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE IsWellFormed(
                    HSTRING languageTag,
                    boolean* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CurrentInputMethodLanguageTag(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguageStatics = __uuidof(ILanguageStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageStatics2[] = L"Windows.Globalization.ILanguageStatics2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("30199f6e-914b-4b2a-9d6e-e3b0e27dbe4f")
            ILanguageStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE TrySetInputMethodLanguageTag(
                    HSTRING languageTag,
                    boolean* result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguageStatics2 = __uuidof(ILanguageStatics2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageStatics3[] = L"Windows.Globalization.ILanguageStatics3";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("d15ecb5a-71de-5752-9542-fac5b4f27261")
            ILanguageStatics3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetMuiCompatibleLanguageListFromLanguageTags(
                    __FIIterable_1_HSTRING* languageTags,
                    __FIVector_1_HSTRING** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILanguageStatics3 = __uuidof(ILanguageStatics3);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageStatics3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Globalization.INumeralSystemIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumeralSystemIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_INumeralSystemIdentifiersStatics[] = L"Windows.Globalization.INumeralSystemIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("a5c662c3-68c9-4d3d-b765-972029e21dec")
            INumeralSystemIdentifiersStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Arab(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ArabExt(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Bali(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Beng(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Cham(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Deva(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FullWide(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Gujr(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Guru(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HaniDec(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Java(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Kali(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Khmr(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Knda(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Lana(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LanaTham(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Laoo(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Latn(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Lepc(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Limb(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Mlym(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Mong(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Mtei(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Mymr(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MymrShan(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Nkoo(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Olck(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Orya(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Saur(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Sund(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Talu(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TamlDec(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Telu(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Thai(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Tibt(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Vaii(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_INumeralSystemIdentifiersStatics = __uuidof(INumeralSystemIdentifiersStatics);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.INumeralSystemIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumeralSystemIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_INumeralSystemIdentifiersStatics2[] = L"Windows.Globalization.INumeralSystemIdentifiersStatics2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("7f003228-9ddb-4a34-9104-0260c091a7c7")
            INumeralSystemIdentifiersStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Brah(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Osma(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MathBold(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MathDbl(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MathSans(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MathSanb(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MathMono(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZmthBold(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZmthDbl(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZmthSans(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZmthSanb(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ZmthMono(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_INumeralSystemIdentifiersStatics2 = __uuidof(INumeralSystemIdentifiersStatics2);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ITimeZoneOnCalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ITimeZoneOnCalendar[] = L"Windows.Globalization.ITimeZoneOnCalendar";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            MIDL_INTERFACE("bb3c25e5-46cf-4317-a3f5-02621ad54478")
            ITimeZoneOnCalendar : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetTimeZone(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ChangeTimeZone(
                    HSTRING timeZoneId
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE TimeZoneAsFullString(
                    HSTRING* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE TimeZoneAsString(
                    INT32 idealLength,
                    HSTRING* result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ITimeZoneOnCalendar = __uuidof(ITimeZoneOnCalendar);
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.ApplicationLanguages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IApplicationLanguagesStatics2 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.IApplicationLanguagesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_ApplicationLanguages_DEFINED
#define RUNTIMECLASS_Windows_Globalization_ApplicationLanguages_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_ApplicationLanguages[] = L"Windows.Globalization.ApplicationLanguages";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Calendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.ICalendarFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.ICalendarFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.ICalendar ** Default Interface **
 *    Windows.Globalization.ITimeZoneOnCalendar
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Calendar_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Calendar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Calendar[] = L"Windows.Globalization.Calendar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.CalendarIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.ICalendarIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICalendarIdentifiersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICalendarIdentifiersStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_CalendarIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_CalendarIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_CalendarIdentifiers[] = L"Windows.Globalization.CalendarIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.ClockIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IClockIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_ClockIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_ClockIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_ClockIdentifiers[] = L"Windows.Globalization.ClockIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.CurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.ICurrencyAmountFactory interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.ICurrencyAmount ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Globalization_CurrencyAmount_DEFINED
#define RUNTIMECLASS_Windows_Globalization_CurrencyAmount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_CurrencyAmount[] = L"Windows.Globalization.CurrencyAmount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Globalization.CurrencyIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.ICurrencyIdentifiersStatics3 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICurrencyIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICurrencyIdentifiersStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_CurrencyIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_CurrencyIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_CurrencyIdentifiers[] = L"Windows.Globalization.CurrencyIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.GeographicRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.IGeographicRegionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IGeographicRegionStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.IGeographicRegion ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_GeographicRegion_DEFINED
#define RUNTIMECLASS_Windows_Globalization_GeographicRegion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_GeographicRegion[] = L"Windows.Globalization.GeographicRegion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.JapanesePhoneme
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.IJapanesePhoneme ** Default Interface **
 *
 * Class Threading Model:  Single Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_JapanesePhoneme_DEFINED
#define RUNTIMECLASS_Windows_Globalization_JapanesePhoneme_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_JapanesePhoneme[] = L"Windows.Globalization.JapanesePhoneme";
#endif
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.JapanesePhoneticAnalyzer
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IJapanesePhoneticAnalyzerStatics interface starting with version 1.0 of the Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract API contract
 *
 * Class Threading Model:  Single Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_JapanesePhoneticAnalyzer_DEFINED
#define RUNTIMECLASS_Windows_Globalization_JapanesePhoneticAnalyzer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_JapanesePhoneticAnalyzer[] = L"Windows.Globalization.JapanesePhoneticAnalyzer";
#endif
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Language
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.ILanguageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.ILanguageStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ILanguageStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ILanguageStatics3 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.ILanguage ** Default Interface **
 *    Windows.Globalization.ILanguageExtensionSubtags
 *    Windows.Globalization.ILanguage2
 *    Windows.Globalization.ILanguage3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Language_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Language_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Language[] = L"Windows.Globalization.Language";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumeralSystemIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.INumeralSystemIdentifiersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.INumeralSystemIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumeralSystemIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumeralSystemIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumeralSystemIdentifiers[] = L"Windows.Globalization.NumeralSystemIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2 __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2;

#endif // ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICalendar __x_ABI_CWindows_CGlobalization_CICalendar;

#endif // ____x_ABI_CWindows_CGlobalization_CICalendar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICalendarFactory __x_ABI_CWindows_CGlobalization_CICalendarFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICalendarFactory2 __x_ABI_CWindows_CGlobalization_CICalendarFactory2;

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2 __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2;

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3 __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3;

#endif // ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICurrencyAmount __x_ABI_CWindows_CGlobalization_CICurrencyAmount;

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2 __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2;

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3 __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3;

#endif // ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIGeographicRegion __x_ABI_CWindows_CGlobalization_CIGeographicRegion;

#endif // ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme;

#endif // ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguage __x_ABI_CWindows_CGlobalization_CILanguage;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguage2 __x_ABI_CWindows_CGlobalization_CILanguage2;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguage3 __x_ABI_CWindows_CGlobalization_CILanguage3;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguageFactory __x_ABI_CWindows_CGlobalization_CILanguageFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguageStatics __x_ABI_CWindows_CGlobalization_CILanguageStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguageStatics2 __x_ABI_CWindows_CGlobalization_CILanguageStatics2;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguageStatics3 __x_ABI_CWindows_CGlobalization_CILanguageStatics3;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2 __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2;

#endif // ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar;

#endif // ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_FWD_DEFINED__

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

#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGlobalization__CJapanesePhoneme;

typedef struct __FIIterator_1_Windows__CGlobalization__CJapanesePhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGlobalization__CJapanesePhoneme* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGlobalization__CJapanesePhonemeVtbl;

interface __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme
{
    CONST_VTBL struct __FIIterator_1_Windows__CGlobalization__CJapanesePhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGlobalization__CJapanesePhoneme;

typedef struct __FIIterable_1_Windows__CGlobalization__CJapanesePhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGlobalization__CJapanesePhoneme* This,
        __FIIterator_1_Windows__CGlobalization__CJapanesePhoneme** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGlobalization__CJapanesePhonemeVtbl;

interface __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme
{
    CONST_VTBL struct __FIIterable_1_Windows__CGlobalization__CJapanesePhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

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

#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme;

typedef struct __FIVectorView_1_Windows__CGlobalization__CJapanesePhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        UINT32 index,
        __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGlobalization__CJapanesePhonemeVtbl;

interface __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGlobalization__CJapanesePhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

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

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGlobalization_CDayOfWeek __x_ABI_CWindows_CGlobalization_CDayOfWeek;

typedef enum __x_ABI_CWindows_CGlobalization_CLanguageLayoutDirection __x_ABI_CWindows_CGlobalization_CLanguageLayoutDirection;

/*
 *
 * Struct Windows.Globalization.DayOfWeek
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDayOfWeek
{
    DayOfWeek_Sunday = 0,
    DayOfWeek_Monday = 1,
    DayOfWeek_Tuesday = 2,
    DayOfWeek_Wednesday = 3,
    DayOfWeek_Thursday = 4,
    DayOfWeek_Friday = 5,
    DayOfWeek_Saturday = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.LanguageLayoutDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGlobalization_CLanguageLayoutDirection
{
    LanguageLayoutDirection_Ltr = 0,
    LanguageLayoutDirection_Rtl = 1,
    LanguageLayoutDirection_TtbLtr = 2,
    LanguageLayoutDirection_TtbRtl = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Globalization.IApplicationLanguagesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.ApplicationLanguages
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IApplicationLanguagesStatics[] = L"Windows.Globalization.IApplicationLanguagesStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryLanguageOverride)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PrimaryLanguageOverride)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ManifestLanguages)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_get_PrimaryLanguageOverride(This, value) \
    ((This)->lpVtbl->get_PrimaryLanguageOverride(This, value))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_put_PrimaryLanguageOverride(This, value) \
    ((This)->lpVtbl->put_PrimaryLanguageOverride(This, value))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_get_ManifestLanguages(This, value) \
    ((This)->lpVtbl->get_ManifestLanguages(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IApplicationLanguagesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.ApplicationLanguages
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IApplicationLanguagesStatics2[] = L"Windows.Globalization.IApplicationLanguagesStatics2";
typedef struct __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetLanguagesForUser)(__x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_GetLanguagesForUser(This, user, value) \
    ((This)->lpVtbl->GetLanguagesForUser(This, user, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIApplicationLanguagesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Globalization.ICalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendar[] = L"Windows.Globalization.ICalendar";
typedef struct __x_ABI_CWindows_CGlobalization_CICalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        __x_ABI_CWindows_CGlobalization_CICalendar** value);
    HRESULT (STDMETHODCALLTYPE* SetToMin)(__x_ABI_CWindows_CGlobalization_CICalendar* This);
    HRESULT (STDMETHODCALLTYPE* SetToMax)(__x_ABI_CWindows_CGlobalization_CICalendar* This);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetCalendarSystem)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ChangeCalendarSystem)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetClock)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ChangeClock)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetDateTime)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);
    HRESULT (STDMETHODCALLTYPE* SetDateTime)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* SetToNow)(__x_ABI_CWindows_CGlobalization_CICalendar* This);
    HRESULT (STDMETHODCALLTYPE* get_FirstEra)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastEra)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfEras)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Era)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Era)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddEras)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 eras);
    HRESULT (STDMETHODCALLTYPE* EraAsFullString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* EraAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 idealLength,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_FirstYearInThisEra)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastYearInThisEra)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfYearsInThisEra)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Year)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Year)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddYears)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 years);
    HRESULT (STDMETHODCALLTYPE* YearAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* YearAsTruncatedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 remainingDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* YearAsPaddedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_FirstMonthInThisYear)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastMonthInThisYear)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfMonthsInThisYear)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Month)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Month)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddMonths)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 months);
    HRESULT (STDMETHODCALLTYPE* MonthAsFullString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* MonthAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 idealLength,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* MonthAsFullSoloString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* MonthAsSoloString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 idealLength,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* MonthAsNumericString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* MonthAsPaddedNumericString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* AddWeeks)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 weeks);
    HRESULT (STDMETHODCALLTYPE* get_FirstDayInThisMonth)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastDayInThisMonth)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfDaysInThisMonth)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Day)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Day)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddDays)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 days);
    HRESULT (STDMETHODCALLTYPE* DayAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* DayAsPaddedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_DayOfWeek)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        enum __x_ABI_CWindows_CGlobalization_CDayOfWeek* value);
    HRESULT (STDMETHODCALLTYPE* DayOfWeekAsFullString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* DayOfWeekAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 idealLength,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* DayOfWeekAsFullSoloString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* DayOfWeekAsSoloString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 idealLength,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_FirstPeriodInThisDay)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastPeriodInThisDay)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfPeriodsInThisDay)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Period)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Period)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddPeriods)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 periods);
    HRESULT (STDMETHODCALLTYPE* PeriodAsFullString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* PeriodAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 idealLength,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_FirstHourInThisPeriod)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastHourInThisPeriod)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfHoursInThisPeriod)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Hour)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Hour)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddHours)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 hours);
    HRESULT (STDMETHODCALLTYPE* HourAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* HourAsPaddedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Minute)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Minute)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddMinutes)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minutes);
    HRESULT (STDMETHODCALLTYPE* MinuteAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* MinuteAsPaddedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Second)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Second)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddSeconds)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 seconds);
    HRESULT (STDMETHODCALLTYPE* SecondAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SecondAsPaddedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Nanosecond)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Nanosecond)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddNanoseconds)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 nanoseconds);
    HRESULT (STDMETHODCALLTYPE* NanosecondAsString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* NanosecondAsPaddedString)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32 minDigits,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* Compare)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        __x_ABI_CWindows_CGlobalization_CICalendar* other,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* CompareDateTime)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime other,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* CopyTo)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        __x_ABI_CWindows_CGlobalization_CICalendar* other);
    HRESULT (STDMETHODCALLTYPE* get_FirstMinuteInThisHour)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastMinuteInThisHour)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfMinutesInThisHour)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_FirstSecondInThisMinute)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastSecondInThisMinute)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfSecondsInThisMinute)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDaylightSavingTime)(__x_ABI_CWindows_CGlobalization_CICalendar* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICalendarVtbl;

interface __x_ABI_CWindows_CGlobalization_CICalendar
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICalendar_Clone(This, value) \
    ((This)->lpVtbl->Clone(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_SetToMin(This) \
    ((This)->lpVtbl->SetToMin(This))

#define __x_ABI_CWindows_CGlobalization_CICalendar_SetToMax(This) \
    ((This)->lpVtbl->SetToMax(This))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumeralSystem(This, value) \
    ((This)->lpVtbl->get_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_NumeralSystem(This, value) \
    ((This)->lpVtbl->put_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_GetCalendarSystem(This, value) \
    ((This)->lpVtbl->GetCalendarSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_ChangeCalendarSystem(This, value) \
    ((This)->lpVtbl->ChangeCalendarSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_GetClock(This, value) \
    ((This)->lpVtbl->GetClock(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_ChangeClock(This, value) \
    ((This)->lpVtbl->ChangeClock(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_GetDateTime(This, result) \
    ((This)->lpVtbl->GetDateTime(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_SetDateTime(This, value) \
    ((This)->lpVtbl->SetDateTime(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_SetToNow(This) \
    ((This)->lpVtbl->SetToNow(This))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstEra(This, value) \
    ((This)->lpVtbl->get_FirstEra(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastEra(This, value) \
    ((This)->lpVtbl->get_LastEra(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfEras(This, value) \
    ((This)->lpVtbl->get_NumberOfEras(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Era(This, value) \
    ((This)->lpVtbl->get_Era(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Era(This, value) \
    ((This)->lpVtbl->put_Era(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddEras(This, eras) \
    ((This)->lpVtbl->AddEras(This, eras))

#define __x_ABI_CWindows_CGlobalization_CICalendar_EraAsFullString(This, result) \
    ((This)->lpVtbl->EraAsFullString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_EraAsString(This, idealLength, result) \
    ((This)->lpVtbl->EraAsString(This, idealLength, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstYearInThisEra(This, value) \
    ((This)->lpVtbl->get_FirstYearInThisEra(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastYearInThisEra(This, value) \
    ((This)->lpVtbl->get_LastYearInThisEra(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfYearsInThisEra(This, value) \
    ((This)->lpVtbl->get_NumberOfYearsInThisEra(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Year(This, value) \
    ((This)->lpVtbl->get_Year(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Year(This, value) \
    ((This)->lpVtbl->put_Year(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddYears(This, years) \
    ((This)->lpVtbl->AddYears(This, years))

#define __x_ABI_CWindows_CGlobalization_CICalendar_YearAsString(This, result) \
    ((This)->lpVtbl->YearAsString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_YearAsTruncatedString(This, remainingDigits, result) \
    ((This)->lpVtbl->YearAsTruncatedString(This, remainingDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_YearAsPaddedString(This, minDigits, result) \
    ((This)->lpVtbl->YearAsPaddedString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstMonthInThisYear(This, value) \
    ((This)->lpVtbl->get_FirstMonthInThisYear(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastMonthInThisYear(This, value) \
    ((This)->lpVtbl->get_LastMonthInThisYear(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfMonthsInThisYear(This, value) \
    ((This)->lpVtbl->get_NumberOfMonthsInThisYear(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Month(This, value) \
    ((This)->lpVtbl->get_Month(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Month(This, value) \
    ((This)->lpVtbl->put_Month(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddMonths(This, months) \
    ((This)->lpVtbl->AddMonths(This, months))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MonthAsFullString(This, result) \
    ((This)->lpVtbl->MonthAsFullString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MonthAsString(This, idealLength, result) \
    ((This)->lpVtbl->MonthAsString(This, idealLength, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MonthAsFullSoloString(This, result) \
    ((This)->lpVtbl->MonthAsFullSoloString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MonthAsSoloString(This, idealLength, result) \
    ((This)->lpVtbl->MonthAsSoloString(This, idealLength, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MonthAsNumericString(This, result) \
    ((This)->lpVtbl->MonthAsNumericString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MonthAsPaddedNumericString(This, minDigits, result) \
    ((This)->lpVtbl->MonthAsPaddedNumericString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddWeeks(This, weeks) \
    ((This)->lpVtbl->AddWeeks(This, weeks))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstDayInThisMonth(This, value) \
    ((This)->lpVtbl->get_FirstDayInThisMonth(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastDayInThisMonth(This, value) \
    ((This)->lpVtbl->get_LastDayInThisMonth(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfDaysInThisMonth(This, value) \
    ((This)->lpVtbl->get_NumberOfDaysInThisMonth(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Day(This, value) \
    ((This)->lpVtbl->get_Day(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Day(This, value) \
    ((This)->lpVtbl->put_Day(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddDays(This, days) \
    ((This)->lpVtbl->AddDays(This, days))

#define __x_ABI_CWindows_CGlobalization_CICalendar_DayAsString(This, result) \
    ((This)->lpVtbl->DayAsString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_DayAsPaddedString(This, minDigits, result) \
    ((This)->lpVtbl->DayAsPaddedString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_DayOfWeek(This, value) \
    ((This)->lpVtbl->get_DayOfWeek(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_DayOfWeekAsFullString(This, result) \
    ((This)->lpVtbl->DayOfWeekAsFullString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_DayOfWeekAsString(This, idealLength, result) \
    ((This)->lpVtbl->DayOfWeekAsString(This, idealLength, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_DayOfWeekAsFullSoloString(This, result) \
    ((This)->lpVtbl->DayOfWeekAsFullSoloString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_DayOfWeekAsSoloString(This, idealLength, result) \
    ((This)->lpVtbl->DayOfWeekAsSoloString(This, idealLength, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstPeriodInThisDay(This, value) \
    ((This)->lpVtbl->get_FirstPeriodInThisDay(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastPeriodInThisDay(This, value) \
    ((This)->lpVtbl->get_LastPeriodInThisDay(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfPeriodsInThisDay(This, value) \
    ((This)->lpVtbl->get_NumberOfPeriodsInThisDay(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Period(This, value) \
    ((This)->lpVtbl->get_Period(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Period(This, value) \
    ((This)->lpVtbl->put_Period(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddPeriods(This, periods) \
    ((This)->lpVtbl->AddPeriods(This, periods))

#define __x_ABI_CWindows_CGlobalization_CICalendar_PeriodAsFullString(This, result) \
    ((This)->lpVtbl->PeriodAsFullString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_PeriodAsString(This, idealLength, result) \
    ((This)->lpVtbl->PeriodAsString(This, idealLength, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstHourInThisPeriod(This, value) \
    ((This)->lpVtbl->get_FirstHourInThisPeriod(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastHourInThisPeriod(This, value) \
    ((This)->lpVtbl->get_LastHourInThisPeriod(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfHoursInThisPeriod(This, value) \
    ((This)->lpVtbl->get_NumberOfHoursInThisPeriod(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Hour(This, value) \
    ((This)->lpVtbl->get_Hour(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Hour(This, value) \
    ((This)->lpVtbl->put_Hour(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddHours(This, hours) \
    ((This)->lpVtbl->AddHours(This, hours))

#define __x_ABI_CWindows_CGlobalization_CICalendar_HourAsString(This, result) \
    ((This)->lpVtbl->HourAsString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_HourAsPaddedString(This, minDigits, result) \
    ((This)->lpVtbl->HourAsPaddedString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Minute(This, value) \
    ((This)->lpVtbl->get_Minute(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Minute(This, value) \
    ((This)->lpVtbl->put_Minute(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddMinutes(This, minutes) \
    ((This)->lpVtbl->AddMinutes(This, minutes))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MinuteAsString(This, result) \
    ((This)->lpVtbl->MinuteAsString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_MinuteAsPaddedString(This, minDigits, result) \
    ((This)->lpVtbl->MinuteAsPaddedString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Second(This, value) \
    ((This)->lpVtbl->get_Second(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Second(This, value) \
    ((This)->lpVtbl->put_Second(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddSeconds(This, seconds) \
    ((This)->lpVtbl->AddSeconds(This, seconds))

#define __x_ABI_CWindows_CGlobalization_CICalendar_SecondAsString(This, result) \
    ((This)->lpVtbl->SecondAsString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_SecondAsPaddedString(This, minDigits, result) \
    ((This)->lpVtbl->SecondAsPaddedString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_Nanosecond(This, value) \
    ((This)->lpVtbl->get_Nanosecond(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_put_Nanosecond(This, value) \
    ((This)->lpVtbl->put_Nanosecond(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_AddNanoseconds(This, nanoseconds) \
    ((This)->lpVtbl->AddNanoseconds(This, nanoseconds))

#define __x_ABI_CWindows_CGlobalization_CICalendar_NanosecondAsString(This, result) \
    ((This)->lpVtbl->NanosecondAsString(This, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_NanosecondAsPaddedString(This, minDigits, result) \
    ((This)->lpVtbl->NanosecondAsPaddedString(This, minDigits, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_Compare(This, other, result) \
    ((This)->lpVtbl->Compare(This, other, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_CompareDateTime(This, other, result) \
    ((This)->lpVtbl->CompareDateTime(This, other, result))

#define __x_ABI_CWindows_CGlobalization_CICalendar_CopyTo(This, other) \
    ((This)->lpVtbl->CopyTo(This, other))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstMinuteInThisHour(This, value) \
    ((This)->lpVtbl->get_FirstMinuteInThisHour(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastMinuteInThisHour(This, value) \
    ((This)->lpVtbl->get_LastMinuteInThisHour(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfMinutesInThisHour(This, value) \
    ((This)->lpVtbl->get_NumberOfMinutesInThisHour(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_FirstSecondInThisMinute(This, value) \
    ((This)->lpVtbl->get_FirstSecondInThisMinute(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_LastSecondInThisMinute(This, value) \
    ((This)->lpVtbl->get_LastSecondInThisMinute(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_NumberOfSecondsInThisMinute(This, value) \
    ((This)->lpVtbl->get_NumberOfSecondsInThisMinute(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendar_get_IsDaylightSavingTime(This, value) \
    ((This)->lpVtbl->get_IsDaylightSavingTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendar;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarFactory[] = L"Windows.Globalization.ICalendarFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CICalendarFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCalendarDefaultCalendarAndClock)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This,
        __FIIterable_1_HSTRING* languages,
        __x_ABI_CWindows_CGlobalization_CICalendar** result);
    HRESULT (STDMETHODCALLTYPE* CreateCalendar)(__x_ABI_CWindows_CGlobalization_CICalendarFactory* This,
        __FIIterable_1_HSTRING* languages,
        HSTRING calendar,
        HSTRING clock,
        __x_ABI_CWindows_CGlobalization_CICalendar** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICalendarFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CICalendarFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICalendarFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_CreateCalendarDefaultCalendarAndClock(This, languages, result) \
    ((This)->lpVtbl->CreateCalendarDefaultCalendarAndClock(This, languages, result))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory_CreateCalendar(This, languages, calendar, clock, result) \
    ((This)->lpVtbl->CreateCalendar(This, languages, calendar, clock, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarFactory2[] = L"Windows.Globalization.ICalendarFactory2";
typedef struct __x_ABI_CWindows_CGlobalization_CICalendarFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCalendarWithTimeZone)(__x_ABI_CWindows_CGlobalization_CICalendarFactory2* This,
        __FIIterable_1_HSTRING* languages,
        HSTRING calendar,
        HSTRING clock,
        HSTRING timeZoneId,
        __x_ABI_CWindows_CGlobalization_CICalendar** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICalendarFactory2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CICalendarFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICalendarFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICalendarFactory2_CreateCalendarWithTimeZone(This, languages, calendar, clock, timeZoneId, result) \
    ((This)->lpVtbl->CreateCalendarWithTimeZone(This, languages, calendar, clock, timeZoneId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarFactory2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CalendarIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarIdentifiersStatics[] = L"Windows.Globalization.ICalendarIdentifiersStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Gregorian)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Hebrew)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Hijri)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Japanese)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Julian)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Korean)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Taiwan)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Thai)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UmAlQura)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Gregorian(This, value) \
    ((This)->lpVtbl->get_Gregorian(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Hebrew(This, value) \
    ((This)->lpVtbl->get_Hebrew(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Hijri(This, value) \
    ((This)->lpVtbl->get_Hijri(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Japanese(This, value) \
    ((This)->lpVtbl->get_Japanese(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Julian(This, value) \
    ((This)->lpVtbl->get_Julian(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Korean(This, value) \
    ((This)->lpVtbl->get_Korean(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Taiwan(This, value) \
    ((This)->lpVtbl->get_Taiwan(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_Thai(This, value) \
    ((This)->lpVtbl->get_Thai(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_get_UmAlQura(This, value) \
    ((This)->lpVtbl->get_UmAlQura(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CalendarIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarIdentifiersStatics2[] = L"Windows.Globalization.ICalendarIdentifiersStatics2";
typedef struct __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Persian)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_get_Persian(This, value) \
    ((This)->lpVtbl->get_Persian(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICalendarIdentifiersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CalendarIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICalendarIdentifiersStatics3[] = L"Windows.Globalization.ICalendarIdentifiersStatics3";
typedef struct __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChineseLunar)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_JapaneseLunar)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KoreanLunar)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TaiwanLunar)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VietnameseLunar)(__x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3Vtbl;

interface __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_get_ChineseLunar(This, value) \
    ((This)->lpVtbl->get_ChineseLunar(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_get_JapaneseLunar(This, value) \
    ((This)->lpVtbl->get_JapaneseLunar(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_get_KoreanLunar(This, value) \
    ((This)->lpVtbl->get_KoreanLunar(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_get_TaiwanLunar(This, value) \
    ((This)->lpVtbl->get_TaiwanLunar(This, value))

#define __x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_get_VietnameseLunar(This, value) \
    ((This)->lpVtbl->get_VietnameseLunar(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICalendarIdentifiersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Globalization.IClockIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.ClockIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IClockIdentifiersStatics[] = L"Windows.Globalization.IClockIdentifiersStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TwelveHour)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TwentyFourHour)(__x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_get_TwelveHour(This, value) \
    ((This)->lpVtbl->get_TwelveHour(This, value))

#define __x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_get_TwentyFourHour(This, value) \
    ((This)->lpVtbl->get_TwentyFourHour(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIClockIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyAmount[] = L"Windows.Globalization.ICurrencyAmount";
typedef struct __x_ABI_CWindows_CGlobalization_CICurrencyAmountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Amount)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Currency)(__x_ABI_CWindows_CGlobalization_CICurrencyAmount* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICurrencyAmountVtbl;

interface __x_ABI_CWindows_CGlobalization_CICurrencyAmount
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICurrencyAmountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_get_Amount(This, value) \
    ((This)->lpVtbl->get_Amount(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmount_get_Currency(This, value) \
    ((This)->lpVtbl->get_Currency(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyAmount;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Globalization.ICurrencyAmountFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyAmountFactory[] = L"Windows.Globalization.ICurrencyAmountFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory* This,
        HSTRING amount,
        HSTRING currency,
        __x_ABI_CWindows_CGlobalization_CICurrencyAmount** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_Create(This, amount, currency, result) \
    ((This)->lpVtbl->Create(This, amount, currency, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyAmountFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Globalization.ICurrencyIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyIdentifiersStatics[] = L"Windows.Globalization.ICurrencyIdentifiersStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AED)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AFN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ALL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AMD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ANG)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AOA)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ARS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AUD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AWG)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AZN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BAM)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BBD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BDT)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BGN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BHD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BIF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BMD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BND)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BOB)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BRL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BSD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BTN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BWP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BYR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BZD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CAD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CDF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CHF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CLP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CNY)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_COP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CRC)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CUP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CVE)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CZK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DJF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DKK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DOP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DZD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EGP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ERN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ETB)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EUR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FJD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FKP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GBP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GEL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GHS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GIP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GMD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GNF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GTQ)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GYD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HKD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HNL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HRK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HTG)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HUF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IDR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ILS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_INR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IQD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IRR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ISK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_JMD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_JOD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_JPY)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KES)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KGS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KHR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KMF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KPW)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KRW)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KWD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KYD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KZT)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LAK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LBP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LKR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LRD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LSL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LTL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LVL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LYD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MAD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MDL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MGA)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MKD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MMK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MNT)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MOP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MRO)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MUR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MVR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MWK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MXN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MYR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MZN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NAD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NGN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NIO)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NOK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NPR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NZD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OMR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PAB)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PEN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PGK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PHP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PKR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PLN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PYG)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_QAR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RON)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RSD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RUB)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RWF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SAR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SBD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SCR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SDG)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SEK)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SGD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SHP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SLL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SOS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SRD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_STD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SYP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SZL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_THB)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TJS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TMT)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TND)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TOP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TRY)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TTD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TWD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TZS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UAH)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UGX)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_USD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UYU)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UZS)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VEF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VND)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VUV)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_WST)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XAF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XCD)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XOF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XPF)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XXX)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_YER)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZAR)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZMW)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZWL)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AED(This, value) \
    ((This)->lpVtbl->get_AED(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AFN(This, value) \
    ((This)->lpVtbl->get_AFN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ALL(This, value) \
    ((This)->lpVtbl->get_ALL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AMD(This, value) \
    ((This)->lpVtbl->get_AMD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ANG(This, value) \
    ((This)->lpVtbl->get_ANG(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AOA(This, value) \
    ((This)->lpVtbl->get_AOA(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ARS(This, value) \
    ((This)->lpVtbl->get_ARS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AUD(This, value) \
    ((This)->lpVtbl->get_AUD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AWG(This, value) \
    ((This)->lpVtbl->get_AWG(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_AZN(This, value) \
    ((This)->lpVtbl->get_AZN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BAM(This, value) \
    ((This)->lpVtbl->get_BAM(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BBD(This, value) \
    ((This)->lpVtbl->get_BBD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BDT(This, value) \
    ((This)->lpVtbl->get_BDT(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BGN(This, value) \
    ((This)->lpVtbl->get_BGN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BHD(This, value) \
    ((This)->lpVtbl->get_BHD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BIF(This, value) \
    ((This)->lpVtbl->get_BIF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BMD(This, value) \
    ((This)->lpVtbl->get_BMD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BND(This, value) \
    ((This)->lpVtbl->get_BND(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BOB(This, value) \
    ((This)->lpVtbl->get_BOB(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BRL(This, value) \
    ((This)->lpVtbl->get_BRL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BSD(This, value) \
    ((This)->lpVtbl->get_BSD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BTN(This, value) \
    ((This)->lpVtbl->get_BTN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BWP(This, value) \
    ((This)->lpVtbl->get_BWP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BYR(This, value) \
    ((This)->lpVtbl->get_BYR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_BZD(This, value) \
    ((This)->lpVtbl->get_BZD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CAD(This, value) \
    ((This)->lpVtbl->get_CAD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CDF(This, value) \
    ((This)->lpVtbl->get_CDF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CHF(This, value) \
    ((This)->lpVtbl->get_CHF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CLP(This, value) \
    ((This)->lpVtbl->get_CLP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CNY(This, value) \
    ((This)->lpVtbl->get_CNY(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_COP(This, value) \
    ((This)->lpVtbl->get_COP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CRC(This, value) \
    ((This)->lpVtbl->get_CRC(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CUP(This, value) \
    ((This)->lpVtbl->get_CUP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CVE(This, value) \
    ((This)->lpVtbl->get_CVE(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_CZK(This, value) \
    ((This)->lpVtbl->get_CZK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_DJF(This, value) \
    ((This)->lpVtbl->get_DJF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_DKK(This, value) \
    ((This)->lpVtbl->get_DKK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_DOP(This, value) \
    ((This)->lpVtbl->get_DOP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_DZD(This, value) \
    ((This)->lpVtbl->get_DZD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_EGP(This, value) \
    ((This)->lpVtbl->get_EGP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ERN(This, value) \
    ((This)->lpVtbl->get_ERN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ETB(This, value) \
    ((This)->lpVtbl->get_ETB(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_EUR(This, value) \
    ((This)->lpVtbl->get_EUR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_FJD(This, value) \
    ((This)->lpVtbl->get_FJD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_FKP(This, value) \
    ((This)->lpVtbl->get_FKP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GBP(This, value) \
    ((This)->lpVtbl->get_GBP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GEL(This, value) \
    ((This)->lpVtbl->get_GEL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GHS(This, value) \
    ((This)->lpVtbl->get_GHS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GIP(This, value) \
    ((This)->lpVtbl->get_GIP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GMD(This, value) \
    ((This)->lpVtbl->get_GMD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GNF(This, value) \
    ((This)->lpVtbl->get_GNF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GTQ(This, value) \
    ((This)->lpVtbl->get_GTQ(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_GYD(This, value) \
    ((This)->lpVtbl->get_GYD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_HKD(This, value) \
    ((This)->lpVtbl->get_HKD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_HNL(This, value) \
    ((This)->lpVtbl->get_HNL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_HRK(This, value) \
    ((This)->lpVtbl->get_HRK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_HTG(This, value) \
    ((This)->lpVtbl->get_HTG(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_HUF(This, value) \
    ((This)->lpVtbl->get_HUF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_IDR(This, value) \
    ((This)->lpVtbl->get_IDR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ILS(This, value) \
    ((This)->lpVtbl->get_ILS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_INR(This, value) \
    ((This)->lpVtbl->get_INR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_IQD(This, value) \
    ((This)->lpVtbl->get_IQD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_IRR(This, value) \
    ((This)->lpVtbl->get_IRR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ISK(This, value) \
    ((This)->lpVtbl->get_ISK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_JMD(This, value) \
    ((This)->lpVtbl->get_JMD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_JOD(This, value) \
    ((This)->lpVtbl->get_JOD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_JPY(This, value) \
    ((This)->lpVtbl->get_JPY(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KES(This, value) \
    ((This)->lpVtbl->get_KES(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KGS(This, value) \
    ((This)->lpVtbl->get_KGS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KHR(This, value) \
    ((This)->lpVtbl->get_KHR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KMF(This, value) \
    ((This)->lpVtbl->get_KMF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KPW(This, value) \
    ((This)->lpVtbl->get_KPW(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KRW(This, value) \
    ((This)->lpVtbl->get_KRW(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KWD(This, value) \
    ((This)->lpVtbl->get_KWD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KYD(This, value) \
    ((This)->lpVtbl->get_KYD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_KZT(This, value) \
    ((This)->lpVtbl->get_KZT(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LAK(This, value) \
    ((This)->lpVtbl->get_LAK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LBP(This, value) \
    ((This)->lpVtbl->get_LBP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LKR(This, value) \
    ((This)->lpVtbl->get_LKR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LRD(This, value) \
    ((This)->lpVtbl->get_LRD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LSL(This, value) \
    ((This)->lpVtbl->get_LSL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LTL(This, value) \
    ((This)->lpVtbl->get_LTL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LVL(This, value) \
    ((This)->lpVtbl->get_LVL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_LYD(This, value) \
    ((This)->lpVtbl->get_LYD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MAD(This, value) \
    ((This)->lpVtbl->get_MAD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MDL(This, value) \
    ((This)->lpVtbl->get_MDL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MGA(This, value) \
    ((This)->lpVtbl->get_MGA(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MKD(This, value) \
    ((This)->lpVtbl->get_MKD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MMK(This, value) \
    ((This)->lpVtbl->get_MMK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MNT(This, value) \
    ((This)->lpVtbl->get_MNT(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MOP(This, value) \
    ((This)->lpVtbl->get_MOP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MRO(This, value) \
    ((This)->lpVtbl->get_MRO(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MUR(This, value) \
    ((This)->lpVtbl->get_MUR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MVR(This, value) \
    ((This)->lpVtbl->get_MVR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MWK(This, value) \
    ((This)->lpVtbl->get_MWK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MXN(This, value) \
    ((This)->lpVtbl->get_MXN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MYR(This, value) \
    ((This)->lpVtbl->get_MYR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_MZN(This, value) \
    ((This)->lpVtbl->get_MZN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_NAD(This, value) \
    ((This)->lpVtbl->get_NAD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_NGN(This, value) \
    ((This)->lpVtbl->get_NGN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_NIO(This, value) \
    ((This)->lpVtbl->get_NIO(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_NOK(This, value) \
    ((This)->lpVtbl->get_NOK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_NPR(This, value) \
    ((This)->lpVtbl->get_NPR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_NZD(This, value) \
    ((This)->lpVtbl->get_NZD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_OMR(This, value) \
    ((This)->lpVtbl->get_OMR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PAB(This, value) \
    ((This)->lpVtbl->get_PAB(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PEN(This, value) \
    ((This)->lpVtbl->get_PEN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PGK(This, value) \
    ((This)->lpVtbl->get_PGK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PHP(This, value) \
    ((This)->lpVtbl->get_PHP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PKR(This, value) \
    ((This)->lpVtbl->get_PKR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PLN(This, value) \
    ((This)->lpVtbl->get_PLN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_PYG(This, value) \
    ((This)->lpVtbl->get_PYG(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_QAR(This, value) \
    ((This)->lpVtbl->get_QAR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_RON(This, value) \
    ((This)->lpVtbl->get_RON(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_RSD(This, value) \
    ((This)->lpVtbl->get_RSD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_RUB(This, value) \
    ((This)->lpVtbl->get_RUB(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_RWF(This, value) \
    ((This)->lpVtbl->get_RWF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SAR(This, value) \
    ((This)->lpVtbl->get_SAR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SBD(This, value) \
    ((This)->lpVtbl->get_SBD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SCR(This, value) \
    ((This)->lpVtbl->get_SCR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SDG(This, value) \
    ((This)->lpVtbl->get_SDG(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SEK(This, value) \
    ((This)->lpVtbl->get_SEK(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SGD(This, value) \
    ((This)->lpVtbl->get_SGD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SHP(This, value) \
    ((This)->lpVtbl->get_SHP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SLL(This, value) \
    ((This)->lpVtbl->get_SLL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SOS(This, value) \
    ((This)->lpVtbl->get_SOS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SRD(This, value) \
    ((This)->lpVtbl->get_SRD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_STD(This, value) \
    ((This)->lpVtbl->get_STD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SYP(This, value) \
    ((This)->lpVtbl->get_SYP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_SZL(This, value) \
    ((This)->lpVtbl->get_SZL(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_THB(This, value) \
    ((This)->lpVtbl->get_THB(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TJS(This, value) \
    ((This)->lpVtbl->get_TJS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TMT(This, value) \
    ((This)->lpVtbl->get_TMT(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TND(This, value) \
    ((This)->lpVtbl->get_TND(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TOP(This, value) \
    ((This)->lpVtbl->get_TOP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TRY(This, value) \
    ((This)->lpVtbl->get_TRY(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TTD(This, value) \
    ((This)->lpVtbl->get_TTD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TWD(This, value) \
    ((This)->lpVtbl->get_TWD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_TZS(This, value) \
    ((This)->lpVtbl->get_TZS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_UAH(This, value) \
    ((This)->lpVtbl->get_UAH(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_UGX(This, value) \
    ((This)->lpVtbl->get_UGX(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_USD(This, value) \
    ((This)->lpVtbl->get_USD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_UYU(This, value) \
    ((This)->lpVtbl->get_UYU(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_UZS(This, value) \
    ((This)->lpVtbl->get_UZS(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_VEF(This, value) \
    ((This)->lpVtbl->get_VEF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_VND(This, value) \
    ((This)->lpVtbl->get_VND(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_VUV(This, value) \
    ((This)->lpVtbl->get_VUV(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_WST(This, value) \
    ((This)->lpVtbl->get_WST(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_XAF(This, value) \
    ((This)->lpVtbl->get_XAF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_XCD(This, value) \
    ((This)->lpVtbl->get_XCD(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_XOF(This, value) \
    ((This)->lpVtbl->get_XOF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_XPF(This, value) \
    ((This)->lpVtbl->get_XPF(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_XXX(This, value) \
    ((This)->lpVtbl->get_XXX(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_YER(This, value) \
    ((This)->lpVtbl->get_YER(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ZAR(This, value) \
    ((This)->lpVtbl->get_ZAR(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ZMW(This, value) \
    ((This)->lpVtbl->get_ZMW(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_get_ZWL(This, value) \
    ((This)->lpVtbl->get_ZWL(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ICurrencyIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyIdentifiersStatics2[] = L"Windows.Globalization.ICurrencyIdentifiersStatics2";
typedef struct __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BYN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_get_BYN(This, value) \
    ((This)->lpVtbl->get_BYN(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Globalization.ICurrencyIdentifiersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.CurrencyIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ICurrencyIdentifiersStatics3[] = L"Windows.Globalization.ICurrencyIdentifiersStatics3";
typedef struct __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MRU)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SSP)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_STN)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VES)(__x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3Vtbl;

interface __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_get_MRU(This, value) \
    ((This)->lpVtbl->get_MRU(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_get_SSP(This, value) \
    ((This)->lpVtbl->get_SSP(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_get_STN(This, value) \
    ((This)->lpVtbl->get_STN(This, value))

#define __x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_get_VES(This, value) \
    ((This)->lpVtbl->get_VES(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CICurrencyIdentifiersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Globalization.IGeographicRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.GeographicRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IGeographicRegion[] = L"Windows.Globalization.IGeographicRegion";
typedef struct __x_ABI_CWindows_CGlobalization_CIGeographicRegionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Code)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CodeTwoLetter)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CodeThreeLetter)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CodeThreeDigit)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NativeName)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrenciesInUse)(__x_ABI_CWindows_CGlobalization_CIGeographicRegion* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIGeographicRegionVtbl;

interface __x_ABI_CWindows_CGlobalization_CIGeographicRegion
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIGeographicRegionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_Code(This, value) \
    ((This)->lpVtbl->get_Code(This, value))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_CodeTwoLetter(This, value) \
    ((This)->lpVtbl->get_CodeTwoLetter(This, value))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_CodeThreeLetter(This, value) \
    ((This)->lpVtbl->get_CodeThreeLetter(This, value))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_CodeThreeDigit(This, value) \
    ((This)->lpVtbl->get_CodeThreeDigit(This, value))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_NativeName(This, value) \
    ((This)->lpVtbl->get_NativeName(This, value))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegion_get_CurrenciesInUse(This, value) \
    ((This)->lpVtbl->get_CurrenciesInUse(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIGeographicRegion;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IGeographicRegionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.GeographicRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IGeographicRegionFactory[] = L"Windows.Globalization.IGeographicRegionFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateGeographicRegion)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory* This,
        HSTRING geographicRegionCode,
        __x_ABI_CWindows_CGlobalization_CIGeographicRegion** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_CreateGeographicRegion(This, geographicRegionCode, result) \
    ((This)->lpVtbl->CreateGeographicRegion(This, geographicRegionCode, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IGeographicRegionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.GeographicRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IGeographicRegionStatics[] = L"Windows.Globalization.IGeographicRegionStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CIGeographicRegionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics* This,
        HSTRING geographicRegionCode,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIGeographicRegionStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIGeographicRegionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_IsSupported(This, geographicRegionCode, result) \
    ((This)->lpVtbl->IsSupported(This, geographicRegionCode, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIGeographicRegionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IJapanesePhoneme
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.JapanesePhoneme
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IJapanesePhoneme[] = L"Windows.Globalization.IJapanesePhoneme";
typedef struct __x_ABI_CWindows_CGlobalization_CIJapanesePhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayText)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_YomiText)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPhraseStart)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneme* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIJapanesePhonemeVtbl;

interface __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIJapanesePhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_get_DisplayText(This, value) \
    ((This)->lpVtbl->get_DisplayText(This, value))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_get_YomiText(This, value) \
    ((This)->lpVtbl->get_YomiText(This, value))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_get_IsPhraseStart(This, value) \
    ((This)->lpVtbl->get_IsPhraseStart(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIJapanesePhoneme;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneme_INTERFACE_DEFINED__) */
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.IJapanesePhoneticAnalyzerStatics
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.JapanesePhoneticAnalyzer
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_IJapanesePhoneticAnalyzerStatics[] = L"Windows.Globalization.IJapanesePhoneticAnalyzerStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetWords)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This,
        HSTRING input,
        __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme** result);
    HRESULT (STDMETHODCALLTYPE* GetWordsWithMonoRubyOption)(__x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics* This,
        HSTRING input,
        boolean monoRuby,
        __FIVectorView_1_Windows__CGlobalization__CJapanesePhoneme** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_GetWords(This, input, result) \
    ((This)->lpVtbl->GetWords(This, input, result))

#define __x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_GetWordsWithMonoRubyOption(This, input, monoRuby, result) \
    ((This)->lpVtbl->GetWordsWithMonoRubyOption(This, input, monoRuby, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CIJapanesePhoneticAnalyzerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguage[] = L"Windows.Globalization.ILanguage";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LanguageTag)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NativeName)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Script)(__x_ABI_CWindows_CGlobalization_CILanguage* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguageVtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguage
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguage_get_LanguageTag(This, value) \
    ((This)->lpVtbl->get_LanguageTag(This, value))

#define __x_ABI_CWindows_CGlobalization_CILanguage_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGlobalization_CILanguage_get_NativeName(This, value) \
    ((This)->lpVtbl->get_NativeName(This, value))

#define __x_ABI_CWindows_CGlobalization_CILanguage_get_Script(This, value) \
    ((This)->lpVtbl->get_Script(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguage;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguage2[] = L"Windows.Globalization.ILanguage2";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LayoutDirection)(__x_ABI_CWindows_CGlobalization_CILanguage2* This,
        enum __x_ABI_CWindows_CGlobalization_CLanguageLayoutDirection* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguage2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguage2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguage2_get_LayoutDirection(This, value) \
    ((This)->lpVtbl->get_LayoutDirection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguage2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Globalization.ILanguage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguage3[] = L"Windows.Globalization.ILanguage3";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguage3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguage3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguage3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguage3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguage3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguage3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguage3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AbbreviatedName)(__x_ABI_CWindows_CGlobalization_CILanguage3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguage3Vtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguage3
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguage3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguage3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguage3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguage3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguage3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguage3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguage3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguage3_get_AbbreviatedName(This, value) \
    ((This)->lpVtbl->get_AbbreviatedName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguage3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Globalization.ILanguageExtensionSubtags
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageExtensionSubtags[] = L"Windows.Globalization.ILanguageExtensionSubtags";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtagsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetExtensionSubtags)(__x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags* This,
        HSTRING singleton,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtagsVtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtagsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_GetExtensionSubtags(This, singleton, value) \
    ((This)->lpVtbl->GetExtensionSubtags(This, singleton, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageExtensionSubtags_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageFactory[] = L"Windows.Globalization.ILanguageFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguageFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateLanguage)(__x_ABI_CWindows_CGlobalization_CILanguageFactory* This,
        HSTRING languageTag,
        __x_ABI_CWindows_CGlobalization_CILanguage** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguageFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguageFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguageFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguageFactory_CreateLanguage(This, languageTag, result) \
    ((This)->lpVtbl->CreateLanguage(This, languageTag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageStatics[] = L"Windows.Globalization.ILanguageStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguageStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsWellFormed)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This,
        HSTRING languageTag,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_CurrentInputMethodLanguageTag)(__x_ABI_CWindows_CGlobalization_CILanguageStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguageStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguageStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguageStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_IsWellFormed(This, languageTag, result) \
    ((This)->lpVtbl->IsWellFormed(This, languageTag, result))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics_get_CurrentInputMethodLanguageTag(This, value) \
    ((This)->lpVtbl->get_CurrentInputMethodLanguageTag(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageStatics2[] = L"Windows.Globalization.ILanguageStatics2";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguageStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TrySetInputMethodLanguageTag)(__x_ABI_CWindows_CGlobalization_CILanguageStatics2* This,
        HSTRING languageTag,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguageStatics2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguageStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguageStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics2_TrySetInputMethodLanguageTag(This, languageTag, result) \
    ((This)->lpVtbl->TrySetInputMethodLanguageTag(This, languageTag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ILanguageStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Language
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CILanguageStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ILanguageStatics3[] = L"Windows.Globalization.ILanguageStatics3";
typedef struct __x_ABI_CWindows_CGlobalization_CILanguageStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetMuiCompatibleLanguageListFromLanguageTags)(__x_ABI_CWindows_CGlobalization_CILanguageStatics3* This,
        __FIIterable_1_HSTRING* languageTags,
        __FIVector_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CILanguageStatics3Vtbl;

interface __x_ABI_CWindows_CGlobalization_CILanguageStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CILanguageStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CILanguageStatics3_GetMuiCompatibleLanguageListFromLanguageTags(This, languageTags, result) \
    ((This)->lpVtbl->GetMuiCompatibleLanguageListFromLanguageTags(This, languageTags, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CILanguageStatics3;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CILanguageStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Globalization.INumeralSystemIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumeralSystemIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_INumeralSystemIdentifiersStatics[] = L"Windows.Globalization.INumeralSystemIdentifiersStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Arab)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ArabExt)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Bali)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Beng)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Cham)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Deva)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FullWide)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Gujr)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Guru)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HaniDec)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Java)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kali)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Khmr)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Knda)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Lana)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LanaTham)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Laoo)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Latn)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Lepc)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Limb)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mlym)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mong)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mtei)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mymr)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MymrShan)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Nkoo)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Olck)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Orya)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Saur)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sund)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Talu)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TamlDec)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Telu)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Thai)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Tibt)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Vaii)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Arab(This, value) \
    ((This)->lpVtbl->get_Arab(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_ArabExt(This, value) \
    ((This)->lpVtbl->get_ArabExt(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Bali(This, value) \
    ((This)->lpVtbl->get_Bali(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Beng(This, value) \
    ((This)->lpVtbl->get_Beng(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Cham(This, value) \
    ((This)->lpVtbl->get_Cham(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Deva(This, value) \
    ((This)->lpVtbl->get_Deva(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_FullWide(This, value) \
    ((This)->lpVtbl->get_FullWide(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Gujr(This, value) \
    ((This)->lpVtbl->get_Gujr(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Guru(This, value) \
    ((This)->lpVtbl->get_Guru(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_HaniDec(This, value) \
    ((This)->lpVtbl->get_HaniDec(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Java(This, value) \
    ((This)->lpVtbl->get_Java(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Kali(This, value) \
    ((This)->lpVtbl->get_Kali(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Khmr(This, value) \
    ((This)->lpVtbl->get_Khmr(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Knda(This, value) \
    ((This)->lpVtbl->get_Knda(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Lana(This, value) \
    ((This)->lpVtbl->get_Lana(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_LanaTham(This, value) \
    ((This)->lpVtbl->get_LanaTham(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Laoo(This, value) \
    ((This)->lpVtbl->get_Laoo(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Latn(This, value) \
    ((This)->lpVtbl->get_Latn(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Lepc(This, value) \
    ((This)->lpVtbl->get_Lepc(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Limb(This, value) \
    ((This)->lpVtbl->get_Limb(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Mlym(This, value) \
    ((This)->lpVtbl->get_Mlym(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Mong(This, value) \
    ((This)->lpVtbl->get_Mong(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Mtei(This, value) \
    ((This)->lpVtbl->get_Mtei(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Mymr(This, value) \
    ((This)->lpVtbl->get_Mymr(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_MymrShan(This, value) \
    ((This)->lpVtbl->get_MymrShan(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Nkoo(This, value) \
    ((This)->lpVtbl->get_Nkoo(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Olck(This, value) \
    ((This)->lpVtbl->get_Olck(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Orya(This, value) \
    ((This)->lpVtbl->get_Orya(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Saur(This, value) \
    ((This)->lpVtbl->get_Saur(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Sund(This, value) \
    ((This)->lpVtbl->get_Sund(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Talu(This, value) \
    ((This)->lpVtbl->get_Talu(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_TamlDec(This, value) \
    ((This)->lpVtbl->get_TamlDec(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Telu(This, value) \
    ((This)->lpVtbl->get_Telu(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Thai(This, value) \
    ((This)->lpVtbl->get_Thai(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Tibt(This, value) \
    ((This)->lpVtbl->get_Tibt(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_get_Vaii(This, value) \
    ((This)->lpVtbl->get_Vaii(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.INumeralSystemIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumeralSystemIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_INumeralSystemIdentifiersStatics2[] = L"Windows.Globalization.INumeralSystemIdentifiersStatics2";
typedef struct __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Brah)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Osma)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MathBold)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MathDbl)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MathSans)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MathSanb)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MathMono)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZmthBold)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZmthDbl)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZmthSans)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZmthSanb)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ZmthMono)(__x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_Brah(This, value) \
    ((This)->lpVtbl->get_Brah(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_Osma(This, value) \
    ((This)->lpVtbl->get_Osma(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_MathBold(This, value) \
    ((This)->lpVtbl->get_MathBold(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_MathDbl(This, value) \
    ((This)->lpVtbl->get_MathDbl(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_MathSans(This, value) \
    ((This)->lpVtbl->get_MathSans(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_MathSanb(This, value) \
    ((This)->lpVtbl->get_MathSanb(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_MathMono(This, value) \
    ((This)->lpVtbl->get_MathMono(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_ZmthBold(This, value) \
    ((This)->lpVtbl->get_ZmthBold(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_ZmthDbl(This, value) \
    ((This)->lpVtbl->get_ZmthDbl(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_ZmthSans(This, value) \
    ((This)->lpVtbl->get_ZmthSans(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_ZmthSanb(This, value) \
    ((This)->lpVtbl->get_ZmthSanb(This, value))

#define __x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_get_ZmthMono(This, value) \
    ((This)->lpVtbl->get_ZmthMono(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CINumeralSystemIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.ITimeZoneOnCalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Calendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_ITimeZoneOnCalendar[] = L"Windows.Globalization.ITimeZoneOnCalendar";
typedef struct __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTimeZone)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ChangeTimeZone)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        HSTRING timeZoneId);
    HRESULT (STDMETHODCALLTYPE* TimeZoneAsFullString)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* TimeZoneAsString)(__x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar* This,
        INT32 idealLength,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendarVtbl;

interface __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_GetTimeZone(This, value) \
    ((This)->lpVtbl->GetTimeZone(This, value))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_ChangeTimeZone(This, timeZoneId) \
    ((This)->lpVtbl->ChangeTimeZone(This, timeZoneId))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_TimeZoneAsFullString(This, result) \
    ((This)->lpVtbl->TimeZoneAsFullString(This, result))

#define __x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_TimeZoneAsString(This, idealLength, result) \
    ((This)->lpVtbl->TimeZoneAsString(This, idealLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CITimeZoneOnCalendar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.ApplicationLanguages
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IApplicationLanguagesStatics2 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.IApplicationLanguagesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_ApplicationLanguages_DEFINED
#define RUNTIMECLASS_Windows_Globalization_ApplicationLanguages_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_ApplicationLanguages[] = L"Windows.Globalization.ApplicationLanguages";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Calendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.ICalendarFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.ICalendarFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.ICalendar ** Default Interface **
 *    Windows.Globalization.ITimeZoneOnCalendar
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Calendar_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Calendar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Calendar[] = L"Windows.Globalization.Calendar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.CalendarIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.ICalendarIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICalendarIdentifiersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICalendarIdentifiersStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_CalendarIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_CalendarIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_CalendarIdentifiers[] = L"Windows.Globalization.CalendarIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.ClockIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IClockIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_ClockIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_ClockIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_ClockIdentifiers[] = L"Windows.Globalization.ClockIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.CurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.ICurrencyAmountFactory interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.ICurrencyAmount ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Globalization_CurrencyAmount_DEFINED
#define RUNTIMECLASS_Windows_Globalization_CurrencyAmount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_CurrencyAmount[] = L"Windows.Globalization.CurrencyAmount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Globalization.CurrencyIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.ICurrencyIdentifiersStatics3 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICurrencyIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ICurrencyIdentifiersStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_CurrencyIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_CurrencyIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_CurrencyIdentifiers[] = L"Windows.Globalization.CurrencyIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.GeographicRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.IGeographicRegionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IGeographicRegionStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.IGeographicRegion ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_GeographicRegion_DEFINED
#define RUNTIMECLASS_Windows_Globalization_GeographicRegion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_GeographicRegion[] = L"Windows.Globalization.GeographicRegion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.JapanesePhoneme
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.IJapanesePhoneme ** Default Interface **
 *
 * Class Threading Model:  Single Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_JapanesePhoneme_DEFINED
#define RUNTIMECLASS_Windows_Globalization_JapanesePhoneme_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_JapanesePhoneme[] = L"Windows.Globalization.JapanesePhoneme";
#endif
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.JapanesePhoneticAnalyzer
 *
 * Introduced to Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.IJapanesePhoneticAnalyzerStatics interface starting with version 1.0 of the Windows.Globalization.GlobalizationJapanesePhoneticAnalyzerContract API contract
 *
 * Class Threading Model:  Single Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_JapanesePhoneticAnalyzer_DEFINED
#define RUNTIMECLASS_Windows_Globalization_JapanesePhoneticAnalyzer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_JapanesePhoneticAnalyzer[] = L"Windows.Globalization.JapanesePhoneticAnalyzer";
#endif
#endif // WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Language
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.ILanguageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.ILanguageStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ILanguageStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.ILanguageStatics3 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.ILanguage ** Default Interface **
 *    Windows.Globalization.ILanguageExtensionSubtags
 *    Windows.Globalization.ILanguage2
 *    Windows.Globalization.ILanguage3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Language_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Language_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Language[] = L"Windows.Globalization.Language";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumeralSystemIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.INumeralSystemIdentifiersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Globalization.INumeralSystemIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumeralSystemIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumeralSystemIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumeralSystemIdentifiers[] = L"Windows.Globalization.NumeralSystemIdentifiers";
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
#endif // __windows2Eglobalization_p_h__

#endif // __windows2Eglobalization_h__
