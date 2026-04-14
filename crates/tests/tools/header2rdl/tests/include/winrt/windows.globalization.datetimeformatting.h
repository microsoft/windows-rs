
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
#ifndef __windows2Eglobalization2Edatetimeformatting_h__
#define __windows2Eglobalization2Edatetimeformatting_h__
#ifndef __windows2Eglobalization2Edatetimeformatting_p_h__
#define __windows2Eglobalization2Edatetimeformatting_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                interface IDateTimeFormatter;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                interface IDateTimeFormatter2;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2 ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter2

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                interface IDateTimeFormatterFactory;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatterFactory

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                interface IDateTimeFormatterStatics;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatterStatics

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum DayFormat : int DayFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum DayOfWeekFormat : int DayOfWeekFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum HourFormat : int HourFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum MinuteFormat : int MinuteFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum MonthFormat : int MonthFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum SecondFormat : int SecondFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                typedef enum YearFormat : int YearFormat;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                class DateTimeFormatter;
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.DayFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum DayFormat : int
                {
                    DayFormat_None = 0,
                    DayFormat_Default = 1,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.DayOfWeekFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum DayOfWeekFormat : int
                {
                    DayOfWeekFormat_None = 0,
                    DayOfWeekFormat_Default = 1,
                    DayOfWeekFormat_Abbreviated = 2,
                    DayOfWeekFormat_Full = 3,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.HourFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum HourFormat : int
                {
                    HourFormat_None = 0,
                    HourFormat_Default = 1,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.MinuteFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum MinuteFormat : int
                {
                    MinuteFormat_None = 0,
                    MinuteFormat_Default = 1,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.MonthFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum MonthFormat : int
                {
                    MonthFormat_None = 0,
                    MonthFormat_Default = 1,
                    MonthFormat_Abbreviated = 2,
                    MonthFormat_Full = 3,
                    MonthFormat_Numeric = 4,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.SecondFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum SecondFormat : int
                {
                    SecondFormat_None = 0,
                    SecondFormat_Default = 1,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.YearFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                enum YearFormat : int
                {
                    YearFormat_None = 0,
                    YearFormat_Default = 1,
                    YearFormat_Abbreviated = 2,
                    YearFormat_Full = 3,
                };
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatter[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatter";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                MIDL_INTERFACE("95eeca10-73e0-4e4b-a183-3d6ad0ba35ec")
                IDateTimeFormatter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Languages(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GeographicRegion(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Calendar(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Clock(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumeralSystem(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NumeralSystem(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Patterns(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Template(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Format(
                        ABI::Windows::Foundation::DateTime value,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeYear(
                        ABI::Windows::Globalization::DateTimeFormatting::YearFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeMonth(
                        ABI::Windows::Globalization::DateTimeFormatting::MonthFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeDayOfWeek(
                        ABI::Windows::Globalization::DateTimeFormatting::DayOfWeekFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeDay(
                        ABI::Windows::Globalization::DateTimeFormatting::DayFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeHour(
                        ABI::Windows::Globalization::DateTimeFormatting::HourFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeMinute(
                        ABI::Windows::Globalization::DateTimeFormatting::MinuteFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeSecond(
                        ABI::Windows::Globalization::DateTimeFormatting::SecondFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedGeographicRegion(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDateTimeFormatter = __uuidof(IDateTimeFormatter);
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatter2[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                MIDL_INTERFACE("27c91a86-bdaa-4fd0-9e36-671d5aa5ee03")
                IDateTimeFormatter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FormatUsingTimeZone(
                        ABI::Windows::Foundation::DateTime datetime,
                        HSTRING timeZoneId,
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDateTimeFormatter2 = __uuidof(IDateTimeFormatter2);
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatterFactory[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                MIDL_INTERFACE("ec8d8a53-1a2e-412d-8815-3b745fb1a2a0")
                IDateTimeFormatterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatter(
                        HSTRING formatTemplate,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatterLanguages(
                        HSTRING formatTemplate,
                        __FIIterable_1_HSTRING* languages,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatterContext(
                        HSTRING formatTemplate,
                        __FIIterable_1_HSTRING* languages,
                        HSTRING geographicRegion,
                        HSTRING calendar,
                        HSTRING clock,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatterDate(
                        ABI::Windows::Globalization::DateTimeFormatting::YearFormat yearFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::MonthFormat monthFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::DayFormat dayFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::DayOfWeekFormat dayOfWeekFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatterTime(
                        ABI::Windows::Globalization::DateTimeFormatting::HourFormat hourFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::MinuteFormat minuteFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::SecondFormat secondFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatterDateTimeLanguages(
                        ABI::Windows::Globalization::DateTimeFormatting::YearFormat yearFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::MonthFormat monthFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::DayFormat dayFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::DayOfWeekFormat dayOfWeekFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::HourFormat hourFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::MinuteFormat minuteFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::SecondFormat secondFormat,
                        __FIIterable_1_HSTRING* languages,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeFormatterDateTimeContext(
                        ABI::Windows::Globalization::DateTimeFormatting::YearFormat yearFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::MonthFormat monthFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::DayFormat dayFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::DayOfWeekFormat dayOfWeekFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::HourFormat hourFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::MinuteFormat minuteFormat,
                        ABI::Windows::Globalization::DateTimeFormatting::SecondFormat secondFormat,
                        __FIIterable_1_HSTRING* languages,
                        HSTRING geographicRegion,
                        HSTRING calendar,
                        HSTRING clock,
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDateTimeFormatterFactory = __uuidof(IDateTimeFormatterFactory);
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatterStatics[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace DateTimeFormatting {
                MIDL_INTERFACE("bfcde7c0-df4c-4a2e-9012-f47daf3f1212")
                IDateTimeFormatterStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LongDate(
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LongTime(
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShortDate(
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShortTime(
                        ABI::Windows::Globalization::DateTimeFormatting::IDateTimeFormatter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDateTimeFormatterStatics = __uuidof(IDateTimeFormatterStatics);
            } /* DateTimeFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.DateTimeFormatting.IDateTimeFormatter ** Default Interface **
 *    Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_DateTimeFormatting_DateTimeFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_DateTimeFormatting_DateTimeFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_DateTimeFormatting_DateTimeFormatter[] = L"Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter;

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2 __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2;

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics;

#endif // ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_FWD_DEFINED__

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

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat;

typedef enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat;

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.DayFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat
{
    DayFormat_None = 0,
    DayFormat_Default = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.DayOfWeekFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat
{
    DayOfWeekFormat_None = 0,
    DayOfWeekFormat_Default = 1,
    DayOfWeekFormat_Abbreviated = 2,
    DayOfWeekFormat_Full = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.HourFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat
{
    HourFormat_None = 0,
    HourFormat_Default = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.MinuteFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat
{
    MinuteFormat_None = 0,
    MinuteFormat_Default = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.MonthFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat
{
    MonthFormat_None = 0,
    MonthFormat_Default = 1,
    MonthFormat_Abbreviated = 2,
    MonthFormat_Full = 3,
    MonthFormat_Numeric = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.SecondFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat
{
    SecondFormat_None = 0,
    SecondFormat_Default = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.DateTimeFormatting.YearFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat
{
    YearFormat_None = 0,
    YearFormat_Default = 1,
    YearFormat_Abbreviated = 2,
    YearFormat_Full = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatter[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatter";
typedef struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_GeographicRegion)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Calendar)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Clock)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Patterns)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Template)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* Format)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_IncludeYear)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeMonth)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeDayOfWeek)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeDay)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeHour)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeMinute)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeSecond)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedGeographicRegion)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterVtbl;

interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_GeographicRegion(This, value) \
    ((This)->lpVtbl->get_GeographicRegion(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_Calendar(This, value) \
    ((This)->lpVtbl->get_Calendar(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_Clock(This, value) \
    ((This)->lpVtbl->get_Clock(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_NumeralSystem(This, value) \
    ((This)->lpVtbl->get_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_put_NumeralSystem(This, value) \
    ((This)->lpVtbl->put_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_Patterns(This, value) \
    ((This)->lpVtbl->get_Patterns(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_Template(This, value) \
    ((This)->lpVtbl->get_Template(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_Format(This, value, result) \
    ((This)->lpVtbl->Format(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeYear(This, value) \
    ((This)->lpVtbl->get_IncludeYear(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeMonth(This, value) \
    ((This)->lpVtbl->get_IncludeMonth(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeDayOfWeek(This, value) \
    ((This)->lpVtbl->get_IncludeDayOfWeek(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeDay(This, value) \
    ((This)->lpVtbl->get_IncludeDay(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeHour(This, value) \
    ((This)->lpVtbl->get_IncludeHour(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeMinute(This, value) \
    ((This)->lpVtbl->get_IncludeMinute(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_IncludeSecond(This, value) \
    ((This)->lpVtbl->get_IncludeSecond(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_get_ResolvedGeographicRegion(This, value) \
    ((This)->lpVtbl->get_ResolvedGeographicRegion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatter2[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2";
typedef struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FormatUsingTimeZone)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime datetime,
        HSTRING timeZoneId,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_FormatUsingTimeZone(This, datetime, timeZoneId, result) \
    ((This)->lpVtbl->FormatUsingTimeZone(This, datetime, timeZoneId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatterFactory[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatter)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        HSTRING formatTemplate,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatterLanguages)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        HSTRING formatTemplate,
        __FIIterable_1_HSTRING* languages,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatterContext)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        HSTRING formatTemplate,
        __FIIterable_1_HSTRING* languages,
        HSTRING geographicRegion,
        HSTRING calendar,
        HSTRING clock,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatterDate)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat yearFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat monthFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat dayFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat dayOfWeekFormat,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatterTime)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat hourFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat minuteFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat secondFormat,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatterDateTimeLanguages)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat yearFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat monthFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat dayFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat dayOfWeekFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat hourFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat minuteFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat secondFormat,
        __FIIterable_1_HSTRING* languages,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeFormatterDateTimeContext)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory* This,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CYearFormat yearFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMonthFormat monthFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayFormat dayFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CDayOfWeekFormat dayOfWeekFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CHourFormat hourFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CMinuteFormat minuteFormat,
        enum __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CSecondFormat secondFormat,
        __FIIterable_1_HSTRING* languages,
        HSTRING geographicRegion,
        HSTRING calendar,
        HSTRING clock,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatter(This, formatTemplate, result) \
    ((This)->lpVtbl->CreateDateTimeFormatter(This, formatTemplate, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatterLanguages(This, formatTemplate, languages, result) \
    ((This)->lpVtbl->CreateDateTimeFormatterLanguages(This, formatTemplate, languages, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatterContext(This, formatTemplate, languages, geographicRegion, calendar, clock, result) \
    ((This)->lpVtbl->CreateDateTimeFormatterContext(This, formatTemplate, languages, geographicRegion, calendar, clock, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatterDate(This, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, result) \
    ((This)->lpVtbl->CreateDateTimeFormatterDate(This, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatterTime(This, hourFormat, minuteFormat, secondFormat, result) \
    ((This)->lpVtbl->CreateDateTimeFormatterTime(This, hourFormat, minuteFormat, secondFormat, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatterDateTimeLanguages(This, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages, result) \
    ((This)->lpVtbl->CreateDateTimeFormatterDateTimeLanguages(This, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages, result))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_CreateDateTimeFormatterDateTimeContext(This, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages, geographicRegion, calendar, clock, result) \
    ((This)->lpVtbl->CreateDateTimeFormatterDateTimeContext(This, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages, geographicRegion, calendar, clock, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_DateTimeFormatting_IDateTimeFormatterStatics[] = L"Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics";
typedef struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LongDate)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** value);
    HRESULT (STDMETHODCALLTYPE* get_LongTime)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** value);
    HRESULT (STDMETHODCALLTYPE* get_ShortDate)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** value);
    HRESULT (STDMETHODCALLTYPE* get_ShortTime)(__x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics* This,
        __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatter** value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStaticsVtbl;

interface __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_get_LongDate(This, value) \
    ((This)->lpVtbl->get_LongDate(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_get_LongTime(This, value) \
    ((This)->lpVtbl->get_LongTime(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_get_ShortDate(This, value) \
    ((This)->lpVtbl->get_ShortDate(This, value))

#define __x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_get_ShortTime(This, value) \
    ((This)->lpVtbl->get_ShortTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CDateTimeFormatting_CIDateTimeFormatterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.DateTimeFormatting.DateTimeFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.DateTimeFormatting.IDateTimeFormatter ** Default Interface **
 *    Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_DateTimeFormatting_DateTimeFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_DateTimeFormatting_DateTimeFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_DateTimeFormatting_DateTimeFormatter[] = L"Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
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
#endif // __windows2Eglobalization2Edatetimeformatting_p_h__

#endif // __windows2Eglobalization2Edatetimeformatting_h__
