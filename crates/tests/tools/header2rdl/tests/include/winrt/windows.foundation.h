
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
#ifndef __windows2Efoundation_h__
#define __windows2Efoundation_h__
#ifndef __windows2Efoundation_p_h__
#define __windows2Efoundation_p_h__


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
#include "IVectorChangedEventArgs.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncActionCompletedHandler;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler ABI::Windows::Foundation::IAsyncActionCompletedHandler

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferralCompletedHandler;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler ABI::Windows::Foundation::IDeferralCompletedHandler

#endif // ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferralFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferralFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferralFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferralFactory ABI::Windows::Foundation::IDeferralFactory

#endif // ____x_ABI_CWindows_CFoundation_CIDeferralFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IGetActivationFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory ABI::Windows::Foundation::IGetActivationFactory

#endif // ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IGuidHelperStatics;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics ABI::Windows::Foundation::IGuidHelperStatics

#endif // ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IMemoryBuffer;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer ABI::Windows::Foundation::IMemoryBuffer

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IMemoryBufferFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory ABI::Windows::Foundation::IMemoryBufferFactory

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IMemoryBufferReference;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference ABI::Windows::Foundation::IMemoryBufferReference

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValueStatics;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics ABI::Windows::Foundation::IPropertyValueStatics

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IStringable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIStringable ABI::Windows::Foundation::IStringable

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriEscapeStatics;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics ABI::Windows::Foundation::IUriEscapeStatics

#endif // ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClassFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory ABI::Windows::Foundation::IUriRuntimeClassFactory

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClassWithAbsoluteCanonicalUri;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri ABI::Windows::Foundation::IUriRuntimeClassWithAbsoluteCanonicalUri

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IWwwFormUrlDecoderEntry;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry ABI::Windows::Foundation::IWwwFormUrlDecoderEntry

#endif // ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IWwwFormUrlDecoderRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass ABI::Windows::Foundation::IWwwFormUrlDecoderRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IWwwFormUrlDecoderRuntimeClassFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory ABI::Windows::Foundation::IWwwFormUrlDecoderRuntimeClassFactory

#endif // ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE
#define DEF___FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("32e54295-373c-50cb-80a1-468a990ca780"))
IIterator<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*> : IIterator_impl<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.IWwwFormUrlDecoderEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*> __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_t;
#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE
#define DEF___FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("876be83b-7218-5bfb-a169-83152ef7e146"))
IIterable<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*> : IIterable_impl<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.IWwwFormUrlDecoderEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*> __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_t;
#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapChangedEventArgs_1_HSTRING_USE
#define DEF___FIMapChangedEventArgs_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60141efb-f2f9-5377-96fd-f8c60d9558b5"))
IMapChangedEventArgs<HSTRING> : IMapChangedEventArgs_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapChangedEventArgs`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapChangedEventArgs<HSTRING> __FIMapChangedEventArgs_1_HSTRING_t;
#define __FIMapChangedEventArgs_1_HSTRING ABI::Windows::Foundation::Collections::__FIMapChangedEventArgs_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapChangedEventArgs_1_HSTRING_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b0d3570-0877-5ec2-8a2c-3b9539506aca"))
IMap<HSTRING, IInspectable*> : IMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, IInspectable*> __FIMap_2_HSTRING_IInspectable_t;
#define __FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */



#ifndef DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE
#define DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("24f981e5-ddca-538d-aada-a59906084cf1"))
MapChangedEventHandler<HSTRING, IInspectable*> : MapChangedEventHandler_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.MapChangedEventHandler`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef MapChangedEventHandler<HSTRING, IInspectable*> __FMapChangedEventHandler_2_HSTRING_IInspectable_t;
#define __FMapChangedEventHandler_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FMapChangedEventHandler_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIObservableMap_2_HSTRING_IInspectable_USE
#define DEF___FIObservableMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("236aac9d-fb12-5c4d-a41c-9e445fb4d7ec"))
IObservableMap<HSTRING, IInspectable*> : IObservableMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableMap<HSTRING, IInspectable*> __FIObservableMap_2_HSTRING_IInspectable_t;
#define __FIObservableMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIObservableMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableMap_2_HSTRING_IInspectable_USE */



