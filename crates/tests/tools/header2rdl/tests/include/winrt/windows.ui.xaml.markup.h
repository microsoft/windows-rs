
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
#ifndef __windows2Eui2Examl2Emarkup_h__
#define __windows2Eui2Examl2Emarkup_h__
#ifndef __windows2Eui2Examl2Emarkup_p_h__
#define __windows2Eui2Examl2Emarkup_p_h__


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
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Interop.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IComponentConnector;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector ABI::Windows::UI::Xaml::Markup::IComponentConnector

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IComponentConnector2;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2 ABI::Windows::UI::Xaml::Markup::IComponentConnector2

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IDataTemplateComponent;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent ABI::Windows::UI::Xaml::Markup::IDataTemplateComponent

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IMarkupExtension;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension ABI::Windows::UI::Xaml::Markup::IMarkupExtension

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IMarkupExtensionFactory;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory ABI::Windows::UI::Xaml::Markup::IMarkupExtensionFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IMarkupExtensionOverrides;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides ABI::Windows::UI::Xaml::Markup::IMarkupExtensionOverrides

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlBinaryWriter;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter ABI::Windows::UI::Xaml::Markup::IXamlBinaryWriter

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlBinaryWriterStatics;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics ABI::Windows::UI::Xaml::Markup::IXamlBinaryWriterStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlBindScopeDiagnostics;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics ABI::Windows::UI::Xaml::Markup::IXamlBindScopeDiagnostics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlBindingHelper;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper ABI::Windows::UI::Xaml::Markup::IXamlBindingHelper

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlBindingHelperStatics;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics ABI::Windows::UI::Xaml::Markup::IXamlBindingHelperStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlMarkupHelper;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper ABI::Windows::UI::Xaml::Markup::IXamlMarkupHelper

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlMarkupHelperStatics;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics ABI::Windows::UI::Xaml::Markup::IXamlMarkupHelperStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlMember;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember ABI::Windows::UI::Xaml::Markup::IXamlMember

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlMetadataProvider;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider ABI::Windows::UI::Xaml::Markup::IXamlMetadataProvider

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlReader;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader ABI::Windows::UI::Xaml::Markup::IXamlReader

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlReaderStatics;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics ABI::Windows::UI::Xaml::Markup::IXamlReaderStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlType;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType ABI::Windows::UI::Xaml::Markup::IXamlType

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    interface IXamlType2;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2 ABI::Windows::UI::Xaml::Markup::IXamlType2

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c875446a-587f-58da-897e-3bbe5ec7c30b"))
IIterator<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IIterator_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ba666a00-1555-5df4-81a5-07d23f7ffceb"))
IIterable<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IIterable_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("92cd0a46-2266-5cd6-9293-e111299f2793"))
IVectorView<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IVectorView_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2736b66b-daa3-5e0c-9842-6a0f44b5440b"))
IVector<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IVector_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class DependencyObject;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IDependencyObject;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIDependencyObject ABI::Windows::UI::Xaml::IDependencyObject

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class DependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IDependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty ABI::Windows::UI::Xaml::IDependencyProperty

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    typedef struct TypeName TypeName;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class UIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IUIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIUIElement ABI::Windows::UI::Xaml::IUIElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    typedef struct XamlBinaryWriterErrorInformation XamlBinaryWriterErrorInformation;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    typedef struct XmlnsDefinition XmlnsDefinition;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    class MarkupExtension;
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Markup.XamlBinaryWriterErrorInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    struct XamlBinaryWriterErrorInformation
                    {
                        UINT32 InputStreamIndex;
                        UINT32 LineNumber;
                        UINT32 LinePosition;
                    };
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Markup.XmlnsDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    struct XmlnsDefinition
                    {
                        HSTRING XmlNamespace;
                        HSTRING Namespace;
                    };
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IComponentConnector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IComponentConnector[] = L"Windows.UI.Xaml.Markup.IComponentConnector";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("f6790987-e6e5-47f2-92c6-eccce4ba159a")
                    IComponentConnector : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Connect(
                            INT32 connectionId,
                            IInspectable* target
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IComponentConnector = __uuidof(IComponentConnector);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IComponentConnector2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IComponentConnector2[] = L"Windows.UI.Xaml.Markup.IComponentConnector2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("dc8f368b-eccc-498e-b139-91142254d7ae")
                    IComponentConnector2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetBindingConnector(
                            INT32 connectionId,
                            IInspectable* target,
                            ABI::Windows::UI::Xaml::Markup::IComponentConnector** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IComponentConnector2 = __uuidof(IComponentConnector2);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IDataTemplateComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IDataTemplateComponent[] = L"Windows.UI.Xaml.Markup.IDataTemplateComponent";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("08429dc8-8ab0-4747-aa9a-feadfc8da8e1")
                    IDataTemplateComponent : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Recycle(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProcessBindings(
                            IInspectable* item,
                            INT32 itemIndex,
                            INT32 phase,
                            INT32* nextPhase
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDataTemplateComponent = __uuidof(IDataTemplateComponent);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IMarkupExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.MarkupExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IMarkupExtension[] = L"Windows.UI.Xaml.Markup.IMarkupExtension";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("1ee3416d-562b-486e-9ee5-0f0cbcc8048c")
                    IMarkupExtension : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IMarkupExtension = __uuidof(IMarkupExtension);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IMarkupExtensionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.MarkupExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IMarkupExtensionFactory[] = L"Windows.UI.Xaml.Markup.IMarkupExtensionFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("65329c05-fb5a-4567-9d55-5cdfbada2739")
                    IMarkupExtensionFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Markup::IMarkupExtension** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMarkupExtensionFactory = __uuidof(IMarkupExtensionFactory);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IMarkupExtensionOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.MarkupExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IMarkupExtensionOverrides[] = L"Windows.UI.Xaml.Markup.IMarkupExtensionOverrides";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("393779bf-b9c0-4ffb-a57f-58e7356e425f")
                    IMarkupExtensionOverrides : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ProvideValue(
                            IInspectable** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMarkupExtensionOverrides = __uuidof(IMarkupExtensionOverrides);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBinaryWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBinaryWriter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBinaryWriter[] = L"Windows.UI.Xaml.Markup.IXamlBinaryWriter";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("829d2ad3-620a-46f6-845d-436a05927100")
                    IXamlBinaryWriter : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXamlBinaryWriter = __uuidof(IXamlBinaryWriter);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBinaryWriter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBinaryWriterStatics[] = L"Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("0d8ed07a-9b82-4aa8-b68b-026f2de1cc86")
                    IXamlBinaryWriterStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Write(
                            __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* inputStreams,
                            __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* outputStreams,
                            ABI::Windows::UI::Xaml::Markup::IXamlMetadataProvider* xamlMetadataProvider,
                            ABI::Windows::UI::Xaml::Markup::XamlBinaryWriterErrorInformation* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlBinaryWriterStatics = __uuidof(IXamlBinaryWriterStatics);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBindScopeDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBindScopeDiagnostics[] = L"Windows.UI.Xaml.Markup.IXamlBindScopeDiagnostics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("f264a29d-bded-43aa-a5b0-26ac21a81eb8")
                    IXamlBindScopeDiagnostics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Disable(
                            INT32 lineNumber,
                            INT32 columnNumber
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlBindScopeDiagnostics = __uuidof(IXamlBindScopeDiagnostics);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBindingHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBindingHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBindingHelper[] = L"Windows.UI.Xaml.Markup.IXamlBindingHelper";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("faa6fb06-8ab9-4ef7-8ae7-fbd30bbfd06d")
                    IXamlBindingHelper : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXamlBindingHelper = __uuidof(IXamlBindingHelper);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBindingHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBindingHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBindingHelperStatics[] = L"Windows.UI.Xaml.Markup.IXamlBindingHelperStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("f65cfb71-c80c-4ffa-86ee-558754ee336d")
                    IXamlBindingHelperStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DataTemplateComponentProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDataTemplateComponent(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Markup::IDataTemplateComponent** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetDataTemplateComponent(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Markup::IDataTemplateComponent* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SuspendRendering(
                            ABI::Windows::UI::Xaml::IUIElement* target
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResumeRendering(
                            ABI::Windows::UI::Xaml::IUIElement* target
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConvertValue(
                            ABI::Windows::UI::Xaml::Interop::TypeName type,
                            IInspectable* value,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromString(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromBoolean(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromChar16(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            WCHAR value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromDateTime(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            ABI::Windows::Foundation::DateTime value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromDouble(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromInt32(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromUInt32(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromInt64(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            INT64 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromUInt64(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            UINT64 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromSingle(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromPoint(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            ABI::Windows::Foundation::Point value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromRect(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            ABI::Windows::Foundation::Rect value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromSize(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            ABI::Windows::Foundation::Size value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromTimeSpan(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromByte(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            BYTE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromUri(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyFromObject(
                            IInspectable* dependencyObject,
                            ABI::Windows::UI::Xaml::IDependencyProperty* propertyToSet,
                            IInspectable* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlBindingHelperStatics = __uuidof(IXamlBindingHelperStatics);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMarkupHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlMarkupHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMarkupHelper[] = L"Windows.UI.Xaml.Markup.IXamlMarkupHelper";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("d0e6673c-5342-44ef-85a7-ed327a739d9a")
                    IXamlMarkupHelper : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXamlMarkupHelper = __uuidof(IXamlMarkupHelper);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlMarkupHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMarkupHelperStatics[] = L"Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("c9bc3725-f34f-445c-81a2-6b72a5e8f072")
                    IXamlMarkupHelperStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE UnloadObject(
                            ABI::Windows::UI::Xaml::IDependencyObject* element
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlMarkupHelperStatics = __uuidof(IXamlMarkupHelperStatics);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMember
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMember[] = L"Windows.UI.Xaml.Markup.IXamlMember";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("c541f58c-43a9-4216-b718-e0b11b14e93e")
                    IXamlMember : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsAttachable(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDependencyProperty(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TargetType(
                            ABI::Windows::UI::Xaml::Markup::IXamlType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Type(
                            ABI::Windows::UI::Xaml::Markup::IXamlType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValue(
                            IInspectable* instance,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetValue(
                            IInspectable* instance,
                            IInspectable* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlMember = __uuidof(IXamlMember);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMetadataProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMetadataProvider[] = L"Windows.UI.Xaml.Markup.IXamlMetadataProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("b3765d69-68a5-4b32-8861-fdb90c1f5836")
                    IXamlMetadataProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetXamlType(
                            ABI::Windows::UI::Xaml::Interop::TypeName type,
                            ABI::Windows::UI::Xaml::Markup::IXamlType** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetXamlTypeByFullName(
                            HSTRING fullName,
                            ABI::Windows::UI::Xaml::Markup::IXamlType** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetXmlnsDefinitions(
                            UINT32* resultLength,
                            ABI::Windows::UI::Xaml::Markup::XmlnsDefinition** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlMetadataProvider = __uuidof(IXamlMetadataProvider);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlReader[] = L"Windows.UI.Xaml.Markup.IXamlReader";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("24374cf1-cceb-48bf-a514-41b0186f84c2")
                    IXamlReader : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXamlReader = __uuidof(IXamlReader);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlReaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlReaderStatics[] = L"Windows.UI.Xaml.Markup.IXamlReaderStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("9891c6bd-534f-4955-b85a-8a8dc0dca602")
                    IXamlReaderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Load(
                            HSTRING xaml,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadWithInitialTemplateValidation(
                            HSTRING xaml,
                            IInspectable** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlReaderStatics = __uuidof(IXamlReaderStatics);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlType[] = L"Windows.UI.Xaml.Markup.IXamlType";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("7920eab1-a2e5-479a-bd50-6cef3c0b4970")
                    IXamlType : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BaseType(
                            ABI::Windows::UI::Xaml::Markup::IXamlType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContentProperty(
                            ABI::Windows::UI::Xaml::Markup::IXamlMember** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FullName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsArray(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCollection(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsConstructible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDictionary(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsMarkupExtension(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsBindable(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ItemType(
                            ABI::Windows::UI::Xaml::Markup::IXamlType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyType(
                            ABI::Windows::UI::Xaml::Markup::IXamlType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UnderlyingType(
                            ABI::Windows::UI::Xaml::Interop::TypeName* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ActivateInstance(
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFromString(
                            HSTRING value,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetMember(
                            HSTRING name,
                            ABI::Windows::UI::Xaml::Markup::IXamlMember** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddToVector(
                            IInspectable* instance,
                            IInspectable* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddToMap(
                            IInspectable* instance,
                            IInspectable* key,
                            IInspectable* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RunInitializer(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlType = __uuidof(IXamlType);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlType2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Markup.IXamlType
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlType2[] = L"Windows.UI.Xaml.Markup.IXamlType2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Markup {
                    MIDL_INTERFACE("9f0c6e3b-433b-56ad-8f69-78a4dd3e64f9")
                    IXamlType2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BoxedType(
                            ABI::Windows::UI::Xaml::Markup::IXamlType** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXamlType2 = __uuidof(IXamlType2);
                } /* Markup */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Xaml.Markup.MarkupExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IMarkupExtension ** Default Interface **
 *    Windows.UI.Xaml.Markup.IMarkupExtensionOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_MarkupExtension_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_MarkupExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_MarkupExtension[] = L"Windows.UI.Xaml.Markup.MarkupExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlBinaryWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlBinaryWriter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBinaryWriter_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBinaryWriter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlBinaryWriter[] = L"Windows.UI.Xaml.Markup.XamlBinaryWriter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlBindingHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlBindingHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlBindingHelper ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBindingHelper_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBindingHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlBindingHelper[] = L"Windows.UI.Xaml.Markup.XamlBindingHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlMarkupHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlMarkupHelper ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlMarkupHelper_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlMarkupHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlMarkupHelper[] = L"Windows.UI.Xaml.Markup.XamlMarkupHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlReaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlReader_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlReader[] = L"Windows.UI.Xaml.Markup.XamlReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2 __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2 __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIIterator_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIVectorView_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** items);

    END_INTERFACE
} __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyObject __x_ABI_CWindows_CUI_CXaml_CIDependencyObject;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CXamlBinaryWriterErrorInformation __x_ABI_CWindows_CUI_CXaml_CMarkup_CXamlBinaryWriterErrorInformation;

typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CXmlnsDefinition __x_ABI_CWindows_CUI_CXaml_CMarkup_CXmlnsDefinition;

/*
 *
 * Struct Windows.UI.Xaml.Markup.XamlBinaryWriterErrorInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CXamlBinaryWriterErrorInformation
{
    UINT32 InputStreamIndex;
    UINT32 LineNumber;
    UINT32 LinePosition;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Markup.XmlnsDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CXmlnsDefinition
{
    HSTRING XmlNamespace;
    HSTRING Namespace;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IComponentConnector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IComponentConnector[] = L"Windows.UI.Xaml.Markup.IComponentConnector";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Connect)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector* This,
        INT32 connectionId,
        IInspectable* target);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnectorVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_Connect(This, connectionId, target) \
    ((This)->lpVtbl->Connect(This, connectionId, target))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IComponentConnector2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IComponentConnector2[] = L"Windows.UI.Xaml.Markup.IComponentConnector2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetBindingConnector)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2* This,
        INT32 connectionId,
        IInspectable* target,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_GetBindingConnector(This, connectionId, target, result) \
    ((This)->lpVtbl->GetBindingConnector(This, connectionId, target, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIComponentConnector2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IDataTemplateComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IDataTemplateComponent[] = L"Windows.UI.Xaml.Markup.IDataTemplateComponent";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Recycle)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This);
    HRESULT (STDMETHODCALLTYPE* ProcessBindings)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* This,
        IInspectable* item,
        INT32 itemIndex,
        INT32 phase,
        INT32* nextPhase);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponentVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_Recycle(This) \
    ((This)->lpVtbl->Recycle(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_ProcessBindings(This, item, itemIndex, phase, nextPhase) \
    ((This)->lpVtbl->ProcessBindings(This, item, itemIndex, phase, nextPhase))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IMarkupExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.MarkupExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IMarkupExtension[] = L"Windows.UI.Xaml.Markup.IMarkupExtension";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IMarkupExtensionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.MarkupExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IMarkupExtensionFactory[] = L"Windows.UI.Xaml.Markup.IMarkupExtensionFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtension** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IMarkupExtensionOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.MarkupExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IMarkupExtensionOverrides[] = L"Windows.UI.Xaml.Markup.IMarkupExtensionOverrides";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverridesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ProvideValue)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides* This,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverridesVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverridesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_ProvideValue(This, result) \
    ((This)->lpVtbl->ProvideValue(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIMarkupExtensionOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBinaryWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBinaryWriter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBinaryWriter[] = L"Windows.UI.Xaml.Markup.IXamlBinaryWriter";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBinaryWriter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBinaryWriterStatics[] = L"Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Write)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics* This,
        __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* inputStreams,
        __FIVector_1_Windows__CStorage__CStreams__CIRandomAccessStream* outputStreams,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* xamlMetadataProvider,
        struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CXamlBinaryWriterErrorInformation* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_Write(This, inputStreams, outputStreams, xamlMetadataProvider, result) \
    ((This)->lpVtbl->Write(This, inputStreams, outputStreams, xamlMetadataProvider, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBinaryWriterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBindScopeDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBindScopeDiagnostics[] = L"Windows.UI.Xaml.Markup.IXamlBindScopeDiagnostics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnosticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Disable)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics* This,
        INT32 lineNumber,
        INT32 columnNumber);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnosticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnosticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_Disable(This, lineNumber, columnNumber) \
    ((This)->lpVtbl->Disable(This, lineNumber, columnNumber))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindScopeDiagnostics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBindingHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBindingHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBindingHelper[] = L"Windows.UI.Xaml.Markup.IXamlBindingHelper";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlBindingHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlBindingHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlBindingHelperStatics[] = L"Windows.UI.Xaml.Markup.IXamlBindingHelperStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataTemplateComponentProperty)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetDataTemplateComponent)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent** result);
    HRESULT (STDMETHODCALLTYPE* SetDataTemplateComponent)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIDataTemplateComponent* value);
    HRESULT (STDMETHODCALLTYPE* SuspendRendering)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* target);
    HRESULT (STDMETHODCALLTYPE* ResumeRendering)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* target);
    HRESULT (STDMETHODCALLTYPE* ConvertValue)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName type,
        IInspectable* value,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromString)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromBoolean)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromChar16)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        WCHAR value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromDateTime)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromDouble)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromInt32)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromUInt32)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromInt64)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        INT64 value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromUInt64)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromSingle)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromPoint)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromRect)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromSize)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        struct __x_ABI_CWindows_CFoundation_CSize value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromTimeSpan)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromByte)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromUri)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyFromObject)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics* This,
        IInspectable* dependencyObject,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty* propertyToSet,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_get_DataTemplateComponentProperty(This, value) \
    ((This)->lpVtbl->get_DataTemplateComponentProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_GetDataTemplateComponent(This, element, result) \
    ((This)->lpVtbl->GetDataTemplateComponent(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetDataTemplateComponent(This, element, value) \
    ((This)->lpVtbl->SetDataTemplateComponent(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SuspendRendering(This, target) \
    ((This)->lpVtbl->SuspendRendering(This, target))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_ResumeRendering(This, target) \
    ((This)->lpVtbl->ResumeRendering(This, target))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_ConvertValue(This, type, value, result) \
    ((This)->lpVtbl->ConvertValue(This, type, value, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromString(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromString(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromBoolean(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromBoolean(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromChar16(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromChar16(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromDateTime(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromDateTime(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromDouble(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromDouble(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromInt32(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromInt32(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromUInt32(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromUInt32(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromInt64(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromInt64(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromUInt64(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromUInt64(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromSingle(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromSingle(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromPoint(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromPoint(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromRect(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromRect(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromSize(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromSize(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromTimeSpan(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromTimeSpan(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromByte(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromByte(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromUri(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromUri(This, dependencyObject, propertyToSet, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_SetPropertyFromObject(This, dependencyObject, propertyToSet, value) \
    ((This)->lpVtbl->SetPropertyFromObject(This, dependencyObject, propertyToSet, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlBindingHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMarkupHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlMarkupHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMarkupHelper[] = L"Windows.UI.Xaml.Markup.IXamlMarkupHelper";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlMarkupHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMarkupHelperStatics[] = L"Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UnloadObject)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_UnloadObject(This, element) \
    ((This)->lpVtbl->UnloadObject(This, element))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMarkupHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMember
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMember[] = L"Windows.UI.Xaml.Markup.IXamlMember";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMemberVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAttachable)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDependencyProperty)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TargetType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** value);
    HRESULT (STDMETHODCALLTYPE* GetValue)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        IInspectable* instance,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* SetValue)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember* This,
        IInspectable* instance,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMemberVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMemberVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_get_IsAttachable(This, value) \
    ((This)->lpVtbl->get_IsAttachable(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_get_IsDependencyProperty(This, value) \
    ((This)->lpVtbl->get_IsDependencyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_get_TargetType(This, value) \
    ((This)->lpVtbl->get_TargetType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_GetValue(This, instance, result) \
    ((This)->lpVtbl->GetValue(This, instance, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_SetValue(This, instance, value) \
    ((This)->lpVtbl->SetValue(This, instance, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlMetadataProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlMetadataProvider[] = L"Windows.UI.Xaml.Markup.IXamlMetadataProvider";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetXamlType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName type,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** result);
    HRESULT (STDMETHODCALLTYPE* GetXamlTypeByFullName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        HSTRING fullName,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** result);
    HRESULT (STDMETHODCALLTYPE* GetXmlnsDefinitions)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider* This,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CXmlnsDefinition** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProviderVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_GetXamlType(This, type, result) \
    ((This)->lpVtbl->GetXamlType(This, type, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_GetXamlTypeByFullName(This, fullName, result) \
    ((This)->lpVtbl->GetXamlTypeByFullName(This, fullName, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_GetXmlnsDefinitions(This, resultLength, result) \
    ((This)->lpVtbl->GetXmlnsDefinitions(This, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMetadataProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlReader[] = L"Windows.UI.Xaml.Markup.IXamlReader";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlReaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Markup.XamlReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlReaderStatics[] = L"Windows.UI.Xaml.Markup.IXamlReaderStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Load)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This,
        HSTRING xaml,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* LoadWithInitialTemplateValidation)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics* This,
        HSTRING xaml,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_Load(This, xaml, result) \
    ((This)->lpVtbl->Load(This, xaml, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_LoadWithInitialTemplateValidation(This, xaml, result) \
    ((This)->lpVtbl->LoadWithInitialTemplateValidation(This, xaml, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlReaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlType[] = L"Windows.UI.Xaml.Markup.IXamlType";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BaseType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** value);
    HRESULT (STDMETHODCALLTYPE* get_ContentProperty)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember** value);
    HRESULT (STDMETHODCALLTYPE* get_FullName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsArray)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCollection)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsConstructible)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDictionary)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMarkupExtension)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBindable)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ItemType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** value);
    HRESULT (STDMETHODCALLTYPE* get_KeyType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** value);
    HRESULT (STDMETHODCALLTYPE* get_UnderlyingType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName* value);
    HRESULT (STDMETHODCALLTYPE* ActivateInstance)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromString)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        HSTRING value,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* GetMember)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        HSTRING name,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlMember** result);
    HRESULT (STDMETHODCALLTYPE* AddToVector)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        IInspectable* instance,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* AddToMap)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This,
        IInspectable* instance,
        IInspectable* key,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* RunInitializer)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlTypeVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_BaseType(This, value) \
    ((This)->lpVtbl->get_BaseType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_ContentProperty(This, value) \
    ((This)->lpVtbl->get_ContentProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_FullName(This, value) \
    ((This)->lpVtbl->get_FullName(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_IsArray(This, value) \
    ((This)->lpVtbl->get_IsArray(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_IsCollection(This, value) \
    ((This)->lpVtbl->get_IsCollection(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_IsConstructible(This, value) \
    ((This)->lpVtbl->get_IsConstructible(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_IsDictionary(This, value) \
    ((This)->lpVtbl->get_IsDictionary(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_IsMarkupExtension(This, value) \
    ((This)->lpVtbl->get_IsMarkupExtension(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_IsBindable(This, value) \
    ((This)->lpVtbl->get_IsBindable(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_ItemType(This, value) \
    ((This)->lpVtbl->get_ItemType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_KeyType(This, value) \
    ((This)->lpVtbl->get_KeyType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_get_UnderlyingType(This, value) \
    ((This)->lpVtbl->get_UnderlyingType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_ActivateInstance(This, result) \
    ((This)->lpVtbl->ActivateInstance(This, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_CreateFromString(This, value, result) \
    ((This)->lpVtbl->CreateFromString(This, value, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_GetMember(This, name, result) \
    ((This)->lpVtbl->GetMember(This, name, result))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_AddToVector(This, instance, value) \
    ((This)->lpVtbl->AddToVector(This, instance, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_AddToMap(This, instance, key, value) \
    ((This)->lpVtbl->AddToMap(This, instance, key, value))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_RunInitializer(This) \
    ((This)->lpVtbl->RunInitializer(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Markup.IXamlType2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Markup.IXamlType
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Markup_IXamlType2[] = L"Windows.UI.Xaml.Markup.IXamlType2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BoxedType)(__x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2* This,
        __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_get_BoxedType(This, value) \
    ((This)->lpVtbl->get_BoxedType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMarkup_CIXamlType2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Xaml.Markup.MarkupExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IMarkupExtension ** Default Interface **
 *    Windows.UI.Xaml.Markup.IMarkupExtensionOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_MarkupExtension_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_MarkupExtension_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_MarkupExtension[] = L"Windows.UI.Xaml.Markup.MarkupExtension";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlBinaryWriter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlBinaryWriter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBinaryWriter_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBinaryWriter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlBinaryWriter[] = L"Windows.UI.Xaml.Markup.XamlBinaryWriter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlBindingHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlBindingHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlBindingHelper ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBindingHelper_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlBindingHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlBindingHelper[] = L"Windows.UI.Xaml.Markup.XamlBindingHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlMarkupHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlMarkupHelper ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlMarkupHelper_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlMarkupHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlMarkupHelper[] = L"Windows.UI.Xaml.Markup.XamlMarkupHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Markup.XamlReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Markup.IXamlReaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Markup.IXamlReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlReader_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Markup_XamlReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Markup_XamlReader[] = L"Windows.UI.Xaml.Markup.XamlReader";
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
#endif // __windows2Eui2Examl2Emarkup_p_h__

#endif // __windows2Eui2Examl2Emarkup_h__
