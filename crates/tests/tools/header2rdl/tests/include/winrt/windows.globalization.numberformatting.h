
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
#ifndef __windows2Eglobalization2Enumberformatting_h__
#define __windows2Eglobalization2Enumberformatting_h__
#ifndef __windows2Eglobalization2Enumberformatting_p_h__
#define __windows2Eglobalization2Enumberformatting_p_h__


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
#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface ICurrencyFormatter;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter ABI::Windows::Globalization::NumberFormatting::ICurrencyFormatter

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface ICurrencyFormatter2;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2 ABI::Windows::Globalization::NumberFormatting::ICurrencyFormatter2

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface ICurrencyFormatterFactory;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory ABI::Windows::Globalization::NumberFormatting::ICurrencyFormatterFactory

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface IDecimalFormatterFactory;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory ABI::Windows::Globalization::NumberFormatting::IDecimalFormatterFactory

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface IIncrementNumberRounder;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder ABI::Windows::Globalization::NumberFormatting::IIncrementNumberRounder

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumberFormatter;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter ABI::Windows::Globalization::NumberFormatting::INumberFormatter

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumberFormatter2;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2 ABI::Windows::Globalization::NumberFormatting::INumberFormatter2

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumberFormatterOptions;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions ABI::Windows::Globalization::NumberFormatting::INumberFormatterOptions

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumberParser;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser ABI::Windows::Globalization::NumberFormatting::INumberParser

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumberRounder;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder ABI::Windows::Globalization::NumberFormatting::INumberRounder

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumberRounderOption;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption ABI::Windows::Globalization::NumberFormatting::INumberRounderOption

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumeralSystemTranslator;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator ABI::Windows::Globalization::NumberFormatting::INumeralSystemTranslator

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface INumeralSystemTranslatorFactory;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory ABI::Windows::Globalization::NumberFormatting::INumeralSystemTranslatorFactory

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface IPercentFormatterFactory;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory ABI::Windows::Globalization::NumberFormatting::IPercentFormatterFactory

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface IPermilleFormatterFactory;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory ABI::Windows::Globalization::NumberFormatting::IPermilleFormatterFactory

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface ISignedZeroOption;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption ABI::Windows::Globalization::NumberFormatting::ISignedZeroOption

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface ISignificantDigitsNumberRounder;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder ABI::Windows::Globalization::NumberFormatting::ISignificantDigitsNumberRounder

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                interface ISignificantDigitsOption;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption ABI::Windows::Globalization::NumberFormatting::ISignificantDigitsOption

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_FWD_DEFINED__

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



#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */



#ifndef DEF___FIReference_1___z__zint64_USE
#define DEF___FIReference_1___z__zint64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4dda9e24-e69f-5c6a-a0a6-93427365af2a"))
IReference<__int64> : IReference_impl<__int64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<__int64> __FIReference_1___z__zint64_t;
#define __FIReference_1___z__zint64 ABI::Windows::Foundation::__FIReference_1___z__zint64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1___z__zint64_USE */



#ifndef DEF___FIReference_1_UINT64_USE
#define DEF___FIReference_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6755e376-53bb-568b-a11d-17239868309e"))
IReference<UINT64> : IReference_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT64> __FIReference_1_UINT64_t;
#define __FIReference_1_UINT64 ABI::Windows::Foundation::__FIReference_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT64_USE */


