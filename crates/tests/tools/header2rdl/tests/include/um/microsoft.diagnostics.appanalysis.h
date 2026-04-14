/* Header file automatically generated from microsoft.diagnostics.appanalysis.idl */
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
#ifndef __microsoft2Ediagnostics2Eappanalysis_h__
#define __microsoft2Ediagnostics2Eappanalysis_h__
#ifndef __microsoft2Ediagnostics2Eappanalysis_p_h__
#define __microsoft2Ediagnostics2Eappanalysis_p_h__


#pragma once

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
#include "Windows.Foundation.h"
#include "eventtoken.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwEvent;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwEventFactory;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventFactory

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwProvider;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwProviderStatics;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProviderStatics

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwEventRecordCallback;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventRecordCallback

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwEventWatcher;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventWatcher

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwRule;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwRuleFactory;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRuleFactory

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IResourceStringFactory;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory ABI::Microsoft::Diagnostics::AppAnalysis::IResourceStringFactory

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IResourceString;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString ABI::Microsoft::Diagnostics::AppAnalysis::IResourceString

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IResourceStringView;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView ABI::Microsoft::Diagnostics::AppAnalysis::IResourceStringView

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IRule;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule ABI::Microsoft::Diagnostics::AppAnalysis::IRule

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IEtwEventRecord;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventRecord

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IRuleTriggeredEventArgs;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs ABI::Microsoft::Diagnostics::AppAnalysis::IRuleTriggeredEventArgs

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_FWD_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                interface IRuleTriggeredEventArgsFactory;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory ABI::Microsoft::Diagnostics::AppAnalysis::IRuleTriggeredEventArgsFactory

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class EtwEvent;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */



#ifndef DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#define DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("13d95680-2f8a-5102-aaf4-c371f5b537a2"))
IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Microsoft.Diagnostics.AppAnalysis.EtwEvent>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t;
#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::__FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
//#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t ABI::Windows::Foundation::Collections::IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE */





#ifndef DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#define DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a23e2728-2f89-5d64-bb57-d081a5a8fe71"))
IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Microsoft.Diagnostics.AppAnalysis.EtwEvent>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t;
#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::__FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
//#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t ABI::Windows::Foundation::Collections::IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE */





#ifndef DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#define DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("534e6c5a-80fc-5078-9aea-d7d32e47b10e"))
IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Microsoft.Diagnostics.AppAnalysis.EtwEvent>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t;
#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::__FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
//#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::IIterator<HSTRING>
//#define __FIIterator_1_HSTRING_t ABI::Windows::Foundation::Collections::IIterator<HSTRING>
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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::IIterable<HSTRING>
//#define __FIIterable_1_HSTRING_t ABI::Windows::Foundation::Collections::IIterable<HSTRING>
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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::IVectorView<HSTRING>
//#define __FIVectorView_1_HSTRING_t ABI::Windows::Foundation::Collections::IVectorView<HSTRING>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::IVector<HSTRING>
//#define __FIVector_1_HSTRING_t ABI::Windows::Foundation::Collections::IVector<HSTRING>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */




namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class RuleTriggeredEventArgs;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */



#ifndef DEF___FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_USE
#define DEF___FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("79971e44-6520-58a7-b3a2-3b596d54e4e0"))
ITypedEventHandler<ABI::Microsoft::Diagnostics::AppAnalysis::IRule*,ABI::Microsoft::Diagnostics::AppAnalysis::RuleTriggeredEventArgs*> : ITypedEventHandler_impl<ABI::Microsoft::Diagnostics::AppAnalysis::IRule*,ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::RuleTriggeredEventArgs*, ABI::Microsoft::Diagnostics::AppAnalysis::IRuleTriggeredEventArgs*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.TypedEventHandler`2<Microsoft.Diagnostics.AppAnalysis.IRule, Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Microsoft::Diagnostics::AppAnalysis::IRule*,ABI::Microsoft::Diagnostics::AppAnalysis::RuleTriggeredEventArgs*> __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_t;
#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs ABI::Windows::Foundation::ITypedEventHandler<ABI::Microsoft::Diagnostics::AppAnalysis::IRule*,ABI::Microsoft::Diagnostics::AppAnalysis::IRuleTriggeredEventArgs*>
//#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_t ABI::Windows::Foundation::ITypedEventHandler<ABI::Microsoft::Diagnostics::AppAnalysis::IRule*,ABI::Microsoft::Diagnostics::AppAnalysis::IRuleTriggeredEventArgs*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_USE */



namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class EtwRule;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */



#ifndef DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#define DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b90eb3f9-0d1e-576f-84a9-3891a568fa9a"))
IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Microsoft.Diagnostics.AppAnalysis.EtwRule>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t;
#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::__FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
//#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t ABI::Windows::Foundation::Collections::IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE */





#ifndef DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#define DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55e15cef-52f2-5cab-ae3e-c5cadfdb96ab"))
IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Microsoft.Diagnostics.AppAnalysis.EtwRule>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t;
#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::__FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
//#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t ABI::Windows::Foundation::Collections::IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE */





#ifndef DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#define DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("64813dce-21e0-5ef3-8400-e40ee34c1d37"))
IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Microsoft.Diagnostics.AppAnalysis.EtwRule>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t;
#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::__FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
//#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE */



namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class EtwProvider;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */



#ifndef DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#define DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b1d212ca-e143-5aa7-98a9-1dc8c77d5987"))
IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Microsoft.Diagnostics.AppAnalysis.EtwProvider>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t;
#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::__FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
//#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t ABI::Windows::Foundation::Collections::IIterator<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE */





#ifndef DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#define DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("95a2f724-e78b-515f-8ded-d1c2890754db"))
IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Microsoft.Diagnostics.AppAnalysis.EtwProvider>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t;
#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::__FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
//#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t ABI::Windows::Foundation::Collections::IIterable<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE */





#ifndef DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#define DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b4eb0239-6627-5082-b343-4e7ebf68fb18"))
IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Microsoft.Diagnostics.AppAnalysis.EtwProvider>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t;
#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::__FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
//#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE */





#ifndef DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#define DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3b5c9d3d-550b-5b7c-9c03-66e8018db03f"))
IVector<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVector`1<Microsoft.Diagnostics.AppAnalysis.EtwProvider>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Microsoft::Diagnostics::AppAnalysis::EtwProvider*> __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t;
#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::__FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider ABI::Windows::Foundation::Collections::IVector<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
//#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_t ABI::Windows::Foundation::Collections::IVector<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_USE */