#ifndef DEF___FMapChangedEventHandler_2_HSTRING_HSTRING_USE
#define DEF___FMapChangedEventHandler_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2663f37-2e1b-500c-ad68-c3ed7a8f74c8"))
MapChangedEventHandler<HSTRING, HSTRING> : MapChangedEventHandler_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.MapChangedEventHandler`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef MapChangedEventHandler<HSTRING, HSTRING> __FMapChangedEventHandler_2_HSTRING_HSTRING_t;
#define __FMapChangedEventHandler_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FMapChangedEventHandler_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FMapChangedEventHandler_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIObservableMap_2_HSTRING_HSTRING_USE
#define DEF___FIObservableMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1e036276-2f60-55f6-b7f3-f86079e6900b"))
IObservableMap<HSTRING, HSTRING> : IObservableMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableMap<HSTRING, HSTRING> __FIObservableMap_2_HSTRING_HSTRING_t;
#define __FIObservableMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIObservableMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableMap_2_HSTRING_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b1f00d3b-1f06-5117-93ea-2a0d79116701"))
IVectorView<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*> : IVectorView_impl<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.IWwwFormUrlDecoderEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Foundation::IWwwFormUrlDecoderEntry*> __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_t;
#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f4637d4a-0760-5431-bfc0-24eb1d4f6c4f"))
ITypedEventHandler<ABI::Windows::Foundation::IMemoryBufferReference*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::IMemoryBufferReference*, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Foundation.IMemoryBufferReference, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Foundation::IMemoryBufferReference*, IInspectable*> __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                typedef enum CollectionChange : int CollectionChange;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef enum PropertyType : int PropertyType;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class MemoryBuffer;
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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class WwwFormUrlDecoder;
        } /* Foundation */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Foundation.PropertyType
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            enum PropertyType : int
            {
                PropertyType_Empty = 0,
                PropertyType_UInt8 = 1,
                PropertyType_Int16 = 2,
                PropertyType_UInt16 = 3,
                PropertyType_Int32 = 4,
                PropertyType_UInt32 = 5,
                PropertyType_Int64 = 6,
                PropertyType_UInt64 = 7,
                PropertyType_Single = 8,
                PropertyType_Double = 9,
                PropertyType_Char16 = 10,
                PropertyType_Boolean = 11,
                PropertyType_String = 12,
                PropertyType_Inspectable = 13,
                PropertyType_DateTime = 14,
                PropertyType_TimeSpan = 15,
                PropertyType_Guid = 16,
                PropertyType_Point = 17,
                PropertyType_Size = 18,
                PropertyType_Rect = 19,
                PropertyType_OtherType = 20,
                PropertyType_UInt8Array = 1025,
                PropertyType_Int16Array = 1026,
                PropertyType_UInt16Array = 1027,
                PropertyType_Int32Array = 1028,
                PropertyType_UInt32Array = 1029,
                PropertyType_Int64Array = 1030,
                PropertyType_UInt64Array = 1031,
                PropertyType_SingleArray = 1032,
                PropertyType_DoubleArray = 1033,
                PropertyType_Char16Array = 1034,
                PropertyType_BooleanArray = 1035,
                PropertyType_StringArray = 1036,
                PropertyType_InspectableArray = 1037,
                PropertyType_DateTimeArray = 1038,
                PropertyType_TimeSpanArray = 1039,
                PropertyType_GuidArray = 1040,
                PropertyType_PointArray = 1041,
                PropertyType_SizeArray = 1042,
                PropertyType_RectArray = 1043,
                PropertyType_OtherTypeArray = 1044,
            };
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.DateTime
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            struct DateTime
            {
                INT64 UniversalTime;
            };
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Point
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            struct Point
            {
                FLOAT X;
                FLOAT Y;
            };
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Rect
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            struct Rect
            {
                FLOAT X;
                FLOAT Y;
                FLOAT Width;
                FLOAT Height;
            };
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Size
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            struct Size
            {
                FLOAT Width;
                FLOAT Height;
            };
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.TimeSpan
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            struct TimeSpan
            {
                INT64 Duration;
            };
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Foundation.AsyncActionCompletedHandler
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7")
            IAsyncActionCompletedHandler : public IUnknown
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Invoke(
                    ABI::Windows::Foundation::IAsyncAction* asyncInfo,
                    AsyncStatus asyncStatus
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAsyncActionCompletedHandler = __uuidof(IAsyncActionCompletedHandler);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Foundation.DeferralCompletedHandler
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("ed32a372-f3c8-4faa-9cfb-470148da3888")
            IDeferralCompletedHandler : public IUnknown
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Invoke(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IDeferralCompletedHandler = __uuidof(IDeferralCompletedHandler);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Collections.IPropertySet
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *     Windows.Foundation.Collections.IMap`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Collections_IPropertySet[] = L"Windows.Foundation.Collections.IPropertySet";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                MIDL_INTERFACE("8a43ed9f-f4e6-4421-acf9-1dab2986820c")
                IPropertySet : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPropertySet = __uuidof(IPropertySet);
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IAsyncAction
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIAsyncAction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IAsyncAction[] = L"Windows.Foundation.IAsyncAction";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("5a648006-843a-4da9-865b-9d26e5dfad7b")
            IAsyncAction : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE put_Completed(
                    ABI::Windows::Foundation::IAsyncActionCompletedHandler* handler
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Completed(
                    ABI::Windows::Foundation::IAsyncActionCompletedHandler** handler
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetResults(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IAsyncAction = __uuidof(IAsyncAction);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIAsyncAction;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIAsyncAction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IClosable
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIClosable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIClosable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IClosable[] = L"Windows.Foundation.IClosable";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("30d5a829-7fa4-4026-83bb-d75bae4ea99e")
            IClosable : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Close(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IClosable = __uuidof(IClosable);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIClosable;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIClosable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IDeferral
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Deferral
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IDeferral[] = L"Windows.Foundation.IDeferral";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("d6269732-3b7f-46a7-b40b-4fdca2a2c693")
            IDeferral : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IDeferral = __uuidof(IDeferral);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDeferral;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IDeferralFactory
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Deferral
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDeferralFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDeferralFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IDeferralFactory[] = L"Windows.Foundation.IDeferralFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("65a1ecc5-3fb5-4832-8ca9-f061b281d13a")
            IDeferralFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Create(
                    ABI::Windows::Foundation::IDeferralCompletedHandler* handler,
                    ABI::Windows::Foundation::IDeferral** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IDeferralFactory = __uuidof(IDeferralFactory);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDeferralFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDeferralFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IGetActivationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIGetActivationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IGetActivationFactory[] = L"Windows.Foundation.IGetActivationFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("4edb8ee2-96dd-49a7-94f7-4607ddab8e3c")
            IGetActivationFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetActivationFactory(
                    HSTRING activatableClassId,
                    IInspectable** factory
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IGetActivationFactory = __uuidof(IGetActivationFactory);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIGetActivationFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIGetActivationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IGuidHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.GuidHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IGuidHelperStatics[] = L"Windows.Foundation.IGuidHelperStatics";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("59c7966b-ae52-5283-ad7f-a1b9e9678add")
            IGuidHelperStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateNewGuid(
                    GUID* result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Empty(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Equals(
                    const GUID* target,
                    const GUID* value,
                    boolean* result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IGuidHelperStatics = __uuidof(IGuidHelperStatics);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIGuidHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Foundation.IMemoryBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIMemoryBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IMemoryBuffer[] = L"Windows.Foundation.IMemoryBuffer";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("fbc4dd2a-245b-11e4-af98-689423260cf8")
            IMemoryBuffer : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateReference(
                    ABI::Windows::Foundation::IMemoryBufferReference** reference
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMemoryBuffer = __uuidof(IMemoryBuffer);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIMemoryBuffer;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIMemoryBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IMemoryBufferFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.MemoryBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IMemoryBufferFactory[] = L"Windows.Foundation.IMemoryBufferFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("fbc4dd2b-245b-11e4-af98-689423260cf8")
            IMemoryBufferFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Create(
                    UINT32 capacity,
                    ABI::Windows::Foundation::IMemoryBuffer** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMemoryBufferFactory = __uuidof(IMemoryBufferFactory);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIMemoryBufferFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IMemoryBufferReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IMemoryBufferReference[] = L"Windows.Foundation.IMemoryBufferReference";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("fbc4dd29-245b-11e4-af98-689423260cf8")
            IMemoryBufferReference : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Capacity(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_Closed(
                    __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                    EventRegistrationToken cookie
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMemoryBufferReference = __uuidof(IMemoryBufferReference);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIMemoryBufferReference;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IPropertyValue
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIPropertyValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IPropertyValue[] = L"Windows.Foundation.IPropertyValue";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("4bd682dd-7554-40e9-9a9b-82654ede7e62")
            IPropertyValue : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Type(
                    ABI::Windows::Foundation::PropertyType* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsNumericScalar(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt8(
                    BYTE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInt16(
                    INT16* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt16(
                    UINT16* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInt32(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt32(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInt64(
                    INT64* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt64(
                    UINT64* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetSingle(
                    FLOAT* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDouble(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetChar16(
                    WCHAR* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetBoolean(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetString(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetGuid(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDateTime(
                    ABI::Windows::Foundation::DateTime* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetTimeSpan(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetPoint(
                    ABI::Windows::Foundation::Point* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetSize(
                    ABI::Windows::Foundation::Size* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetRect(
                    ABI::Windows::Foundation::Rect* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt8Array(
                    UINT32* valueLength,
                    BYTE** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInt16Array(
                    UINT32* valueLength,
                    INT16** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt16Array(
                    UINT32* valueLength,
                    UINT16** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInt32Array(
                    UINT32* valueLength,
                    INT32** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt32Array(
                    UINT32* valueLength,
                    UINT32** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInt64Array(
                    UINT32* valueLength,
                    INT64** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetUInt64Array(
                    UINT32* valueLength,
                    UINT64** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetSingleArray(
                    UINT32* valueLength,
                    FLOAT** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDoubleArray(
                    UINT32* valueLength,
                    DOUBLE** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetChar16Array(
                    UINT32* valueLength,
                    WCHAR** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetBooleanArray(
                    UINT32* valueLength,
                    boolean** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetStringArray(
                    UINT32* valueLength,
                    HSTRING** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInspectableArray(
                    UINT32* valueLength,
                    IInspectable*** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetGuidArray(
                    UINT32* valueLength,
                    GUID** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDateTimeArray(
                    UINT32* valueLength,
                    ABI::Windows::Foundation::DateTime** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetTimeSpanArray(
                    UINT32* valueLength,
                    ABI::Windows::Foundation::TimeSpan** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetPointArray(
                    UINT32* valueLength,
                    ABI::Windows::Foundation::Point** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetSizeArray(
                    UINT32* valueLength,
                    ABI::Windows::Foundation::Size** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetRectArray(
                    UINT32* valueLength,
                    ABI::Windows::Foundation::Rect** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPropertyValue = __uuidof(IPropertyValue);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIPropertyValue;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIPropertyValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IPropertyValueStatics
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.PropertyValue
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IPropertyValueStatics[] = L"Windows.Foundation.IPropertyValueStatics";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("629bdbc8-d932-4ff4-96b9-8d96c5c1e858")
            IPropertyValueStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateEmpty(
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt8(
                    BYTE value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInt16(
                    INT16 value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt16(
                    UINT16 value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInt32(
                    INT32 value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt32(
                    UINT32 value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInt64(
                    INT64 value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt64(
                    UINT64 value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateSingle(
                    FLOAT value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateDouble(
                    DOUBLE value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateChar16(
                    WCHAR value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateBoolean(
                    boolean value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateString(
                    HSTRING value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInspectable(
                    IInspectable* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateGuid(
                    GUID value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateDateTime(
                    ABI::Windows::Foundation::DateTime value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateTimeSpan(
                    ABI::Windows::Foundation::TimeSpan value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreatePoint(
                    ABI::Windows::Foundation::Point value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateSize(
                    ABI::Windows::Foundation::Size value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateRect(
                    ABI::Windows::Foundation::Rect value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt8Array(
                    UINT32 valueLength,
                    BYTE* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInt16Array(
                    UINT32 valueLength,
                    INT16* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt16Array(
                    UINT32 valueLength,
                    UINT16* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInt32Array(
                    UINT32 valueLength,
                    INT32* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt32Array(
                    UINT32 valueLength,
                    UINT32* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInt64Array(
                    UINT32 valueLength,
                    INT64* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateUInt64Array(
                    UINT32 valueLength,
                    UINT64* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateSingleArray(
                    UINT32 valueLength,
                    FLOAT* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateDoubleArray(
                    UINT32 valueLength,
                    DOUBLE* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateChar16Array(
                    UINT32 valueLength,
                    WCHAR* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateBooleanArray(
                    UINT32 valueLength,
                    boolean* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateStringArray(
                    UINT32 valueLength,
                    HSTRING* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateInspectableArray(
                    UINT32 valueLength,
                    IInspectable** value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateGuidArray(
                    UINT32 valueLength,
                    GUID* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateDateTimeArray(
                    UINT32 valueLength,
                    ABI::Windows::Foundation::DateTime* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateTimeSpanArray(
                    UINT32 valueLength,
                    ABI::Windows::Foundation::TimeSpan* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreatePointArray(
                    UINT32 valueLength,
                    ABI::Windows::Foundation::Point* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateSizeArray(
                    UINT32 valueLength,
                    ABI::Windows::Foundation::Size* value,
                    IInspectable** propertyValue
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateRectArray(
                    UINT32 valueLength,
                    ABI::Windows::Foundation::Rect* value,
                    IInspectable** propertyValue
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPropertyValueStatics = __uuidof(IPropertyValueStatics);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIPropertyValueStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IStringable
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIStringable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIStringable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IStringable[] = L"Windows.Foundation.IStringable";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("96369f54-8eb6-48f0-abce-c1b211e627c3")
            IStringable : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE ToString(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStringable = __uuidof(IStringable);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIStringable;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIStringable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriEscapeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriEscapeStatics[] = L"Windows.Foundation.IUriEscapeStatics";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("c1d432ba-c824-4452-a7fd-512bc3bbe9a1")
            IUriEscapeStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE UnescapeComponent(
                    HSTRING toUnescape,
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE EscapeComponent(
                    HSTRING toEscape,
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUriEscapeStatics = __uuidof(IUriEscapeStatics);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriEscapeStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriRuntimeClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriRuntimeClass[] = L"Windows.Foundation.IUriRuntimeClass";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("9e365e57-48b2-4160-956f-c7385120bbfc")
            IUriRuntimeClass : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AbsoluteUri(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayUri(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Domain(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Extension(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Fragment(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Host(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Password(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Path(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Query(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_QueryParsed(
                    ABI::Windows::Foundation::IWwwFormUrlDecoderRuntimeClass** ppWwwFormUrlDecoder
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RawUri(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SchemeName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UserName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Port(
                    INT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Suspicious(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Equals(
                    ABI::Windows::Foundation::IUriRuntimeClass* pUri,
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CombineUri(
                    HSTRING relativeUri,
                    ABI::Windows::Foundation::IUriRuntimeClass** instance
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUriRuntimeClass = __uuidof(IUriRuntimeClass);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriRuntimeClass;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriRuntimeClassFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriRuntimeClassFactory[] = L"Windows.Foundation.IUriRuntimeClassFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("44a9796f-723e-4fdf-a218-033e75b0c084")
            IUriRuntimeClassFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateUri(
                    HSTRING uri,
                    ABI::Windows::Foundation::IUriRuntimeClass** instance
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateWithRelativeUri(
                    HSTRING baseUri,
                    HSTRING relativeUri,
                    ABI::Windows::Foundation::IUriRuntimeClass** instance
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUriRuntimeClassFactory = __uuidof(IUriRuntimeClassFactory);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriRuntimeClassWithAbsoluteCanonicalUri[] = L"Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("758d9661-221c-480f-a339-50656673f46f")
            IUriRuntimeClassWithAbsoluteCanonicalUri : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AbsoluteCanonicalUri(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayIri(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUriRuntimeClassWithAbsoluteCanonicalUri = __uuidof(IUriRuntimeClassWithAbsoluteCanonicalUri);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IWwwFormUrlDecoderEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IWwwFormUrlDecoderEntry[] = L"Windows.Foundation.IWwwFormUrlDecoderEntry";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("125e7431-f678-4e8e-b670-20a9b06c512d")
            IWwwFormUrlDecoderEntry : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Name(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Value(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IWwwFormUrlDecoderEntry = __uuidof(IWwwFormUrlDecoderEntry);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IWwwFormUrlDecoderRuntimeClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.WwwFormUrlDecoder
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IWwwFormUrlDecoderRuntimeClass[] = L"Windows.Foundation.IWwwFormUrlDecoderRuntimeClass";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("d45a0451-f225-4542-9296-0e1df5d254df")
            IWwwFormUrlDecoderRuntimeClass : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetFirstValueByName(
                    HSTRING name,
                    HSTRING* phstrValue
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IWwwFormUrlDecoderRuntimeClass = __uuidof(IWwwFormUrlDecoderRuntimeClass);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.WwwFormUrlDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IWwwFormUrlDecoderRuntimeClassFactory[] = L"Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            MIDL_INTERFACE("5b8c6b3d-24ae-41b5-a1bf-f0c3d544845b")
            IWwwFormUrlDecoderRuntimeClassFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateWwwFormUrlDecoder(
                    HSTRING query,
                    ABI::Windows::Foundation::IWwwFormUrlDecoderRuntimeClass** instance
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IWwwFormUrlDecoderRuntimeClassFactory = __uuidof(IWwwFormUrlDecoderRuntimeClassFactory);
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Collections.PropertySet
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Collections_PropertySet_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Collections_PropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Collections_PropertySet[] = L"Windows.Foundation.Collections.PropertySet";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Collections.StringMap
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<String, String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>
 *    Windows.Foundation.Collections.IObservableMap`2<String, String>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Collections_StringMap_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Collections_StringMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Collections_StringMap[] = L"Windows.Foundation.Collections.StringMap";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Collections.ValueSet
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Collections_ValueSet_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Collections_ValueSet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Collections_ValueSet[] = L"Windows.Foundation.Collections.ValueSet";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Deferral
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IDeferralFactory interface starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IDeferral ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Deferral_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Deferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Deferral[] = L"Windows.Foundation.Deferral";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.GuidHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.IGuidHelperStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Foundation_GuidHelper_DEFINED
#define RUNTIMECLASS_Windows_Foundation_GuidHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_GuidHelper[] = L"Windows.Foundation.GuidHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Foundation.MemoryBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IMemoryBufferFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IMemoryBuffer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_MemoryBuffer_DEFINED
#define RUNTIMECLASS_Windows_Foundation_MemoryBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_MemoryBuffer[] = L"Windows.Foundation.MemoryBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.PropertyValue
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.IPropertyValueStatics interface starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_PropertyValue_DEFINED
#define RUNTIMECLASS_Windows_Foundation_PropertyValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_PropertyValue[] = L"Windows.Foundation.PropertyValue";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Uri
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IUriRuntimeClassFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.IUriEscapeStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IUriRuntimeClass ** Default Interface **
 *    Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Uri_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Uri_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Uri[] = L"Windows.Foundation.Uri";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.WwwFormUrlDecoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IWwwFormUrlDecoderRuntimeClass ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoder_DEFINED
#define RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_WwwFormUrlDecoder[] = L"Windows.Foundation.WwwFormUrlDecoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.WwwFormUrlDecoderEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IWwwFormUrlDecoderEntry ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoderEntry_DEFINED
#define RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoderEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_WwwFormUrlDecoderEntry[] = L"Windows.Foundation.WwwFormUrlDecoderEntry";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferralFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferralFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferralFactory __x_ABI_CWindows_CFoundation_CIDeferralFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferralFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIGetActivationFactory __x_ABI_CWindows_CFoundation_CIGetActivationFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIGuidHelperStatics __x_ABI_CWindows_CFoundation_CIGuidHelperStatics;

#endif // ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBufferReference __x_ABI_CWindows_CFoundation_CIMemoryBufferReference;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValueStatics __x_ABI_CWindows_CFoundation_CIPropertyValueStatics;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIStringable __x_ABI_CWindows_CFoundation_CIStringable;

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriEscapeStatics __x_ABI_CWindows_CFoundation_CIUriEscapeStatics;

#endif // ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry;

#endif // ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry;

typedef struct __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl;

interface __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry;

typedef struct __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        __FIIterator_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl;

interface __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapChangedEventArgs_1_HSTRING __FIMapChangedEventArgs_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapChangedEventArgs_1_HSTRING;

typedef struct __FIMapChangedEventArgs_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapChangedEventArgs_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapChangedEventArgs_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapChangedEventArgs_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapChangedEventArgs_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapChangedEventArgs_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CollectionChange)(__FIMapChangedEventArgs_1_HSTRING* This,
        enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange* result);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIMapChangedEventArgs_1_HSTRINGVtbl;

interface __FIMapChangedEventArgs_1_HSTRING
{
    CONST_VTBL struct __FIMapChangedEventArgs_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapChangedEventArgs_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapChangedEventArgs_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapChangedEventArgs_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapChangedEventArgs_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapChangedEventArgs_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapChangedEventArgs_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapChangedEventArgs_1_HSTRING_get_CollectionChange(This, result) \
    ((This)->lpVtbl->get_CollectionChange(This, result))

#define __FIMapChangedEventArgs_1_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#endif /* COBJMACROS */

#endif // ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_IInspectable __FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_IInspectable;

typedef struct __FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_IInspectable* This);

    END_INTERFACE
} __FIMap_2_HSTRING_IInspectableVtbl;

interface __FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_IInspectable __FIObservableMap_2_HSTRING_IInspectable;

#if !defined(____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FMapChangedEventHandler_2_HSTRING_IInspectable __FMapChangedEventHandler_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FMapChangedEventHandler_2_HSTRING_IInspectable;

typedef struct __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This,
        __FIObservableMap_2_HSTRING_IInspectable* sender,
        __FIMapChangedEventArgs_1_HSTRING* event);

    END_INTERFACE
} __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl;

interface __FMapChangedEventHandler_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_IInspectable __FIObservableMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableMap_2_HSTRING_IInspectable;

typedef struct __FIObservableMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MapChanged)(__FIObservableMap_2_HSTRING_IInspectable* This,
        __FMapChangedEventHandler_2_HSTRING_IInspectable* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_MapChanged)(__FIObservableMap_2_HSTRING_IInspectable* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableMap_2_HSTRING_IInspectableVtbl;

interface __FIObservableMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIObservableMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableMap_2_HSTRING_IInspectable_add_MapChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_MapChanged(This, vhnd, result))

#define __FIObservableMap_2_HSTRING_IInspectable_remove_MapChanged(This, token) \
    ((This)->lpVtbl->remove_MapChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_HSTRING __FIObservableMap_2_HSTRING_HSTRING;

#if !defined(____FMapChangedEventHandler_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FMapChangedEventHandler_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FMapChangedEventHandler_2_HSTRING_HSTRING __FMapChangedEventHandler_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FMapChangedEventHandler_2_HSTRING_HSTRING;

typedef struct __FMapChangedEventHandler_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This,
        __FIObservableMap_2_HSTRING_HSTRING* sender,
        __FIMapChangedEventArgs_1_HSTRING* event);

    END_INTERFACE
} __FMapChangedEventHandler_2_HSTRING_HSTRINGVtbl;

interface __FMapChangedEventHandler_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FMapChangedEventHandler_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FMapChangedEventHandler_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIObservableMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIObservableMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_HSTRING __FIObservableMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableMap_2_HSTRING_HSTRING;

typedef struct __FIObservableMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MapChanged)(__FIObservableMap_2_HSTRING_HSTRING* This,
        __FMapChangedEventHandler_2_HSTRING_HSTRING* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_MapChanged)(__FIObservableMap_2_HSTRING_HSTRING* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableMap_2_HSTRING_HSTRINGVtbl;

interface __FIObservableMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIObservableMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableMap_2_HSTRING_HSTRING_add_MapChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_MapChanged(This, vhnd, result))

#define __FIObservableMap_2_HSTRING_HSTRING_remove_MapChanged(This, token) \
    ((This)->lpVtbl->remove_MapChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry;

typedef struct __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl;

interface __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable* This,
        __x_ABI_CWindows_CFoundation_CIMemoryBufferReference* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange;

typedef enum __x_ABI_CWindows_CFoundation_CPropertyType __x_ABI_CWindows_CFoundation_CPropertyType;

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

/*
 *
 * Struct Windows.Foundation.PropertyType
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CPropertyType
{
    PropertyType_Empty = 0,
    PropertyType_UInt8 = 1,
    PropertyType_Int16 = 2,
    PropertyType_UInt16 = 3,
    PropertyType_Int32 = 4,
    PropertyType_UInt32 = 5,
    PropertyType_Int64 = 6,
    PropertyType_UInt64 = 7,
    PropertyType_Single = 8,
    PropertyType_Double = 9,
    PropertyType_Char16 = 10,
    PropertyType_Boolean = 11,
    PropertyType_String = 12,
    PropertyType_Inspectable = 13,
    PropertyType_DateTime = 14,
    PropertyType_TimeSpan = 15,
    PropertyType_Guid = 16,
    PropertyType_Point = 17,
    PropertyType_Size = 18,
    PropertyType_Rect = 19,
    PropertyType_OtherType = 20,
    PropertyType_UInt8Array = 1025,
    PropertyType_Int16Array = 1026,
    PropertyType_UInt16Array = 1027,
    PropertyType_Int32Array = 1028,
    PropertyType_UInt32Array = 1029,
    PropertyType_Int64Array = 1030,
    PropertyType_UInt64Array = 1031,
    PropertyType_SingleArray = 1032,
    PropertyType_DoubleArray = 1033,
    PropertyType_Char16Array = 1034,
    PropertyType_BooleanArray = 1035,
    PropertyType_StringArray = 1036,
    PropertyType_InspectableArray = 1037,
    PropertyType_DateTimeArray = 1038,
    PropertyType_TimeSpanArray = 1039,
    PropertyType_GuidArray = 1040,
    PropertyType_PointArray = 1041,
    PropertyType_SizeArray = 1042,
    PropertyType_RectArray = 1043,
    PropertyType_OtherTypeArray = 1044,
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.DateTime
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CFoundation_CDateTime
{
    INT64 UniversalTime;
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Point
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CFoundation_CPoint
{
    FLOAT X;
    FLOAT Y;
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Rect
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CFoundation_CRect
{
    FLOAT X;
    FLOAT Y;
    FLOAT Width;
    FLOAT Height;
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Size
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CFoundation_CSize
{
    FLOAT Width;
    FLOAT Height;
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.TimeSpan
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CFoundation_CTimeSpan
{
    INT64 Duration;
};
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Foundation.AsyncActionCompletedHandler
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandlerVtbl;

interface __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Foundation.DeferralCompletedHandler
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler* This);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandlerVtbl;

interface __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_Invoke(This) \
    ((This)->lpVtbl->Invoke(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Collections.IPropertySet
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *     Windows.Foundation.Collections.IMap`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Collections_IPropertySet[] = L"Windows.Foundation.Collections.IPropertySet";
typedef struct __x_ABI_CWindows_CFoundation_CCollections_CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CCollections_CIPropertySetVtbl;

interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CCollections_CIPropertySetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IAsyncAction
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIAsyncAction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IAsyncAction[] = L"Windows.Foundation.IAsyncAction";
typedef struct __x_ABI_CWindows_CFoundation_CIAsyncActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This,
        __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This,
        __x_ABI_CWindows_CFoundation_CIAsyncActionCompletedHandler** handler);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__x_ABI_CWindows_CFoundation_CIAsyncAction* This);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIAsyncActionVtbl;

interface __x_ABI_CWindows_CFoundation_CIAsyncAction
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIAsyncActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_get_Completed(This, handler) \
    ((This)->lpVtbl->get_Completed(This, handler))

#define __x_ABI_CWindows_CFoundation_CIAsyncAction_GetResults(This) \
    ((This)->lpVtbl->GetResults(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIAsyncAction;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIAsyncAction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IClosable
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIClosable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIClosable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IClosable[] = L"Windows.Foundation.IClosable";
typedef struct __x_ABI_CWindows_CFoundation_CIClosableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIClosable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIClosable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIClosable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIClosable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIClosable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIClosable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CFoundation_CIClosable* This);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIClosableVtbl;

interface __x_ABI_CWindows_CFoundation_CIClosable
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIClosableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIClosable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIClosable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIClosable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIClosable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIClosable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIClosable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIClosable_Close(This) \
    ((This)->lpVtbl->Close(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIClosable;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIClosable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IDeferral
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Deferral
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IDeferral[] = L"Windows.Foundation.IDeferral";
typedef struct __x_ABI_CWindows_CFoundation_CIDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CFoundation_CIDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIDeferralVtbl;

interface __x_ABI_CWindows_CFoundation_CIDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDeferral;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IDeferralFactory
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Deferral
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDeferralFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDeferralFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IDeferralFactory[] = L"Windows.Foundation.IDeferralFactory";
typedef struct __x_ABI_CWindows_CFoundation_CIDeferralFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CFoundation_CIDeferralFactory* This,
        __x_ABI_CWindows_CFoundation_CIDeferralCompletedHandler* handler,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIDeferralFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CIDeferralFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIDeferralFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIDeferralFactory_Create(This, handler, result) \
    ((This)->lpVtbl->Create(This, handler, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDeferralFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDeferralFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IGetActivationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIGetActivationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IGetActivationFactory[] = L"Windows.Foundation.IGetActivationFactory";
typedef struct __x_ABI_CWindows_CFoundation_CIGetActivationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetActivationFactory)(__x_ABI_CWindows_CFoundation_CIGetActivationFactory* This,
        HSTRING activatableClassId,
        IInspectable** factory);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIGetActivationFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CIGetActivationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIGetActivationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory_GetActivationFactory(This, activatableClassId, factory) \
    ((This)->lpVtbl->GetActivationFactory(This, activatableClassId, factory))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIGetActivationFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIGetActivationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IGuidHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.GuidHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IGuidHelperStatics[] = L"Windows.Foundation.IGuidHelperStatics";
typedef struct __x_ABI_CWindows_CFoundation_CIGuidHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateNewGuid)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_Empty)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* Equals)(__x_ABI_CWindows_CFoundation_CIGuidHelperStatics* This,
        const GUID* target,
        const GUID* value,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIGuidHelperStaticsVtbl;

interface __x_ABI_CWindows_CFoundation_CIGuidHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIGuidHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_CreateNewGuid(This, result) \
    ((This)->lpVtbl->CreateNewGuid(This, result))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_get_Empty(This, value) \
    ((This)->lpVtbl->get_Empty(This, value))

#define __x_ABI_CWindows_CFoundation_CIGuidHelperStatics_Equals(This, target, value, result) \
    ((This)->lpVtbl->Equals(This, target, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIGuidHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIGuidHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Foundation.IMemoryBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIMemoryBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IMemoryBuffer[] = L"Windows.Foundation.IMemoryBuffer";
typedef struct __x_ABI_CWindows_CFoundation_CIMemoryBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateReference)(__x_ABI_CWindows_CFoundation_CIMemoryBuffer* This,
        __x_ABI_CWindows_CFoundation_CIMemoryBufferReference** reference);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIMemoryBufferVtbl;

interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIMemoryBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer_CreateReference(This, reference) \
    ((This)->lpVtbl->CreateReference(This, reference))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIMemoryBuffer;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIMemoryBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IMemoryBufferFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.MemoryBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IMemoryBufferFactory[] = L"Windows.Foundation.IMemoryBufferFactory";
typedef struct __x_ABI_CWindows_CFoundation_CIMemoryBufferFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CFoundation_CIMemoryBufferFactory* This,
        UINT32 capacity,
        __x_ABI_CWindows_CFoundation_CIMemoryBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIMemoryBufferFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIMemoryBufferFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_Create(This, capacity, value) \
    ((This)->lpVtbl->Create(This, capacity, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIMemoryBufferFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IMemoryBufferReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IMemoryBufferReference[] = L"Windows.Foundation.IMemoryBufferReference";
typedef struct __x_ABI_CWindows_CFoundation_CIMemoryBufferReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capacity)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        __FITypedEventHandler_2_Windows__CFoundation__CIMemoryBufferReference_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CFoundation_CIMemoryBufferReference* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIMemoryBufferReferenceVtbl;

interface __x_ABI_CWindows_CFoundation_CIMemoryBufferReference
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIMemoryBufferReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_get_Capacity(This, value) \
    ((This)->lpVtbl->get_Capacity(This, value))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_add_Closed(This, handler, cookie) \
    ((This)->lpVtbl->add_Closed(This, handler, cookie))

#define __x_ABI_CWindows_CFoundation_CIMemoryBufferReference_remove_Closed(This, cookie) \
    ((This)->lpVtbl->remove_Closed(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIMemoryBufferReference;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIMemoryBufferReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IPropertyValue
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIPropertyValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IPropertyValue[] = L"Windows.Foundation.IPropertyValue";
typedef struct __x_ABI_CWindows_CFoundation_CIPropertyValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        enum __x_ABI_CWindows_CFoundation_CPropertyType* value);
    HRESULT (STDMETHODCALLTYPE* get_IsNumericScalar)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetUInt8)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* GetInt16)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        INT16* value);
    HRESULT (STDMETHODCALLTYPE* GetUInt16)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* GetInt32)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetUInt32)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetInt64)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* GetUInt64)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* GetSingle)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* GetDouble)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* GetChar16)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        WCHAR* value);
    HRESULT (STDMETHODCALLTYPE* GetBoolean)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetString)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetGuid)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* GetDateTime)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* GetTimeSpan)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* GetPoint)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* GetSize)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* GetRect)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* GetUInt8Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* GetInt16Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        INT16** value);
    HRESULT (STDMETHODCALLTYPE* GetUInt16Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        UINT16** value);
    HRESULT (STDMETHODCALLTYPE* GetInt32Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        INT32** value);
    HRESULT (STDMETHODCALLTYPE* GetUInt32Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        UINT32** value);
    HRESULT (STDMETHODCALLTYPE* GetInt64Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        INT64** value);
    HRESULT (STDMETHODCALLTYPE* GetUInt64Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        UINT64** value);
    HRESULT (STDMETHODCALLTYPE* GetSingleArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        FLOAT** value);
    HRESULT (STDMETHODCALLTYPE* GetDoubleArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        DOUBLE** value);
    HRESULT (STDMETHODCALLTYPE* GetChar16Array)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        WCHAR** value);
    HRESULT (STDMETHODCALLTYPE* GetBooleanArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        boolean** value);
    HRESULT (STDMETHODCALLTYPE* GetStringArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetInspectableArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        IInspectable*** value);
    HRESULT (STDMETHODCALLTYPE* GetGuidArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        GUID** value);
    HRESULT (STDMETHODCALLTYPE* GetDateTimeArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* GetTimeSpanArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* GetPointArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CPoint** value);
    HRESULT (STDMETHODCALLTYPE* GetSizeArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CSize** value);
    HRESULT (STDMETHODCALLTYPE* GetRectArray)(__x_ABI_CWindows_CFoundation_CIPropertyValue* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CRect** value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIPropertyValueVtbl;

interface __x_ABI_CWindows_CFoundation_CIPropertyValue
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIPropertyValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_get_IsNumericScalar(This, value) \
    ((This)->lpVtbl->get_IsNumericScalar(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt8(This, value) \
    ((This)->lpVtbl->GetUInt8(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInt16(This, value) \
    ((This)->lpVtbl->GetInt16(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt16(This, value) \
    ((This)->lpVtbl->GetUInt16(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInt32(This, value) \
    ((This)->lpVtbl->GetInt32(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt32(This, value) \
    ((This)->lpVtbl->GetUInt32(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInt64(This, value) \
    ((This)->lpVtbl->GetInt64(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt64(This, value) \
    ((This)->lpVtbl->GetUInt64(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetSingle(This, value) \
    ((This)->lpVtbl->GetSingle(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetDouble(This, value) \
    ((This)->lpVtbl->GetDouble(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetChar16(This, value) \
    ((This)->lpVtbl->GetChar16(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetBoolean(This, value) \
    ((This)->lpVtbl->GetBoolean(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetString(This, value) \
    ((This)->lpVtbl->GetString(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetGuid(This, value) \
    ((This)->lpVtbl->GetGuid(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetDateTime(This, value) \
    ((This)->lpVtbl->GetDateTime(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetTimeSpan(This, value) \
    ((This)->lpVtbl->GetTimeSpan(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetPoint(This, value) \
    ((This)->lpVtbl->GetPoint(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetSize(This, value) \
    ((This)->lpVtbl->GetSize(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetRect(This, value) \
    ((This)->lpVtbl->GetRect(This, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt8Array(This, valueLength, value) \
    ((This)->lpVtbl->GetUInt8Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInt16Array(This, valueLength, value) \
    ((This)->lpVtbl->GetInt16Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt16Array(This, valueLength, value) \
    ((This)->lpVtbl->GetUInt16Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInt32Array(This, valueLength, value) \
    ((This)->lpVtbl->GetInt32Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt32Array(This, valueLength, value) \
    ((This)->lpVtbl->GetUInt32Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInt64Array(This, valueLength, value) \
    ((This)->lpVtbl->GetInt64Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetUInt64Array(This, valueLength, value) \
    ((This)->lpVtbl->GetUInt64Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetSingleArray(This, valueLength, value) \
    ((This)->lpVtbl->GetSingleArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetDoubleArray(This, valueLength, value) \
    ((This)->lpVtbl->GetDoubleArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetChar16Array(This, valueLength, value) \
    ((This)->lpVtbl->GetChar16Array(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetBooleanArray(This, valueLength, value) \
    ((This)->lpVtbl->GetBooleanArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetStringArray(This, valueLength, value) \
    ((This)->lpVtbl->GetStringArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetInspectableArray(This, valueLength, value) \
    ((This)->lpVtbl->GetInspectableArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetGuidArray(This, valueLength, value) \
    ((This)->lpVtbl->GetGuidArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetDateTimeArray(This, valueLength, value) \
    ((This)->lpVtbl->GetDateTimeArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetTimeSpanArray(This, valueLength, value) \
    ((This)->lpVtbl->GetTimeSpanArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetPointArray(This, valueLength, value) \
    ((This)->lpVtbl->GetPointArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetSizeArray(This, valueLength, value) \
    ((This)->lpVtbl->GetSizeArray(This, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CIPropertyValue_GetRectArray(This, valueLength, value) \
    ((This)->lpVtbl->GetRectArray(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIPropertyValue;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIPropertyValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IPropertyValueStatics
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.PropertyValue
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IPropertyValueStatics[] = L"Windows.Foundation.IPropertyValueStatics";
typedef struct __x_ABI_CWindows_CFoundation_CIPropertyValueStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateEmpty)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt8)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        BYTE value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInt16)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        INT16 value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt16)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT16 value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInt32)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        INT32 value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt32)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInt64)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        INT64 value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt64)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT64 value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateSingle)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        FLOAT value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateDouble)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        DOUBLE value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateChar16)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        WCHAR value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateBoolean)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        boolean value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateString)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        HSTRING value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInspectable)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        IInspectable* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateGuid)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        GUID value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateDateTime)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateTimeSpan)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreatePoint)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateSize)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        struct __x_ABI_CWindows_CFoundation_CSize value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateRect)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        struct __x_ABI_CWindows_CFoundation_CRect value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt8Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        BYTE* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInt16Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        INT16* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt16Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        UINT16* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInt32Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        INT32* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt32Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        UINT32* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInt64Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        INT64* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateUInt64Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        UINT64* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateSingleArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        FLOAT* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateDoubleArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        DOUBLE* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateChar16Array)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        WCHAR* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateBooleanArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        boolean* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateStringArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        HSTRING* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateInspectableArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        IInspectable** value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateGuidArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        GUID* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateTimeSpanArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreatePointArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateSizeArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CSize* value,
        IInspectable** propertyValue);
    HRESULT (STDMETHODCALLTYPE* CreateRectArray)(__x_ABI_CWindows_CFoundation_CIPropertyValueStatics* This,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CRect* value,
        IInspectable** propertyValue);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIPropertyValueStaticsVtbl;

interface __x_ABI_CWindows_CFoundation_CIPropertyValueStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIPropertyValueStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateEmpty(This, propertyValue) \
    ((This)->lpVtbl->CreateEmpty(This, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt8(This, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt8(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInt16(This, value, propertyValue) \
    ((This)->lpVtbl->CreateInt16(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt16(This, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt16(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInt32(This, value, propertyValue) \
    ((This)->lpVtbl->CreateInt32(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt32(This, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt32(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInt64(This, value, propertyValue) \
    ((This)->lpVtbl->CreateInt64(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt64(This, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt64(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateSingle(This, value, propertyValue) \
    ((This)->lpVtbl->CreateSingle(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateDouble(This, value, propertyValue) \
    ((This)->lpVtbl->CreateDouble(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateChar16(This, value, propertyValue) \
    ((This)->lpVtbl->CreateChar16(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateBoolean(This, value, propertyValue) \
    ((This)->lpVtbl->CreateBoolean(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateString(This, value, propertyValue) \
    ((This)->lpVtbl->CreateString(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInspectable(This, value, propertyValue) \
    ((This)->lpVtbl->CreateInspectable(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateGuid(This, value, propertyValue) \
    ((This)->lpVtbl->CreateGuid(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateDateTime(This, value, propertyValue) \
    ((This)->lpVtbl->CreateDateTime(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateTimeSpan(This, value, propertyValue) \
    ((This)->lpVtbl->CreateTimeSpan(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreatePoint(This, value, propertyValue) \
    ((This)->lpVtbl->CreatePoint(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateSize(This, value, propertyValue) \
    ((This)->lpVtbl->CreateSize(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateRect(This, value, propertyValue) \
    ((This)->lpVtbl->CreateRect(This, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt8Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt8Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInt16Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateInt16Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt16Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt16Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInt32Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateInt32Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt32Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt32Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInt64Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateInt64Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateUInt64Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateUInt64Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateSingleArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateSingleArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateDoubleArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateDoubleArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateChar16Array(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateChar16Array(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateBooleanArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateBooleanArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateStringArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateStringArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateInspectableArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateInspectableArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateGuidArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateGuidArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateDateTimeArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateDateTimeArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateTimeSpanArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateTimeSpanArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreatePointArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreatePointArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateSizeArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateSizeArray(This, valueLength, value, propertyValue))

#define __x_ABI_CWindows_CFoundation_CIPropertyValueStatics_CreateRectArray(This, valueLength, value, propertyValue) \
    ((This)->lpVtbl->CreateRectArray(This, valueLength, value, propertyValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIPropertyValueStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIPropertyValueStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IStringable
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIStringable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIStringable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IStringable[] = L"Windows.Foundation.IStringable";
typedef struct __x_ABI_CWindows_CFoundation_CIStringableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIStringable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIStringable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIStringable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIStringable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIStringable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIStringable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ToString)(__x_ABI_CWindows_CFoundation_CIStringable* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIStringableVtbl;

interface __x_ABI_CWindows_CFoundation_CIStringable
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIStringableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIStringable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIStringable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIStringable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIStringable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIStringable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIStringable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIStringable_ToString(This, value) \
    ((This)->lpVtbl->ToString(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIStringable;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIStringable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriEscapeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriEscapeStatics[] = L"Windows.Foundation.IUriEscapeStatics";
typedef struct __x_ABI_CWindows_CFoundation_CIUriEscapeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UnescapeComponent)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This,
        HSTRING toUnescape,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* EscapeComponent)(__x_ABI_CWindows_CFoundation_CIUriEscapeStatics* This,
        HSTRING toEscape,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIUriEscapeStaticsVtbl;

interface __x_ABI_CWindows_CFoundation_CIUriEscapeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIUriEscapeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_UnescapeComponent(This, toUnescape, value) \
    ((This)->lpVtbl->UnescapeComponent(This, toUnescape, value))

#define __x_ABI_CWindows_CFoundation_CIUriEscapeStatics_EscapeComponent(This, toEscape, value) \
    ((This)->lpVtbl->EscapeComponent(This, toEscape, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriEscapeStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriEscapeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriRuntimeClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriRuntimeClass[] = L"Windows.Foundation.IUriRuntimeClass";
typedef struct __x_ABI_CWindows_CFoundation_CIUriRuntimeClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Domain)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Extension)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Fragment)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Host)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Password)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Query)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_QueryParsed)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass** ppWwwFormUrlDecoder);
    HRESULT (STDMETHODCALLTYPE* get_RawUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SchemeName)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserName)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Port)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Suspicious)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* Equals)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* pUri,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* CombineUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClass* This,
        HSTRING relativeUri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** instance);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIUriRuntimeClassVtbl;

interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIUriRuntimeClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_AbsoluteUri(This, value) \
    ((This)->lpVtbl->get_AbsoluteUri(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_DisplayUri(This, value) \
    ((This)->lpVtbl->get_DisplayUri(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Domain(This, value) \
    ((This)->lpVtbl->get_Domain(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Extension(This, value) \
    ((This)->lpVtbl->get_Extension(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Fragment(This, value) \
    ((This)->lpVtbl->get_Fragment(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Host(This, value) \
    ((This)->lpVtbl->get_Host(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Password(This, value) \
    ((This)->lpVtbl->get_Password(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Query(This, value) \
    ((This)->lpVtbl->get_Query(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_QueryParsed(This, ppWwwFormUrlDecoder) \
    ((This)->lpVtbl->get_QueryParsed(This, ppWwwFormUrlDecoder))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_RawUri(This, value) \
    ((This)->lpVtbl->get_RawUri(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_SchemeName(This, value) \
    ((This)->lpVtbl->get_SchemeName(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_UserName(This, value) \
    ((This)->lpVtbl->get_UserName(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Port(This, value) \
    ((This)->lpVtbl->get_Port(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_get_Suspicious(This, value) \
    ((This)->lpVtbl->get_Suspicious(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_Equals(This, pUri, value) \
    ((This)->lpVtbl->Equals(This, pUri, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass_CombineUri(This, relativeUri, instance) \
    ((This)->lpVtbl->CombineUri(This, relativeUri, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriRuntimeClass;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriRuntimeClassFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriRuntimeClassFactory[] = L"Windows.Foundation.IUriRuntimeClassFactory";
typedef struct __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This,
        HSTRING uri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** instance);
    HRESULT (STDMETHODCALLTYPE* CreateWithRelativeUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory* This,
        HSTRING baseUri,
        HSTRING relativeUri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** instance);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_CreateUri(This, uri, instance) \
    ((This)->lpVtbl->CreateUri(This, uri, instance))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_CreateWithRelativeUri(This, baseUri, relativeUri, instance) \
    ((This)->lpVtbl->CreateWithRelativeUri(This, baseUri, relativeUri, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Uri
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IUriRuntimeClassWithAbsoluteCanonicalUri[] = L"Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri";
typedef struct __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AbsoluteCanonicalUri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayIri)(__x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUriVtbl;

interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_get_AbsoluteCanonicalUri(This, value) \
    ((This)->lpVtbl->get_AbsoluteCanonicalUri(This, value))

#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_get_DisplayIri(This, value) \
    ((This)->lpVtbl->get_DisplayIri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIUriRuntimeClassWithAbsoluteCanonicalUri_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IWwwFormUrlDecoderEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IWwwFormUrlDecoderEntry[] = L"Windows.Foundation.IWwwFormUrlDecoderEntry";
typedef struct __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntryVtbl;

interface __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IWwwFormUrlDecoderRuntimeClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.WwwFormUrlDecoder
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IWwwFormUrlDecoderRuntimeClass[] = L"Windows.Foundation.IWwwFormUrlDecoderRuntimeClass";
typedef struct __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFirstValueByName)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass* This,
        HSTRING name,
        HSTRING* phstrValue);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassVtbl;

interface __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_GetFirstValueByName(This, name, phstrValue) \
    ((This)->lpVtbl->GetFirstValueByName(This, name, phstrValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.WwwFormUrlDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IWwwFormUrlDecoderRuntimeClassFactory[] = L"Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory";
typedef struct __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWwwFormUrlDecoder)(__x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory* This,
        HSTRING query,
        __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClass** instance);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_CreateWwwFormUrlDecoder(This, query, instance) \
    ((This)->lpVtbl->CreateWwwFormUrlDecoder(This, query, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIWwwFormUrlDecoderRuntimeClassFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Collections.PropertySet
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Collections_PropertySet_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Collections_PropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Collections_PropertySet[] = L"Windows.Foundation.Collections.PropertySet";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Collections.StringMap
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<String, String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>
 *    Windows.Foundation.Collections.IObservableMap`2<String, String>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Collections_StringMap_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Collections_StringMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Collections_StringMap[] = L"Windows.Foundation.Collections.StringMap";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Collections.ValueSet
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Collections_ValueSet_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Collections_ValueSet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Collections_ValueSet[] = L"Windows.Foundation.Collections.ValueSet";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Deferral
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IDeferralFactory interface starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IDeferral ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Deferral_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Deferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Deferral[] = L"Windows.Foundation.Deferral";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.GuidHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.IGuidHelperStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Foundation_GuidHelper_DEFINED
#define RUNTIMECLASS_Windows_Foundation_GuidHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_GuidHelper[] = L"Windows.Foundation.GuidHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Foundation.MemoryBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IMemoryBufferFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IMemoryBuffer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_MemoryBuffer_DEFINED
#define RUNTIMECLASS_Windows_Foundation_MemoryBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_MemoryBuffer[] = L"Windows.Foundation.MemoryBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.PropertyValue
 *
 * Introduced to Windows.Foundation.FoundationContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.IPropertyValueStatics interface starting with version 1.0 of the Windows.Foundation.FoundationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_PropertyValue_DEFINED
#define RUNTIMECLASS_Windows_Foundation_PropertyValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_PropertyValue[] = L"Windows.Foundation.PropertyValue";
#endif
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Uri
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IUriRuntimeClassFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.IUriEscapeStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IUriRuntimeClass ** Default Interface **
 *    Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Uri_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Uri_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Uri[] = L"Windows.Foundation.Uri";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.WwwFormUrlDecoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IWwwFormUrlDecoderRuntimeClass ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.IWwwFormUrlDecoderEntry>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoder_DEFINED
#define RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_WwwFormUrlDecoder[] = L"Windows.Foundation.WwwFormUrlDecoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.WwwFormUrlDecoderEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IWwwFormUrlDecoderEntry ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoderEntry_DEFINED
#define RUNTIMECLASS_Windows_Foundation_WwwFormUrlDecoderEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_WwwFormUrlDecoderEntry[] = L"Windows.Foundation.WwwFormUrlDecoderEntry";
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
#endif // __windows2Efoundation_p_h__

#endif // __windows2Efoundation_h__