#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValue;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValue ABI::Windows::Foundation::IPropertyValue

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                typedef enum CurrencyFormatterMode : int CurrencyFormatterMode;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                typedef enum RoundingAlgorithm : int RoundingAlgorithm;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                class CurrencyFormatter;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                class DecimalFormatter;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                class NumeralSystemTranslator;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                class PercentFormatter;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                class PermilleFormatter;
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Globalization.NumberFormatting.CurrencyFormatterMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                enum CurrencyFormatterMode : int
                {
                    CurrencyFormatterMode_UseSymbol = 0,
                    CurrencyFormatterMode_UseCurrencyCode = 1,
                };
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.NumberFormatting.RoundingAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                enum RoundingAlgorithm : int
                {
                    RoundingAlgorithm_None = 0,
                    RoundingAlgorithm_RoundDown = 1,
                    RoundingAlgorithm_RoundUp = 2,
                    RoundingAlgorithm_RoundTowardsZero = 3,
                    RoundingAlgorithm_RoundAwayFromZero = 4,
                    RoundingAlgorithm_RoundHalfDown = 5,
                    RoundingAlgorithm_RoundHalfUp = 6,
                    RoundingAlgorithm_RoundHalfTowardsZero = 7,
                    RoundingAlgorithm_RoundHalfAwayFromZero = 8,
                    RoundingAlgorithm_RoundHalfToEven = 9,
                    RoundingAlgorithm_RoundHalfToOdd = 10,
                };
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ICurrencyFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *     Windows.Globalization.NumberFormatting.INumberFormatter
 *     Windows.Globalization.NumberFormatting.INumberFormatter2
 *     Windows.Globalization.NumberFormatting.INumberParser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ICurrencyFormatter[] = L"Windows.Globalization.NumberFormatting.ICurrencyFormatter";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("11730ca5-4b00-41b2-b332-73b12a497d54")
                ICurrencyFormatter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Currency(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Currency may be read-only for releases after Windows 8.1. Instead, use a new CurrencyFormatter.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Currency(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrencyFormatter = __uuidof(ICurrencyFormatter);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ICurrencyFormatter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ICurrencyFormatter2[] = L"Windows.Globalization.NumberFormatting.ICurrencyFormatter2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("072c2f1d-e7ba-4197-920e-247c92f7dea6")
                ICurrencyFormatter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Globalization::NumberFormatting::CurrencyFormatterMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Globalization::NumberFormatting::CurrencyFormatterMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ApplyRoundingForCurrency(
                        ABI::Windows::Globalization::NumberFormatting::RoundingAlgorithm roundingAlgorithm
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrencyFormatter2 = __uuidof(ICurrencyFormatter2);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ICurrencyFormatterFactory[] = L"Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("86c7537e-b938-4aa2-84b0-2c33dc5b1450")
                ICurrencyFormatterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCurrencyFormatterCode(
                        HSTRING currencyCode,
                        ABI::Windows::Globalization::NumberFormatting::ICurrencyFormatter** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCurrencyFormatterCodeContext(
                        HSTRING currencyCode,
                        __FIIterable_1_HSTRING* languages,
                        HSTRING geographicRegion,
                        ABI::Windows::Globalization::NumberFormatting::ICurrencyFormatter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrencyFormatterFactory = __uuidof(ICurrencyFormatterFactory);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IDecimalFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.DecimalFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IDecimalFormatterFactory[] = L"Windows.Globalization.NumberFormatting.IDecimalFormatterFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("0d018c9a-e393-46b8-b830-7a69c8f89fbb")
                IDecimalFormatterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDecimalFormatter(
                        __FIIterable_1_HSTRING* languages,
                        HSTRING geographicRegion,
                        ABI::Windows::Globalization::NumberFormatting::INumberFormatter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDecimalFormatterFactory = __uuidof(IDecimalFormatterFactory);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IIncrementNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.IncrementNumberRounder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IIncrementNumberRounder[] = L"Windows.Globalization.NumberFormatting.IIncrementNumberRounder";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("70a64ff8-66ab-4155-9da1-739e46764543")
                IIncrementNumberRounder : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RoundingAlgorithm(
                        ABI::Windows::Globalization::NumberFormatting::RoundingAlgorithm* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoundingAlgorithm(
                        ABI::Windows::Globalization::NumberFormatting::RoundingAlgorithm value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Increment(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Increment(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIncrementNumberRounder = __uuidof(IIncrementNumberRounder);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberFormatter[] = L"Windows.Globalization.NumberFormatting.INumberFormatter";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("a5007c49-7676-4db7-8631-1b6ff265caa9")
                INumberFormatter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FormatInt(
                        INT64 value,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FormatUInt(
                        UINT64 value,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FormatDouble(
                        DOUBLE value,
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumberFormatter = __uuidof(INumberFormatter);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberFormatter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberFormatter2[] = L"Windows.Globalization.NumberFormatting.INumberFormatter2";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("d4a8c1f0-80d0-4b0d-a89e-882c1e8f8310")
                INumberFormatter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FormatInt(
                        INT64 value,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FormatUInt(
                        UINT64 value,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FormatDouble(
                        DOUBLE value,
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumberFormatter2 = __uuidof(INumberFormatter2);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberFormatterOptions[] = L"Windows.Globalization.NumberFormatting.INumberFormatterOptions";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("80332d21-aee1-4a39-baa2-07ed8c96daf6")
                INumberFormatterOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Languages(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GeographicRegion(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IntegerDigits(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IntegerDigits(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FractionDigits(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FractionDigits(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGrouped(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsGrouped(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDecimalPointAlwaysDisplayed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsDecimalPointAlwaysDisplayed(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumeralSystem(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NumeralSystem(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedGeographicRegion(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumberFormatterOptions = __uuidof(INumberFormatterOptions);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberParser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberParser[] = L"Windows.Globalization.NumberFormatting.INumberParser";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("e6659412-4a13-4a53-83a1-392fbe4cff9f")
                INumberParser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ParseInt(
                        HSTRING text,
                        __FIReference_1___z__zint64** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ParseUInt(
                        HSTRING text,
                        __FIReference_1_UINT64** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ParseDouble(
                        HSTRING text,
                        __FIReference_1_double** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumberParser = __uuidof(INumberParser);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberRounder[] = L"Windows.Globalization.NumberFormatting.INumberRounder";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("5473c375-38ed-4631-b80c-ef34fc48b7f5")
                INumberRounder : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RoundInt32(
                        INT32 value,
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RoundUInt32(
                        UINT32 value,
                        UINT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RoundInt64(
                        INT64 value,
                        INT64* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RoundUInt64(
                        UINT64 value,
                        UINT64* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RoundSingle(
                        FLOAT value,
                        FLOAT* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RoundDouble(
                        DOUBLE value,
                        DOUBLE* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumberRounder = __uuidof(INumberRounder);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberRounderOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberRounderOption[] = L"Windows.Globalization.NumberFormatting.INumberRounderOption";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("3b088433-646f-4efe-8d48-66eb2e49e736")
                INumberRounderOption : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NumberRounder(
                        ABI::Windows::Globalization::NumberFormatting::INumberRounder** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NumberRounder(
                        ABI::Windows::Globalization::NumberFormatting::INumberRounder* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumberRounderOption = __uuidof(INumberRounderOption);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumeralSystemTranslator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.NumeralSystemTranslator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumeralSystemTranslator[] = L"Windows.Globalization.NumberFormatting.INumeralSystemTranslator";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("28f5bc2c-8c23-4234-ad2e-fa5a3a426e9b")
                INumeralSystemTranslator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Languages(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumeralSystem(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NumeralSystem(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TranslateNumerals(
                        HSTRING value,
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumeralSystemTranslator = __uuidof(INumeralSystemTranslator);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.NumeralSystemTranslator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumeralSystemTranslatorFactory[] = L"Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("9630c8da-36ef-4d88-a85c-6f0d98d620a6")
                INumeralSystemTranslatorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1_HSTRING* languages,
                        ABI::Windows::Globalization::NumberFormatting::INumeralSystemTranslator** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INumeralSystemTranslatorFactory = __uuidof(INumeralSystemTranslatorFactory);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IPercentFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.PercentFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IPercentFormatterFactory[] = L"Windows.Globalization.NumberFormatting.IPercentFormatterFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("b7828aef-fed4-4018-a6e2-e09961e03765")
                IPercentFormatterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePercentFormatter(
                        __FIIterable_1_HSTRING* languages,
                        HSTRING geographicRegion,
                        ABI::Windows::Globalization::NumberFormatting::INumberFormatter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPercentFormatterFactory = __uuidof(IPercentFormatterFactory);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IPermilleFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.PermilleFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IPermilleFormatterFactory[] = L"Windows.Globalization.NumberFormatting.IPermilleFormatterFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("2b37b4ac-e638-4ed5-a998-62f6b06a49ae")
                IPermilleFormatterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePermilleFormatter(
                        __FIIterable_1_HSTRING* languages,
                        HSTRING geographicRegion,
                        ABI::Windows::Globalization::NumberFormatting::INumberFormatter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPermilleFormatterFactory = __uuidof(IPermilleFormatterFactory);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ISignedZeroOption[] = L"Windows.Globalization.NumberFormatting.ISignedZeroOption";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("fd1cdd31-0a3c-49c4-a642-96a1564f4f30")
                ISignedZeroOption : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsZeroSigned(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsZeroSigned(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISignedZeroOption = __uuidof(ISignedZeroOption);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ISignificantDigitsNumberRounder[] = L"Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("f5941bca-6646-4913-8c76-1b191ff94dfd")
                ISignificantDigitsNumberRounder : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RoundingAlgorithm(
                        ABI::Windows::Globalization::NumberFormatting::RoundingAlgorithm* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoundingAlgorithm(
                        ABI::Windows::Globalization::NumberFormatting::RoundingAlgorithm value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignificantDigits(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignificantDigits(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISignificantDigitsNumberRounder = __uuidof(ISignificantDigitsNumberRounder);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ISignificantDigitsOption[] = L"Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace NumberFormatting {
                MIDL_INTERFACE("1d4dfcdd-2d43-4ee8-bbf1-c1b26a711a58")
                ISignificantDigitsOption : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SignificantDigits(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignificantDigits(
                        INT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISignificantDigitsOption = __uuidof(ISignificantDigitsOption);
            } /* NumberFormatting */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.ICurrencyFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberFormatter
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.ICurrencyFormatter2
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_CurrencyFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_CurrencyFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_CurrencyFormatter[] = L"Windows.Globalization.NumberFormatting.CurrencyFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.DecimalFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.NumberFormatting.IDecimalFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.INumberFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_DecimalFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_DecimalFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_DecimalFormatter[] = L"Windows.Globalization.NumberFormatting.DecimalFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.IncrementNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberRounder ** Default Interface **
 *    Windows.Globalization.NumberFormatting.IIncrementNumberRounder
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_IncrementNumberRounder_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_IncrementNumberRounder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_IncrementNumberRounder[] = L"Windows.Globalization.NumberFormatting.IncrementNumberRounder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.NumeralSystemTranslator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumeralSystemTranslator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_NumeralSystemTranslator_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_NumeralSystemTranslator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_NumeralSystemTranslator[] = L"Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.PercentFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.NumberFormatting.IPercentFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.INumberFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_PercentFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_PercentFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_PercentFormatter[] = L"Windows.Globalization.NumberFormatting.PercentFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.PermilleFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.NumberFormatting.IPermilleFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.INumberFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_PermilleFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_PermilleFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_PermilleFormatter[] = L"Windows.Globalization.NumberFormatting.PermilleFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberRounder ** Default Interface **
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_SignificantDigitsNumberRounder_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_SignificantDigitsNumberRounder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_SignificantDigitsNumberRounder[] = L"Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2 __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2 __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption;

#endif // ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_FWD_DEFINED__

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

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

#if !defined(____FIReference_1___z__zint64_INTERFACE_DEFINED__)
#define ____FIReference_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIReference_1___z__zint64 __FIReference_1___z__zint64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1___z__zint64;

typedef struct __FIReference_1___z__zint64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1___z__zint64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1___z__zint64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1___z__zint64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1___z__zint64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1___z__zint64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1___z__zint64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1___z__zint64* This,
        INT64* result);

    END_INTERFACE
} __FIReference_1___z__zint64Vtbl;

interface __FIReference_1___z__zint64
{
    CONST_VTBL struct __FIReference_1___z__zint64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1___z__zint64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1___z__zint64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1___z__zint64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1___z__zint64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1___z__zint64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1___z__zint64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1___z__zint64_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1___z__zint64_INTERFACE_DEFINED__

#if !defined(____FIReference_1_UINT64_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT64 __FIReference_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT64;

typedef struct __FIReference_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT64* This,
        UINT64* result);

    END_INTERFACE
} __FIReference_1_UINT64Vtbl;

interface __FIReference_1_UINT64
{
    CONST_VTBL struct __FIReference_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT64_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT64_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CCurrencyFormatterMode __x_ABI_CWindows_CGlobalization_CNumberFormatting_CCurrencyFormatterMode;

typedef enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm;

/*
 *
 * Struct Windows.Globalization.NumberFormatting.CurrencyFormatterMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CCurrencyFormatterMode
{
    CurrencyFormatterMode_UseSymbol = 0,
    CurrencyFormatterMode_UseCurrencyCode = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Globalization.NumberFormatting.RoundingAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm
{
    RoundingAlgorithm_None = 0,
    RoundingAlgorithm_RoundDown = 1,
    RoundingAlgorithm_RoundUp = 2,
    RoundingAlgorithm_RoundTowardsZero = 3,
    RoundingAlgorithm_RoundAwayFromZero = 4,
    RoundingAlgorithm_RoundHalfDown = 5,
    RoundingAlgorithm_RoundHalfUp = 6,
    RoundingAlgorithm_RoundHalfTowardsZero = 7,
    RoundingAlgorithm_RoundHalfAwayFromZero = 8,
    RoundingAlgorithm_RoundHalfToEven = 9,
    RoundingAlgorithm_RoundHalfToOdd = 10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ICurrencyFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *     Windows.Globalization.NumberFormatting.INumberFormatter
 *     Windows.Globalization.NumberFormatting.INumberFormatter2
 *     Windows.Globalization.NumberFormatting.INumberParser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ICurrencyFormatter[] = L"Windows.Globalization.NumberFormatting.ICurrencyFormatter";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Currency)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Currency may be read-only for releases after Windows 8.1. Instead, use a new CurrencyFormatter.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Currency)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_get_Currency(This, value) \
    ((This)->lpVtbl->get_Currency(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Currency may be read-only for releases after Windows 8.1. Instead, use a new CurrencyFormatter.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_put_Currency(This, value) \
    ((This)->lpVtbl->put_Currency(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ICurrencyFormatter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ICurrencyFormatter2[] = L"Windows.Globalization.NumberFormatting.ICurrencyFormatter2";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CCurrencyFormatterMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CCurrencyFormatterMode value);
    HRESULT (STDMETHODCALLTYPE* ApplyRoundingForCurrency)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm roundingAlgorithm);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_ApplyRoundingForCurrency(This, roundingAlgorithm) \
    ((This)->lpVtbl->ApplyRoundingForCurrency(This, roundingAlgorithm))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ICurrencyFormatterFactory[] = L"Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCurrencyFormatterCode)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This,
        HSTRING currencyCode,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter** result);
    HRESULT (STDMETHODCALLTYPE* CreateCurrencyFormatterCodeContext)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory* This,
        HSTRING currencyCode,
        __FIIterable_1_HSTRING* languages,
        HSTRING geographicRegion,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatter** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_CreateCurrencyFormatterCode(This, currencyCode, result) \
    ((This)->lpVtbl->CreateCurrencyFormatterCode(This, currencyCode, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_CreateCurrencyFormatterCodeContext(This, currencyCode, languages, geographicRegion, result) \
    ((This)->lpVtbl->CreateCurrencyFormatterCodeContext(This, currencyCode, languages, geographicRegion, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CICurrencyFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IDecimalFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.DecimalFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IDecimalFormatterFactory[] = L"Windows.Globalization.NumberFormatting.IDecimalFormatterFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDecimalFormatter)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory* This,
        __FIIterable_1_HSTRING* languages,
        HSTRING geographicRegion,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_CreateDecimalFormatter(This, languages, geographicRegion, result) \
    ((This)->lpVtbl->CreateDecimalFormatter(This, languages, geographicRegion, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIDecimalFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IIncrementNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.IncrementNumberRounder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IIncrementNumberRounder[] = L"Windows.Globalization.NumberFormatting.IIncrementNumberRounder";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RoundingAlgorithm)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm* value);
    HRESULT (STDMETHODCALLTYPE* put_RoundingAlgorithm)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm value);
    HRESULT (STDMETHODCALLTYPE* get_Increment)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Increment)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounderVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_get_RoundingAlgorithm(This, value) \
    ((This)->lpVtbl->get_RoundingAlgorithm(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_put_RoundingAlgorithm(This, value) \
    ((This)->lpVtbl->put_RoundingAlgorithm(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_get_Increment(This, value) \
    ((This)->lpVtbl->get_Increment(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_put_Increment(This, value) \
    ((This)->lpVtbl->put_Increment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIIncrementNumberRounder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberFormatter[] = L"Windows.Globalization.NumberFormatting.INumberFormatter";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FormatInt)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        INT64 value,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FormatUInt)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        UINT64 value,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FormatDouble)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter* This,
        DOUBLE value,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FormatInt(This, value, result) \
    ((This)->lpVtbl->FormatInt(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FormatUInt(This, value, result) \
    ((This)->lpVtbl->FormatUInt(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_FormatDouble(This, value, result) \
    ((This)->lpVtbl->FormatDouble(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberFormatter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberFormatter2[] = L"Windows.Globalization.NumberFormatting.INumberFormatter2";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FormatInt)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        INT64 value,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FormatUInt)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        UINT64 value,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FormatDouble)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2* This,
        DOUBLE value,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2Vtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FormatInt(This, value, result) \
    ((This)->lpVtbl->FormatInt(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FormatUInt(This, value, result) \
    ((This)->lpVtbl->FormatUInt(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_FormatDouble(This, value, result) \
    ((This)->lpVtbl->FormatDouble(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberFormatterOptions[] = L"Windows.Globalization.NumberFormatting.INumberFormatterOptions";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_GeographicRegion)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IntegerDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_IntegerDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_FractionDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_FractionDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_IsGrouped)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsGrouped)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsDecimalPointAlwaysDisplayed)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsDecimalPointAlwaysDisplayed)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedGeographicRegion)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptionsVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_GeographicRegion(This, value) \
    ((This)->lpVtbl->get_GeographicRegion(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_IntegerDigits(This, value) \
    ((This)->lpVtbl->get_IntegerDigits(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_put_IntegerDigits(This, value) \
    ((This)->lpVtbl->put_IntegerDigits(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_FractionDigits(This, value) \
    ((This)->lpVtbl->get_FractionDigits(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_put_FractionDigits(This, value) \
    ((This)->lpVtbl->put_FractionDigits(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_IsGrouped(This, value) \
    ((This)->lpVtbl->get_IsGrouped(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_put_IsGrouped(This, value) \
    ((This)->lpVtbl->put_IsGrouped(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_IsDecimalPointAlwaysDisplayed(This, value) \
    ((This)->lpVtbl->get_IsDecimalPointAlwaysDisplayed(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_put_IsDecimalPointAlwaysDisplayed(This, value) \
    ((This)->lpVtbl->put_IsDecimalPointAlwaysDisplayed(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_NumeralSystem(This, value) \
    ((This)->lpVtbl->get_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_put_NumeralSystem(This, value) \
    ((This)->lpVtbl->put_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_get_ResolvedGeographicRegion(This, value) \
    ((This)->lpVtbl->get_ResolvedGeographicRegion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatterOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberParser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberParser[] = L"Windows.Globalization.NumberFormatting.INumberParser";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ParseInt)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        HSTRING text,
        __FIReference_1___z__zint64** result);
    HRESULT (STDMETHODCALLTYPE* ParseUInt)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        HSTRING text,
        __FIReference_1_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* ParseDouble)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser* This,
        HSTRING text,
        __FIReference_1_double** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParserVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_ParseInt(This, text, result) \
    ((This)->lpVtbl->ParseInt(This, text, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_ParseUInt(This, text, result) \
    ((This)->lpVtbl->ParseUInt(This, text, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_ParseDouble(This, text, result) \
    ((This)->lpVtbl->ParseDouble(This, text, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberParser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberRounder[] = L"Windows.Globalization.NumberFormatting.INumberRounder";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RoundInt32)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        INT32 value,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* RoundUInt32)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        UINT32 value,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* RoundInt64)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        INT64 value,
        INT64* result);
    HRESULT (STDMETHODCALLTYPE* RoundUInt64)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        UINT64 value,
        UINT64* result);
    HRESULT (STDMETHODCALLTYPE* RoundSingle)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        FLOAT value,
        FLOAT* result);
    HRESULT (STDMETHODCALLTYPE* RoundDouble)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* This,
        DOUBLE value,
        DOUBLE* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_RoundInt32(This, value, result) \
    ((This)->lpVtbl->RoundInt32(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_RoundUInt32(This, value, result) \
    ((This)->lpVtbl->RoundUInt32(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_RoundInt64(This, value, result) \
    ((This)->lpVtbl->RoundInt64(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_RoundUInt64(This, value, result) \
    ((This)->lpVtbl->RoundUInt64(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_RoundSingle(This, value, result) \
    ((This)->lpVtbl->RoundSingle(This, value, result))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_RoundDouble(This, value, result) \
    ((This)->lpVtbl->RoundDouble(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumberRounderOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumberRounderOption[] = L"Windows.Globalization.NumberFormatting.INumberRounderOption";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NumberRounder)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder** value);
    HRESULT (STDMETHODCALLTYPE* put_NumberRounder)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption* This,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounder* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOptionVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_get_NumberRounder(This, value) \
    ((This)->lpVtbl->get_NumberRounder(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_put_NumberRounder(This, value) \
    ((This)->lpVtbl->put_NumberRounder(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberRounderOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumeralSystemTranslator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.NumeralSystemTranslator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumeralSystemTranslator[] = L"Windows.Globalization.NumberFormatting.INumeralSystemTranslator";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NumeralSystem)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* TranslateNumerals)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator* This,
        HSTRING value,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_get_NumeralSystem(This, value) \
    ((This)->lpVtbl->get_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_put_NumeralSystem(This, value) \
    ((This)->lpVtbl->put_NumeralSystem(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_TranslateNumerals(This, value, result) \
    ((This)->lpVtbl->TranslateNumerals(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.NumeralSystemTranslator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_INumeralSystemTranslatorFactory[] = L"Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory* This,
        __FIIterable_1_HSTRING* languages,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslator** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_Create(This, languages, result) \
    ((This)->lpVtbl->Create(This, languages, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumeralSystemTranslatorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IPercentFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.PercentFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IPercentFormatterFactory[] = L"Windows.Globalization.NumberFormatting.IPercentFormatterFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePercentFormatter)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory* This,
        __FIIterable_1_HSTRING* languages,
        HSTRING geographicRegion,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_CreatePercentFormatter(This, languages, geographicRegion, result) \
    ((This)->lpVtbl->CreatePercentFormatter(This, languages, geographicRegion, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPercentFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.IPermilleFormatterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.PermilleFormatter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_IPermilleFormatterFactory[] = L"Windows.Globalization.NumberFormatting.IPermilleFormatterFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePermilleFormatter)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory* This,
        __FIIterable_1_HSTRING* languages,
        HSTRING geographicRegion,
        __x_ABI_CWindows_CGlobalization_CNumberFormatting_CINumberFormatter** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_CreatePermilleFormatter(This, languages, geographicRegion, result) \
    ((This)->lpVtbl->CreatePermilleFormatter(This, languages, geographicRegion, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CIPermilleFormatterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ISignedZeroOption[] = L"Windows.Globalization.NumberFormatting.ISignedZeroOption";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsZeroSigned)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsZeroSigned)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOptionVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_get_IsZeroSigned(This, value) \
    ((This)->lpVtbl->get_IsZeroSigned(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_put_IsZeroSigned(This, value) \
    ((This)->lpVtbl->put_IsZeroSigned(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignedZeroOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ISignificantDigitsNumberRounder[] = L"Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RoundingAlgorithm)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm* value);
    HRESULT (STDMETHODCALLTYPE* put_RoundingAlgorithm)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        enum __x_ABI_CWindows_CGlobalization_CNumberFormatting_CRoundingAlgorithm value);
    HRESULT (STDMETHODCALLTYPE* get_SignificantDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_SignificantDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounderVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_get_RoundingAlgorithm(This, value) \
    ((This)->lpVtbl->get_RoundingAlgorithm(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_put_RoundingAlgorithm(This, value) \
    ((This)->lpVtbl->put_RoundingAlgorithm(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_get_SignificantDigits(This, value) \
    ((This)->lpVtbl->get_SignificantDigits(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_put_SignificantDigits(This, value) \
    ((This)->lpVtbl->put_SignificantDigits(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsNumberRounder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_NumberFormatting_ISignificantDigitsOption[] = L"Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
typedef struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SignificantDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_SignificantDigits)(__x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOptionVtbl;

interface __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_get_SignificantDigits(This, value) \
    ((This)->lpVtbl->get_SignificantDigits(This, value))

#define __x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_put_SignificantDigits(This, value) \
    ((This)->lpVtbl->put_SignificantDigits(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CNumberFormatting_CISignificantDigitsOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.CurrencyFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.ICurrencyFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberFormatter
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.ICurrencyFormatter2
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_CurrencyFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_CurrencyFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_CurrencyFormatter[] = L"Windows.Globalization.NumberFormatting.CurrencyFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.DecimalFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.NumberFormatting.IDecimalFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.INumberFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_DecimalFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_DecimalFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_DecimalFormatter[] = L"Windows.Globalization.NumberFormatting.DecimalFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.IncrementNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberRounder ** Default Interface **
 *    Windows.Globalization.NumberFormatting.IIncrementNumberRounder
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_IncrementNumberRounder_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_IncrementNumberRounder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_IncrementNumberRounder[] = L"Windows.Globalization.NumberFormatting.IncrementNumberRounder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.NumeralSystemTranslator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumeralSystemTranslator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_NumeralSystemTranslator_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_NumeralSystemTranslator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_NumeralSystemTranslator[] = L"Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.PercentFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.NumberFormatting.IPercentFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.INumberFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_PercentFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_PercentFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_PercentFormatter[] = L"Windows.Globalization.NumberFormatting.PercentFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.PermilleFormatter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Globalization.NumberFormatting.IPermilleFormatterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberFormatterOptions
 *    Windows.Globalization.NumberFormatting.INumberFormatter ** Default Interface **
 *    Windows.Globalization.NumberFormatting.INumberFormatter2
 *    Windows.Globalization.NumberFormatting.INumberParser
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsOption
 *    Windows.Globalization.NumberFormatting.INumberRounderOption
 *    Windows.Globalization.NumberFormatting.ISignedZeroOption
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_PermilleFormatter_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_PermilleFormatter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_PermilleFormatter[] = L"Windows.Globalization.NumberFormatting.PermilleFormatter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.NumberFormatting.INumberRounder ** Default Interface **
 *    Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_NumberFormatting_SignificantDigitsNumberRounder_DEFINED
#define RUNTIMECLASS_Windows_Globalization_NumberFormatting_SignificantDigitsNumberRounder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_NumberFormatting_SignificantDigitsNumberRounder[] = L"Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
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
#endif // __windows2Eglobalization2Enumberformatting_p_h__

#endif // __windows2Eglobalization2Enumberformatting_h__