#ifndef DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#define DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("38039511-50fe-5f79-915c-74a775a60779"))
IVector<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVector`1<Microsoft.Diagnostics.AppAnalysis.EtwEvent>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Microsoft::Diagnostics::AppAnalysis::EtwEvent*> __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t;
#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::__FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent ABI::Windows::Foundation::Collections::IVector<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
//#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_t ABI::Windows::Foundation::Collections::IVector<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_USE */





#ifndef DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#define DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("477765ed-a54e-56a9-ada5-0478a755824b"))
IVector<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*, ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVector`1<Microsoft.Diagnostics.AppAnalysis.EtwRule>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Microsoft::Diagnostics::AppAnalysis::EtwRule*> __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t;
#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::__FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule ABI::Windows::Foundation::Collections::IVector<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
//#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_t ABI::Windows::Foundation::Collections::IVector<ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_USE */






#pragma warning (push)
#pragma warning (disable:4668) 
#pragma warning (disable:4001) 
#pragma once 
#pragma warning (pop)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// Win32 API definitions
#include <evntcons.h>
_Check_return_ STDAPI ProcessEvent(_In_ PEVENT_RECORD eventRecord);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */ 
#pragma endregion










namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class ResourceString;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */


namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class EtwEventRecord;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */




namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class EtwEventWatcher;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */



namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                class EtwRuleSet;
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */



/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.MeasurementUnit
 *
 */
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [version, v1_enum] */
                typedef 
                enum MeasurementUnit : int
                {
                    MeasurementUnit_Milliseconds = 0,
                    MeasurementUnit_Kilobytes,
                    MeasurementUnit_Elements,
                    MeasurementUnit_Percentage,
                } MeasurementUnit;
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.RuleCategories
 *
 */
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [flags, version, v1_enum] */
                typedef 
                enum RuleCategories : unsigned int
                {
                    RuleCategories_None = 0,
                    RuleCategories_Performance = 0x1,
                    RuleCategories_Accessibility = 0x2,
                } RuleCategories;
                
                DEFINE_ENUM_FLAG_OPERATORS(RuleCategories)
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.ProviderType
 *
 */
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [version, v1_enum] */
                typedef 
                enum ProviderType : int
                {
                    ProviderType_Manifest = 0,
                    ProviderType_Kernel,
                } ProviderType;
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.Measurement
 *
 */
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [version] */
                typedef 
                struct Measurement
                {
                    DOUBLE Value;
                    ABI::Microsoft::Diagnostics::AppAnalysis::MeasurementUnit Unit;
                } Measurement;
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.TimelineInfo
 *
 */
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [version] */
                typedef 
                struct TimelineInfo
                {
                    INT64 Start;
                    INT64 Stop;
                } TimelineInfo;
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEvent
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEvent
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEvent[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEvent";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("07A0C227-BBEB-43FA-AF86-108114187CDC"), version] */
                MIDL_INTERFACE("07A0C227-BBEB-43FA-AF86-108114187CDC")
                IEtwEvent : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_EventId(
                        /* [retval, out] */__RPC__out UINT16 * eventId
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_EventVersion(
                        /* [retval, out] */__RPC__out BYTE * eventVersion
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Provider(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider * * provider
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwEvent=__uuidof(IEtwEvent);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEventFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEvent
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEventFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEventFactory";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("80FDEE60-5888-47DB-92D3-E5BC3C9F57CF"), version] */
                MIDL_INTERFACE("80FDEE60-5888-47DB-92D3-E5BC3C9F57CF")
                IEtwEventFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        /* [in] */UINT16 eventId,
                        /* [in] */BYTE eventVersion,
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider * provider,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent * * instance
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwEventFactory=__uuidof(IEtwEventFactory);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwProvider
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwProvider
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwProvider[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwProvider";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("11624719-7C26-4BDE-A80F-441DF7670BB8"), version] */
                MIDL_INTERFACE("11624719-7C26-4BDE-A80F-441DF7670BB8")
                IEtwProvider : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ID(
                        /* [retval, out] */__RPC__out GUID * ID
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_EnableFlags(
                        /* [retval, out] */__RPC__out UINT32 * enableFlags
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ProviderType(
                        /* [retval, out] */__RPC__out ABI::Microsoft::Diagnostics::AppAnalysis::ProviderType * providerType
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Manifest(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * manifest
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwProvider=__uuidof(IEtwProvider);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwProviderStatics
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwProvider
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwProviderStatics[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwProviderStatics";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("598024F7-A743-4555-A1FD-5EB465308194"), version] */
                MIDL_INTERFACE("598024F7-A743-4555-A1FD-5EB465308194")
                IEtwProviderStatics : public IInspectable
                {
                public:
                    /* [default_overload, overload] */virtual HRESULT STDMETHODCALLTYPE Create(
                        /* [in] */GUID providerId,
                        /* [in] */__RPC__in HSTRING manifest,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider * * instance
                        ) = 0;
                    /* [overload] */virtual HRESULT STDMETHODCALLTYPE CreateKernelProvider(
                        /* [in] */GUID providerId,
                        /* [in] */UINT32 enableFlags,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwProvider * * instance
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwProviderStatics=__uuidof(IEtwProviderStatics);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_INTERFACE_DEFINED__) */


/*
 *
 * Delegate Microsoft.Diagnostics.AppAnalysis.EtwEventRecordCallback
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_INTERFACE_DEFINED__
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, uuid("8929948F-2A94-4ED4-ABB6-92EF45E61B21"), version] */
                MIDL_INTERFACE("8929948F-2A94-4ED4-ABB6-92EF45E61B21")
                IEtwEventRecordCallback : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventRecord * eventrecord
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwEventRecordCallback=__uuidof(IEtwEventRecordCallback);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEventWatcher
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEventWatcher
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEventWatcher[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEventWatcher";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("012A38E6-6B29-473E-A219-CAAE719E9996"), version] */
                MIDL_INTERFACE("012A38E6-6B29-473E-A219-CAAE719E9996")
                IEtwEventWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RegisterEvent(
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEvent * etwEvent,
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventRecordCallback  * eventCallback
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwEventWatcher=__uuidof(IEtwEventWatcher);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwRule
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwRule[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwRule";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("C2205647-2FD8-43E0-92CB-0ED1A0DB9E55"), version] */
                MIDL_INTERFACE("C2205647-2FD8-43E0-92CB-0ED1A0DB9E55")
                IEtwRule : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_BackingRule(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IRule * * backingRule
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_RegisteredEvents(
                        /* [retval, out] */__RPC__deref_out_opt __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * * registeredEvents
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwRule=__uuidof(IEtwRule);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwRuleFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwRuleFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwRuleFactory";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("363FA5D1-2E25-4CC0-8BCF-62ECD32ABF9A"), version] */
                MIDL_INTERFACE("363FA5D1-2E25-4CC0-8BCF-62ECD32ABF9A")
                IEtwRuleFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IRule * backingRule,
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwEventWatcher * watcher,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IEtwRule * * instance
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwRuleFactory=__uuidof(IEtwRuleFactory);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IResourceStringFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.ResourceString
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IResourceStringFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IResourceStringFactory";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("111BA2AD-ECE9-4FEE-8168-E94F290CC451"), version] */
                MIDL_INTERFACE("111BA2AD-ECE9-4FEE-8168-E94F290CC451")
                IResourceStringFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        /* [in] */UINT32 identifier,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceString * * instance
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IResourceStringFactory=__uuidof(IResourceStringFactory);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IResourceString
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.ResourceString
 *
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVector_1_HSTRING
 *     Windows.Foundation.Collections.IIterable_1_HSTRING
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IResourceString[] = L"Microsoft.Diagnostics.AppAnalysis.IResourceString";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("CD75217D-5BF1-4056-9253-71D9C57D2B06"), version] */
                MIDL_INTERFACE("CD75217D-5BF1-4056-9253-71D9C57D2B06")
                IResourceString : public IInspectable
                {
                public:
                    /* [propput] */virtual HRESULT STDMETHODCALLTYPE put_Identifier(
                        /* [in] */UINT32 identifer
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Identifier(
                        /* [retval, out] */__RPC__out UINT32 * identifer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetResourceStringView(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceStringView * * view
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IResourceString=__uuidof(IResourceString);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IResourceStringView
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView_1_HSTRING
 *     Windows.Foundation.Collections.IIterable_1_HSTRING
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IResourceStringView[] = L"Microsoft.Diagnostics.AppAnalysis.IResourceStringView";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, uuid("E433A2D7-AAB7-4749-A66C-1530941AC47B"), version] */
                MIDL_INTERFACE("E433A2D7-AAB7-4749-A66C-1530941AC47B")
                IResourceStringView : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Identifier(
                        /* [retval, out] */__RPC__out UINT32 * identifer
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IResourceStringView=__uuidof(IResourceStringView);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IRule
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IRule[] = L"Microsoft.Diagnostics.AppAnalysis.IRule";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, uuid("155EB649-1F41-465F-8C13-D49DFD8EB7D4"), version] */
                MIDL_INTERFACE("155EB649-1F41-465F-8C13-D49DFD8EB7D4")
                IRule : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Id(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * id
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Title(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * title
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Impact(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * impact
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_LinkTitle(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * linkTitle
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_LinkUri(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * linkUri
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Categories(
                        /* [retval, out] */__RPC__out ABI::Microsoft::Diagnostics::AppAnalysis::RuleCategories * categories
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FormatString(
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceStringView * resourceString,
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * result
                        ) = 0;
                    /* [eventadd] */virtual HRESULT STDMETHODCALLTYPE add_Triggered(
                        /* [in] */__RPC__in_opt __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs * handler,
                        /* [retval, out] */__RPC__out EventRegistrationToken * token
                        ) = 0;
                    /* [eventremove] */virtual HRESULT STDMETHODCALLTYPE remove_Triggered(
                        /* [in] */EventRegistrationToken token
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IRule=__uuidof(IRule);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEventRecord
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEventRecord
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEventRecord[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEventRecord";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("79380594-6095-49C8-82E6-65DDC099DC59"), version] */
                MIDL_INTERFACE("79380594-6095-49C8-82E6-65DDC099DC59")
                IEtwEventRecord : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        /* [retval, out] */__RPC__out INT64 * timestamp
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ThreadId(
                        /* [retval, out] */__RPC__out UINT32 * threadId
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_EventId(
                        /* [retval, out] */__RPC__out UINT16 * eventId
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_EventVersion(
                        /* [retval, out] */__RPC__out BYTE * eventVersion
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        /* [retval, out] */__RPC__out GUID * providerId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBooleanProperty(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFloatProperty(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out float * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDoubleProperty(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out double * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInt32Property(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out INT32 * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUInt32Property(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out UINT32 * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInt64Property(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out INT64 * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUInt64Property(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__out UINT64 * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUnicodeStringProperty(
                        /* [in] */__RPC__in HSTRING propertyName,
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IEtwEventRecord=__uuidof(IEtwEventRecord);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgs
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IRuleTriggeredEventArgs[] = L"Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgs";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("E60D2D49-19D8-4614-A526-5BC97EFA72CB"), version] */
                MIDL_INTERFACE("E60D2D49-19D8-4614-A526-5BC97EFA72CB")
                IRuleTriggeredEventArgs : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_TimelineStart(
                        /* [retval, out] */__RPC__out INT64 * timelineStart
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_TimelineStop(
                        /* [retval, out] */__RPC__out INT64 * timelineStop
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_FileName(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * fileName
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_LineNumber(
                        /* [retval, out] */__RPC__out UINT32 * lineNumber
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ColumnNumber(
                        /* [retval, out] */__RPC__out UINT32 * columnNumber
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ElementId(
                        /* [retval, out] */__RPC__out UINT64 * elementId
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_MeasurementUnit(
                        /* [retval, out] */__RPC__out ABI::Microsoft::Diagnostics::AppAnalysis::MeasurementUnit * measurementUnit
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_MeasurementValue(
                        /* [retval, out] */__RPC__out double * measurementValue
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Description(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceStringView * * description
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Solution(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceStringView * * solution
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_FileHash(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * hash
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IRuleTriggeredEventArgs=__uuidof(IRuleTriggeredEventArgs);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgsFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IRuleTriggeredEventArgsFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgsFactory";
namespace ABI {
    namespace Microsoft {
        namespace Diagnostics {
            namespace AppAnalysis {
                /* [object, exclusiveto, uuid("B6090E2B-D71F-42EE-80DA-3393DA4038FD"), version] */
                MIDL_INTERFACE("B6090E2B-D71F-42EE-80DA-3393DA4038FD")
                IRuleTriggeredEventArgsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        /* [in] */UINT64 elementId,
                        /* [in] */ABI::Microsoft::Diagnostics::AppAnalysis::TimelineInfo timelineInfo,
                        /* [in] */ABI::Microsoft::Diagnostics::AppAnalysis::Measurement measurement,
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceString * description,
                        /* [in] */__RPC__in_opt ABI::Microsoft::Diagnostics::AppAnalysis::IResourceString * solution,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Microsoft::Diagnostics::AppAnalysis::IRuleTriggeredEventArgs * * instance
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IRuleTriggeredEventArgsFactory=__uuidof(IRuleTriggeredEventArgsFactory);
                
            } /* AppAnalysis */
        } /* Diagnostics */
    } /* Microsoft */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_INTERFACE_DEFINED__) */


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_RuleTriggeredEventArgs_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_RuleTriggeredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_RuleTriggeredEventArgs[] = L"Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwEventWatcher
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwEventWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventWatcher_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwEventWatcher[] = L"Microsoft.Diagnostics.AppAnalysis.EtwEventWatcher";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwRuleSet
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView_1_Microsoft.Diagnostics.AppAnalysis.EtwRule ** Default Interface **
 *    Windows.Foundation.Collections.IIterable_1_Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRuleSet_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRuleSet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwRuleSet[] = L"Microsoft.Diagnostics.AppAnalysis.EtwRuleSet";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwEvent
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwEvent ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEvent_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEvent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwEvent[] = L"Microsoft.Diagnostics.AppAnalysis.EtwEvent";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwProvider
 *
 * RuntimeClass contains static methods.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwProvider_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwProvider[] = L"Microsoft.Diagnostics.AppAnalysis.EtwProvider";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwEventRecord
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwEventRecord ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventRecord_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventRecord_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwEventRecord[] = L"Microsoft.Diagnostics.AppAnalysis.EtwEventRecord";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwRule ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRule_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRule_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwRule[] = L"Microsoft.Diagnostics.AppAnalysis.EtwRule";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.ResourceString
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IResourceString ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_ResourceString_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_ResourceString_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_ResourceString[] = L"Microsoft.Diagnostics.AppAnalysis.ResourceString";
#endif



#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_FWD_DEFINED__
typedef interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory;

#endif // ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__)
#define ____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

typedef struct __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [retval][out] */ __RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl;

interface __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent
{
    CONST_VTBL struct __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__



#if !defined(____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__)
#define ____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

typedef  struct __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent **first);

    END_INTERFACE
} __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl;

interface __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent
{
    CONST_VTBL struct __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__



#if !defined(____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

typedef struct __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
            /* [in] */ __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl;

interface __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent
{
    CONST_VTBL struct __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__


#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_HSTRING * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_HSTRING * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_HSTRING * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_HSTRING * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_HSTRING * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_HSTRING * This, /* [retval][out] */ __RPC__out HSTRING *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_HSTRING * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_HSTRING * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_HSTRING * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) HSTRING *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_HSTRING_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_HSTRING_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_HSTRING_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_HSTRING_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__


#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef  struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_HSTRING * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_HSTRING * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_HSTRING * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_HSTRING * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_HSTRING * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_HSTRING * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_HSTRING * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_HSTRING **first);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_HSTRING_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

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

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_HSTRING * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_HSTRING * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_HSTRING * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_HSTRING * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_HSTRING * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_HSTRING * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_HSTRING * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out HSTRING *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_HSTRING * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_HSTRING * This,
            /* [in] */ HSTRING item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_HSTRING * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) HSTRING *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_HSTRING_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_HSTRING_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_HSTRING_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_HSTRING_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__


#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_HSTRING * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_HSTRING * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_HSTRING * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_HSTRING * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_HSTRING * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt HSTRING *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_HSTRING * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_HSTRING * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_HSTRING **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ __RPC__in HSTRING item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in HSTRING item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in HSTRING item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_HSTRING * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_HSTRING * This, /* [in] */ __RPC__in HSTRING item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) HSTRING *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) HSTRING *value);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_HSTRING_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_HSTRING_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_HSTRING_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_HSTRING_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_HSTRING_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_HSTRING_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_HSTRING_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_HSTRING_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_HSTRING_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_HSTRING_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_HSTRING_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_HSTRING_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__




#if !defined(____FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs;

typedef struct __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs * This,/* [in] */ __RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * sender,/* [in] */ __RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * e);
    END_INTERFACE
} __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgsVtbl;

interface __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_Invoke(This,sender,e)	\
    ( (This)->lpVtbl -> Invoke(This,sender,e) ) 
#endif /* COBJMACROS */



#endif // ____FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs_INTERFACE_DEFINED__



#if !defined(____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__)
#define ____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

typedef struct __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [retval][out] */ __RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl;

interface __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule
{
    CONST_VTBL struct __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__



#if !defined(____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__)
#define ____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

typedef  struct __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule **first);

    END_INTERFACE
} __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl;

interface __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule
{
    CONST_VTBL struct __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__



#if !defined(____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

typedef struct __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
            /* [in] */ __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl;

interface __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule
{
    CONST_VTBL struct __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__



#if !defined(____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__)
#define ____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

typedef struct __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [retval][out] */ __RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl;

interface __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider
{
    CONST_VTBL struct __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__



#if !defined(____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__)
#define ____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

typedef  struct __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider **first);

    END_INTERFACE
} __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl;

interface __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider
{
    CONST_VTBL struct __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__



#if !defined(____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

typedef struct __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
            /* [in] */ __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl;

interface __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider
{
    CONST_VTBL struct __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__



#if !defined(____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__)
#define ____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__

typedef interface __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider;

typedef struct __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [out] */ __RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This, /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * *value);

    END_INTERFACE
} __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl;

interface __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider
{
    CONST_VTBL struct __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProviderVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwProvider_INTERFACE_DEFINED__



#if !defined(____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__)
#define ____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__

typedef interface __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent;

typedef struct __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [out] */ __RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This, /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * *value);

    END_INTERFACE
} __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl;

interface __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent
{
    CONST_VTBL struct __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEventVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent_INTERFACE_DEFINED__



#if !defined(____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__)
#define ____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__

typedef interface __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule;

typedef struct __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [out] */ __RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This, /* [in] */ __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * *value);

    END_INTERFACE
} __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl;

interface __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule
{
    CONST_VTBL struct __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRuleVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwRule_INTERFACE_DEFINED__




#pragma warning (push)
#pragma warning (disable:4668) 
#pragma warning (disable:4001) 
#pragma once 
#pragma warning (pop)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// Win32 API definitions
#include <evntcons.h>
_Check_return_ STDAPI ProcessEvent(_In_ PEVENT_RECORD eventRecord);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */ 
#pragma endregion


















/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.MeasurementUnit
 *
 */
/* [version, v1_enum] */
typedef 
enum __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurementUnit
{
    MeasurementUnit_Milliseconds = 0,
    MeasurementUnit_Kilobytes,
    MeasurementUnit_Elements,
    MeasurementUnit_Percentage,
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurementUnit;


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.RuleCategories
 *
 */
/* [flags, version, v1_enum] */
typedef 
enum __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CRuleCategories
{
    RuleCategories_None = 0,
    RuleCategories_Performance = 0x1,
    RuleCategories_Accessibility = 0x2,
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CRuleCategories;


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.ProviderType
 *
 */
/* [version, v1_enum] */
typedef 
enum __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CProviderType
{
    ProviderType_Manifest = 0,
    ProviderType_Kernel,
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CProviderType;


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.Measurement
 *
 */
/* [version] */
typedef 
struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurement
{
    DOUBLE Value;
    __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurementUnit Unit;
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurement;


/*
 *
 * Typedef of Microsoft.Diagnostics.AppAnalysis.TimelineInfo
 *
 */
/* [version] */
typedef 
struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CTimelineInfo
{
    INT64 Start;
    INT64 Stop;
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CTimelineInfo;


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEvent
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEvent
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEvent[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEvent";
/* [object, exclusiveto, uuid("07A0C227-BBEB-43FA-AF86-108114187CDC"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_EventId )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
        /* [retval, out] */__RPC__out UINT16 * eventId
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_EventVersion )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
        /* [retval, out] */__RPC__out BYTE * eventVersion
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Provider )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * * provider
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_get_EventId(This,eventId) \
    ( (This)->lpVtbl->get_EventId(This,eventId) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_get_EventVersion(This,eventVersion) \
    ( (This)->lpVtbl->get_EventVersion(This,eventVersion) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_get_Provider(This,provider) \
    ( (This)->lpVtbl->get_Provider(This,provider) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEventFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEvent
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEventFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEventFactory";
/* [object, exclusiveto, uuid("80FDEE60-5888-47DB-92D3-E5BC3C9F57CF"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactoryVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateInstance )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory * This,
        /* [in] */UINT16 eventId,
        /* [in] */BYTE eventVersion,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * provider,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * * instance
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactoryVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactoryVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_CreateInstance(This,eventId,eventVersion,provider,instance) \
    ( (This)->lpVtbl->CreateInstance(This,eventId,eventVersion,provider,instance) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventFactory_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwProvider
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwProvider
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwProvider[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwProvider";
/* [object, exclusiveto, uuid("11624719-7C26-4BDE-A80F-441DF7670BB8"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ID )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
        /* [retval, out] */__RPC__out GUID * ID
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_EnableFlags )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
        /* [retval, out] */__RPC__out UINT32 * enableFlags
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ProviderType )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
        /* [retval, out] */__RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CProviderType * providerType
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Manifest )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * manifest
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_get_ID(This,ID) \
    ( (This)->lpVtbl->get_ID(This,ID) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_get_EnableFlags(This,enableFlags) \
    ( (This)->lpVtbl->get_EnableFlags(This,enableFlags) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_get_ProviderType(This,providerType) \
    ( (This)->lpVtbl->get_ProviderType(This,providerType) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_get_Manifest(This,manifest) \
    ( (This)->lpVtbl->get_Manifest(This,manifest) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwProviderStatics
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwProvider
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwProviderStatics[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwProviderStatics";
/* [object, exclusiveto, uuid("598024F7-A743-4555-A1FD-5EB465308194"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStaticsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [default_overload, overload] */HRESULT ( STDMETHODCALLTYPE *Create )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This,
        /* [in] */GUID providerId,
        /* [in] */__RPC__in HSTRING manifest,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * * instance
        );
    /* [overload] */HRESULT ( STDMETHODCALLTYPE *CreateKernelProvider )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics * This,
        /* [in] */GUID providerId,
        /* [in] */UINT32 enableFlags,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProvider * * instance
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStaticsVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStaticsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_Create(This,providerId,manifest,instance) \
    ( (This)->lpVtbl->Create(This,providerId,manifest,instance) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_CreateKernelProvider(This,providerId,enableFlags,instance) \
    ( (This)->lpVtbl->CreateKernelProvider(This,providerId,enableFlags,instance) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwProviderStatics_INTERFACE_DEFINED__) */


/*
 *
 * Delegate Microsoft.Diagnostics.AppAnalysis.EtwEventRecordCallback
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_INTERFACE_DEFINED__
/* [object, uuid("8929948F-2A94-4ED4-ABB6-92EF45E61B21"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallbackVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject);

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback * This);

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback * This);
HRESULT ( STDMETHODCALLTYPE *Invoke )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback * This,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * eventrecord
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallbackVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallbackVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_Invoke(This,eventrecord) \
    ( (This)->lpVtbl->Invoke(This,eventrecord) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEventWatcher
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEventWatcher
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEventWatcher[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEventWatcher";
/* [object, exclusiveto, uuid("012A38E6-6B29-473E-A219-CAAE719E9996"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcherVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *RegisterEvent )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * This,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEvent * etwEvent,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordCallback  * eventCallback
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcherVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcherVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_RegisterEvent(This,etwEvent,eventCallback) \
    ( (This)->lpVtbl->RegisterEvent(This,etwEvent,eventCallback) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwRule
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwRule[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwRule";
/* [object, exclusiveto, uuid("C2205647-2FD8-43E0-92CB-0ED1A0DB9E55"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_BackingRule )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * * backingRule
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_RegisteredEvents )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVectorView_1_Microsoft__CDiagnostics__CAppAnalysis__CEtwEvent * * registeredEvents
        );
    HRESULT ( STDMETHODCALLTYPE *Start )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This
        );
    HRESULT ( STDMETHODCALLTYPE *Stop )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * This
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_get_BackingRule(This,backingRule) \
    ( (This)->lpVtbl->get_BackingRule(This,backingRule) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_get_RegisteredEvents(This,registeredEvents) \
    ( (This)->lpVtbl->get_RegisteredEvents(This,registeredEvents) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_Start(This) \
    ( (This)->lpVtbl->Start(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_Stop(This) \
    ( (This)->lpVtbl->Stop(This) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwRuleFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwRuleFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwRuleFactory";
/* [object, exclusiveto, uuid("363FA5D1-2E25-4CC0-8BCF-62ECD32ABF9A"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactoryVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateInstance )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory * This,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * backingRule,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventWatcher * watcher,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRule * * instance
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactoryVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactoryVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_CreateInstance(This,backingRule,watcher,instance) \
    ( (This)->lpVtbl->CreateInstance(This,backingRule,watcher,instance) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwRuleFactory_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IResourceStringFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.ResourceString
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IResourceStringFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IResourceStringFactory";
/* [object, exclusiveto, uuid("111BA2AD-ECE9-4FEE-8168-E94F290CC451"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactoryVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateInstance )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory * This,
        /* [in] */UINT32 identifier,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * * instance
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactoryVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactoryVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_CreateInstance(This,identifier,instance) \
    ( (This)->lpVtbl->CreateInstance(This,identifier,instance) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringFactory_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IResourceString
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.ResourceString
 *
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVector_1_HSTRING
 *     Windows.Foundation.Collections.IIterable_1_HSTRING
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IResourceString[] = L"Microsoft.Diagnostics.AppAnalysis.IResourceString";
/* [object, exclusiveto, uuid("CD75217D-5BF1-4056-9253-71D9C57D2B06"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propput] */HRESULT ( STDMETHODCALLTYPE *put_Identifier )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
        /* [in] */UINT32 identifer
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Identifier )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
        /* [retval, out] */__RPC__out UINT32 * identifer
        );
    HRESULT ( STDMETHODCALLTYPE *GetResourceStringView )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * * view
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_put_Identifier(This,identifer) \
    ( (This)->lpVtbl->put_Identifier(This,identifer) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_get_Identifier(This,identifer) \
    ( (This)->lpVtbl->get_Identifier(This,identifer) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_GetResourceStringView(This,view) \
    ( (This)->lpVtbl->GetResourceStringView(This,view) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IResourceStringView
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView_1_HSTRING
 *     Windows.Foundation.Collections.IIterable_1_HSTRING
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IResourceStringView[] = L"Microsoft.Diagnostics.AppAnalysis.IResourceStringView";
/* [object, uuid("E433A2D7-AAB7-4749-A66C-1530941AC47B"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringViewVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Identifier )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * This,
        /* [retval, out] */__RPC__out UINT32 * identifer
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringViewVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringViewVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_get_Identifier(This,identifer) \
    ( (This)->lpVtbl->get_Identifier(This,identifer) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IRule
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IRule[] = L"Microsoft.Diagnostics.AppAnalysis.IRule";
/* [object, uuid("155EB649-1F41-465F-8C13-D49DFD8EB7D4"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Id )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * id
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Title )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * title
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Impact )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * impact
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_LinkTitle )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * linkTitle
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_LinkUri )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * linkUri
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Categories )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [retval, out] */__RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CRuleCategories * categories
        );
    HRESULT ( STDMETHODCALLTYPE *FormatString )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * resourceString,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * result
        );
    /* [eventadd] */HRESULT ( STDMETHODCALLTYPE *add_Triggered )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [in] */__RPC__in_opt __FITypedEventHandler_2_Microsoft__CDiagnostics__CAppAnalysis__CIRule_Microsoft__CDiagnostics__CAppAnalysis__CRuleTriggeredEventArgs * handler,
        /* [retval, out] */__RPC__out EventRegistrationToken * token
        );
    /* [eventremove] */HRESULT ( STDMETHODCALLTYPE *remove_Triggered )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule * This,
        /* [in] */EventRegistrationToken token
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_get_Id(This,id) \
    ( (This)->lpVtbl->get_Id(This,id) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_get_Title(This,title) \
    ( (This)->lpVtbl->get_Title(This,title) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_get_Impact(This,impact) \
    ( (This)->lpVtbl->get_Impact(This,impact) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_get_LinkTitle(This,linkTitle) \
    ( (This)->lpVtbl->get_LinkTitle(This,linkTitle) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_get_LinkUri(This,linkUri) \
    ( (This)->lpVtbl->get_LinkUri(This,linkUri) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_get_Categories(This,categories) \
    ( (This)->lpVtbl->get_Categories(This,categories) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_FormatString(This,resourceString,result) \
    ( (This)->lpVtbl->FormatString(This,resourceString,result) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_add_Triggered(This,handler,token) \
    ( (This)->lpVtbl->add_Triggered(This,handler,token) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_remove_Triggered(This,token) \
    ( (This)->lpVtbl->remove_Triggered(This,token) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRule_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IEtwEventRecord
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.EtwEventRecord
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IEtwEventRecord[] = L"Microsoft.Diagnostics.AppAnalysis.IEtwEventRecord";
/* [object, exclusiveto, uuid("79380594-6095-49C8-82E6-65DDC099DC59"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Timestamp )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [retval, out] */__RPC__out INT64 * timestamp
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ThreadId )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [retval, out] */__RPC__out UINT32 * threadId
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_EventId )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [retval, out] */__RPC__out UINT16 * eventId
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_EventVersion )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [retval, out] */__RPC__out BYTE * eventVersion
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ProviderId )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [retval, out] */__RPC__out GUID * providerId
        );
    HRESULT ( STDMETHODCALLTYPE *GetBooleanProperty )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out boolean * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetFloatProperty )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out float * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetDoubleProperty )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out double * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetInt32Property )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out INT32 * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetUInt32Property )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out UINT32 * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetInt64Property )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out INT64 * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetUInt64Property )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__out UINT64 * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetUnicodeStringProperty )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord * This,
        /* [in] */__RPC__in HSTRING propertyName,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecordVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_get_Timestamp(This,timestamp) \
    ( (This)->lpVtbl->get_Timestamp(This,timestamp) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_get_ThreadId(This,threadId) \
    ( (This)->lpVtbl->get_ThreadId(This,threadId) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_get_EventId(This,eventId) \
    ( (This)->lpVtbl->get_EventId(This,eventId) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_get_EventVersion(This,eventVersion) \
    ( (This)->lpVtbl->get_EventVersion(This,eventVersion) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_get_ProviderId(This,providerId) \
    ( (This)->lpVtbl->get_ProviderId(This,providerId) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetBooleanProperty(This,propertyName,value) \
    ( (This)->lpVtbl->GetBooleanProperty(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetFloatProperty(This,propertyName,value) \
    ( (This)->lpVtbl->GetFloatProperty(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetDoubleProperty(This,propertyName,value) \
    ( (This)->lpVtbl->GetDoubleProperty(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetInt32Property(This,propertyName,value) \
    ( (This)->lpVtbl->GetInt32Property(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetUInt32Property(This,propertyName,value) \
    ( (This)->lpVtbl->GetUInt32Property(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetInt64Property(This,propertyName,value) \
    ( (This)->lpVtbl->GetInt64Property(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetUInt64Property(This,propertyName,value) \
    ( (This)->lpVtbl->GetUInt64Property(This,propertyName,value) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_GetUnicodeStringProperty(This,propertyName,value) \
    ( (This)->lpVtbl->GetUnicodeStringProperty(This,propertyName,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIEtwEventRecord_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgs
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IRuleTriggeredEventArgs[] = L"Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgs";
/* [object, exclusiveto, uuid("E60D2D49-19D8-4614-A526-5BC97EFA72CB"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_TimelineStart )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out INT64 * timelineStart
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_TimelineStop )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out INT64 * timelineStop
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_FileName )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * fileName
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_LineNumber )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out UINT32 * lineNumber
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ColumnNumber )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out UINT32 * columnNumber
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ElementId )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out UINT64 * elementId
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_MeasurementUnit )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurementUnit * measurementUnit
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_MeasurementValue )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__out double * measurementValue
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Description )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * * description
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Solution )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceStringView * * solution
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_FileHash )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * hash
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_TimelineStart(This,timelineStart) \
    ( (This)->lpVtbl->get_TimelineStart(This,timelineStart) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_TimelineStop(This,timelineStop) \
    ( (This)->lpVtbl->get_TimelineStop(This,timelineStop) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_FileName(This,fileName) \
    ( (This)->lpVtbl->get_FileName(This,fileName) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_LineNumber(This,lineNumber) \
    ( (This)->lpVtbl->get_LineNumber(This,lineNumber) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_ColumnNumber(This,columnNumber) \
    ( (This)->lpVtbl->get_ColumnNumber(This,columnNumber) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_ElementId(This,elementId) \
    ( (This)->lpVtbl->get_ElementId(This,elementId) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_MeasurementUnit(This,measurementUnit) \
    ( (This)->lpVtbl->get_MeasurementUnit(This,measurementUnit) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_MeasurementValue(This,measurementValue) \
    ( (This)->lpVtbl->get_MeasurementValue(This,measurementValue) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_Description(This,description) \
    ( (This)->lpVtbl->get_Description(This,description) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_Solution(This,solution) \
    ( (This)->lpVtbl->get_Solution(This,solution) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_get_FileHash(This,hash) \
    ( (This)->lpVtbl->get_FileHash(This,hash) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs_INTERFACE_DEFINED__) */


/*
 *
 * Interface Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgsFactory
 *
 * Interface is a part of the implementation of type Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs
 *
 *
 */
#if !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Microsoft_Diagnostics_AppAnalysis_IRuleTriggeredEventArgsFactory[] = L"Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgsFactory";
/* [object, exclusiveto, uuid("B6090E2B-D71F-42EE-80DA-3393DA4038FD"), version] */
typedef struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactoryVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateInstance )(
        __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory * This,
        /* [in] */UINT64 elementId,
        /* [in] */__x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CTimelineInfo timelineInfo,
        /* [in] */__x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CMeasurement measurement,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * description,
        /* [in] */__RPC__in_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIResourceString * solution,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgs * * instance
        );
    END_INTERFACE
    
} __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactoryVtbl;

interface __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory
{
    CONST_VTBL struct __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactoryVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_CreateInstance(This,elementId,timelineInfo,measurement,description,solution,instance) \
    ( (This)->lpVtbl->CreateInstance(This,elementId,timelineInfo,measurement,description,solution,instance) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory;
#endif /* !defined(____x_ABI_CMicrosoft_CDiagnostics_CAppAnalysis_CIRuleTriggeredEventArgsFactory_INTERFACE_DEFINED__) */


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IRuleTriggeredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_RuleTriggeredEventArgs_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_RuleTriggeredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_RuleTriggeredEventArgs[] = L"Microsoft.Diagnostics.AppAnalysis.RuleTriggeredEventArgs";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwEventWatcher
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwEventWatcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventWatcher_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwEventWatcher[] = L"Microsoft.Diagnostics.AppAnalysis.EtwEventWatcher";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwRuleSet
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView_1_Microsoft.Diagnostics.AppAnalysis.EtwRule ** Default Interface **
 *    Windows.Foundation.Collections.IIterable_1_Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRuleSet_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRuleSet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwRuleSet[] = L"Microsoft.Diagnostics.AppAnalysis.EtwRuleSet";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwEvent
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwEvent ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEvent_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEvent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwEvent[] = L"Microsoft.Diagnostics.AppAnalysis.EtwEvent";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwProvider
 *
 * RuntimeClass contains static methods.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwProvider_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwProvider[] = L"Microsoft.Diagnostics.AppAnalysis.EtwProvider";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwEventRecord
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwEventRecord ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventRecord_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwEventRecord_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwEventRecord[] = L"Microsoft.Diagnostics.AppAnalysis.EtwEventRecord";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.EtwRule
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IEtwRule ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRule_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_EtwRule_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_EtwRule[] = L"Microsoft.Diagnostics.AppAnalysis.EtwRule";
#endif


/*
 *
 * Class Microsoft.Diagnostics.AppAnalysis.ResourceString
 *
 * RuntimeClass can be activated.
 *
 * Class implements the following interfaces:
 *    Microsoft.Diagnostics.AppAnalysis.IResourceString ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */

#ifndef RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_ResourceString_DEFINED
#define RUNTIMECLASS_Microsoft_Diagnostics_AppAnalysis_ResourceString_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Microsoft_Diagnostics_AppAnalysis_ResourceString[] = L"Microsoft.Diagnostics.AppAnalysis.ResourceString";
#endif



#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
#endif // __microsoft2Ediagnostics2Eappanalysis_p_h__

#endif // __microsoft2Ediagnostics2Eappanalysis_h__
