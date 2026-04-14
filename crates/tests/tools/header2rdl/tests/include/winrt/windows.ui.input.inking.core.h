
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
#ifndef __windows2Eui2Einput2Einking2Ecore_h__
#define __windows2Eui2Einput2Einking2Ecore_h__
#ifndef __windows2Eui2Einput2Einking2Ecore_p_h__
#define __windows2Eui2Einput2Einking2Ecore_p_h__


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

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.UI.Composition.h"
#include "Windows.UI.Core.h"
#include "Windows.UI.Input.Inking.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreIncrementalInkStroke;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke ABI::Windows::UI::Input::Inking::Core::ICoreIncrementalInkStroke

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreIncrementalInkStrokeFactory;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory ABI::Windows::UI::Input::Inking::Core::ICoreIncrementalInkStrokeFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreInkIndependentInputSource;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource ABI::Windows::UI::Input::Inking::Core::ICoreInkIndependentInputSource

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreInkIndependentInputSource2;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2 ABI::Windows::UI::Input::Inking::Core::ICoreInkIndependentInputSource2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreInkIndependentInputSourceStatics;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics ABI::Windows::UI::Input::Inking::Core::ICoreInkIndependentInputSourceStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreInkPresenterHost;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost ABI::Windows::UI::Input::Inking::Core::ICoreInkPresenterHost

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreWetStrokeUpdateEventArgs;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs ABI::Windows::UI::Input::Inking::Core::ICoreWetStrokeUpdateEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreWetStrokeUpdateSource;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource ABI::Windows::UI::Input::Inking::Core::ICoreWetStrokeUpdateSource

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        interface ICoreWetStrokeUpdateSourceStatics;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics ABI::Windows::UI::Input::Inking::Core::ICoreWetStrokeUpdateSourceStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkPoint;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPoint;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint ABI::Windows::UI::Input::Inking::IInkPoint

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("47415452-db79-567e-84d5-e9912330f944"))
IIterator<ABI::Windows::UI::Input::Inking::InkPoint*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkPoint*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0630c0ef-a4e2-5af6-b2e9-8e042e294e17"))
IIterable<ABI::Windows::UI::Input::Inking::InkPoint*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkPoint*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d1ac414b-c87d-540f-8ab1-4e0d09d9d283"))
IVectorView<ABI::Windows::UI::Input::Inking::InkPoint*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::InkPoint*> __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("10c47202-47ab-58bc-91de-d5000f1a74c0"))
IVector<ABI::Windows::UI::Input::Inking::InkPoint*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Input::Inking::InkPoint*> __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        class CoreInkIndependentInputSource;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class PointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IPointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs ABI::Windows::UI::Core::IPointerEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b83fbe98-882a-5b69-bd1c-c66690707fef"))
ITypedEventHandler<ABI::Windows::UI::Input::Inking::Core::CoreInkIndependentInputSource*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::Core::CoreInkIndependentInputSource*, ABI::Windows::UI::Input::Inking::Core::ICoreInkIndependentInputSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Inking::Core::CoreInkIndependentInputSource*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        class CoreWetStrokeUpdateSource;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        class CoreWetStrokeUpdateEventArgs;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("67ff75e8-02a4-5254-a965-0b254c7d0788"))
ITypedEventHandler<ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeUpdateSource*, ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeUpdateEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeUpdateSource*, ABI::Windows::UI::Input::Inking::Core::ICoreWetStrokeUpdateSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeUpdateEventArgs*, ABI::Windows::UI::Input::Inking::Core::ICoreWetStrokeUpdateEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource, Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeUpdateSource*, ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeUpdateEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Matrix3x2 Matrix3x2;
            } /* Numerics */
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
        namespace UI {
            namespace Composition {
                class ContainerVisual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIContainerVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIContainerVisual_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface IContainerVisual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CIContainerVisual ABI::Windows::UI::Composition::IContainerVisual

#endif // ____x_ABI_CWindows_CUI_CComposition_CIContainerVisual_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreCursor;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreCursor;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreCursor ABI::Windows::UI::Core::ICoreCursor

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkDrawingAttributes;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributes;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes ABI::Windows::UI::Input::Inking::IInkDrawingAttributes

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkPresenter;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenter;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter ABI::Windows::UI::Input::Inking::IInkPresenter

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStroke;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStroke;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke ABI::Windows::UI::Input::Inking::IInkStroke

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        typedef enum CoreWetStrokeDisposition : int CoreWetStrokeDisposition;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        class CoreIncrementalInkStroke;
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        enum CoreWetStrokeDisposition : int
                        {
                            CoreWetStrokeDisposition_Inking = 0,
                            CoreWetStrokeDisposition_Completed = 1,
                            CoreWetStrokeDisposition_Canceled = 2,
                        };
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreIncrementalInkStroke[] = L"Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("fda015d3-9d66-4f7d-a57f-cc70b9cfaa76")
                        ICoreIncrementalInkStroke : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE AppendInkPoints(
                                __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* inkPoints,
                                ABI::Windows::Foundation::Rect* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateInkStroke(
                                ABI::Windows::UI::Input::Inking::IInkStroke** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DrawingAttributes(
                                ABI::Windows::UI::Input::Inking::IInkDrawingAttributes** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PointTransform(
                                ABI::Windows::Foundation::Numerics::Matrix3x2* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_BoundingRect(
                                ABI::Windows::Foundation::Rect* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreIncrementalInkStroke = __uuidof(ICoreIncrementalInkStroke);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreIncrementalInkStrokeFactory[] = L"Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("d7c59f46-8da8-4f70-9751-e53bb6df4596")
                        ICoreIncrementalInkStrokeFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::UI::Input::Inking::IInkDrawingAttributes* drawingAttributes,
                                ABI::Windows::Foundation::Numerics::Matrix3x2 pointTransform,
                                ABI::Windows::UI::Input::Inking::Core::ICoreIncrementalInkStroke** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreIncrementalInkStrokeFactory = __uuidof(ICoreIncrementalInkStrokeFactory);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkIndependentInputSource[] = L"Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("39b38da9-7639-4499-a5b5-191d00e35b16")
                        ICoreInkIndependentInputSource : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE add_PointerEntering(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerEntering(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_PointerHovering(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerHovering(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_PointerExiting(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerExiting(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_PointerPressing(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerPressing(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_PointerMoving(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerMoving(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_PointerReleasing(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerReleasing(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_PointerLost(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_PointerLost(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InkPresenter(
                                ABI::Windows::UI::Input::Inking::IInkPresenter** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreInkIndependentInputSource = __uuidof(ICoreInkIndependentInputSource);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkIndependentInputSource2[] = L"Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("2846b012-0b59-5bb9-a3c5-becb7cf03a33")
                        ICoreInkIndependentInputSource2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PointerCursor(
                                ABI::Windows::UI::Core::ICoreCursor** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PointerCursor(
                                ABI::Windows::UI::Core::ICoreCursor* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreInkIndependentInputSource2 = __uuidof(ICoreInkIndependentInputSource2);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkIndependentInputSourceStatics[] = L"Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("73e6011b-80c0-4dfb-9b66-10ba7f3f9c84")
                        ICoreInkIndependentInputSourceStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::UI::Input::Inking::IInkPresenter* inkPresenter,
                                ABI::Windows::UI::Input::Inking::Core::ICoreInkIndependentInputSource** inkIndependentInputSource
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreInkIndependentInputSourceStatics = __uuidof(ICoreInkIndependentInputSourceStatics);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkPresenterHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkPresenterHost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkPresenterHost[] = L"Windows.UI.Input.Inking.Core.ICoreInkPresenterHost";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("396e89e6-7d55-4617-9e58-68c70c9169b9")
                        ICoreInkPresenterHost : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_InkPresenter(
                                ABI::Windows::UI::Input::Inking::IInkPresenter** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RootVisual(
                                ABI::Windows::UI::Composition::IContainerVisual** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_RootVisual(
                                ABI::Windows::UI::Composition::IContainerVisual* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreInkPresenterHost = __uuidof(ICoreInkPresenterHost);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreWetStrokeUpdateEventArgs[] = L"Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("fb07d14c-3380-457a-a987-991357896c1b")
                        ICoreWetStrokeUpdateEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_NewInkPoints(
                                __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PointerId(
                                UINT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Disposition(
                                ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeDisposition* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Disposition(
                                ABI::Windows::UI::Input::Inking::Core::CoreWetStrokeDisposition value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreWetStrokeUpdateEventArgs = __uuidof(ICoreWetStrokeUpdateEventArgs);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreWetStrokeUpdateSource[] = L"Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("1f718e22-ee52-4e00-8209-4c3e5b21a3cc")
                        ICoreWetStrokeUpdateSource : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE add_WetStrokeStarting(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_WetStrokeStarting(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_WetStrokeContinuing(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_WetStrokeContinuing(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_WetStrokeStopping(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_WetStrokeStopping(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_WetStrokeCompleted(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_WetStrokeCompleted(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_WetStrokeCanceled(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
                                EventRegistrationToken* cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_WetStrokeCanceled(
                                EventRegistrationToken cookie
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InkPresenter(
                                ABI::Windows::UI::Input::Inking::IInkPresenter** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreWetStrokeUpdateSource = __uuidof(ICoreWetStrokeUpdateSource);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreWetStrokeUpdateSourceStatics[] = L"Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Core {
                        MIDL_INTERFACE("3dad9cba-1d3d-46ae-ab9d-8647486c6f90")
                        ICoreWetStrokeUpdateSourceStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::UI::Input::Inking::IInkPresenter* inkPresenter,
                                ABI::Windows::UI::Input::Inking::Core::ICoreWetStrokeUpdateSource** WetStrokeUpdateSource
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ICoreWetStrokeUpdateSourceStatics = __uuidof(ICoreWetStrokeUpdateSourceStatics);
                    } /* Core */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreIncrementalInkStroke_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreIncrementalInkStroke_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreIncrementalInkStroke[] = L"Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource ** Default Interface **
 *    Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkIndependentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkIndependentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreInkIndependentInputSource[] = L"Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreInkPresenterHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreInkPresenterHost ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkPresenterHost_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkPresenterHost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreInkPresenterHost[] = L"Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateEventArgs[] = L"Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateSource[] = L"Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2 __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIVector_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* sender,
        __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIContainerVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIContainerVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIContainerVisual __x_ABI_CWindows_CUI_CComposition_CIContainerVisual;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIContainerVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreCursor __x_ABI_CWindows_CUI_CCore_CICoreCursor;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CCore_CCoreWetStrokeDisposition __x_ABI_CWindows_CUI_CInput_CInking_CCore_CCoreWetStrokeDisposition;

/*
 *
 * Struct Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CInking_CCore_CCoreWetStrokeDisposition
{
    CoreWetStrokeDisposition_Inking = 0,
    CoreWetStrokeDisposition_Completed = 1,
    CoreWetStrokeDisposition_Canceled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreIncrementalInkStroke[] = L"Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AppendInkPoints)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* inkPoints,
        struct __x_ABI_CWindows_CFoundation_CRect* result);
    HRESULT (STDMETHODCALLTYPE* CreateInkStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** result);
    HRESULT (STDMETHODCALLTYPE* get_DrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes** value);
    HRESULT (STDMETHODCALLTYPE* get_PointTransform)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2* value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRect)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_AppendInkPoints(This, inkPoints, result) \
    ((This)->lpVtbl->AppendInkPoints(This, inkPoints, result))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_CreateInkStroke(This, result) \
    ((This)->lpVtbl->CreateInkStroke(This, result))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_get_DrawingAttributes(This, value) \
    ((This)->lpVtbl->get_DrawingAttributes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_get_PointTransform(This, value) \
    ((This)->lpVtbl->get_PointTransform(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_get_BoundingRect(This, value) \
    ((This)->lpVtbl->get_BoundingRect(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreIncrementalInkStrokeFactory[] = L"Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* drawingAttributes,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 pointTransform,
        __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStroke** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_Create(This, drawingAttributes, pointTransform, result) \
    ((This)->lpVtbl->Create(This, drawingAttributes, pointTransform, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreIncrementalInkStrokeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkIndependentInputSource[] = L"Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PointerEntering)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerEntering)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerHovering)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerHovering)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerExiting)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerExiting)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerPressing)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerPressing)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerMoving)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerMoving)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerReleasing)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerReleasing)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerLost)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreInkIndependentInputSource_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerLost)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_InkPresenter)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerEntering(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerEntering(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerEntering(This, cookie) \
    ((This)->lpVtbl->remove_PointerEntering(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerHovering(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerHovering(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerHovering(This, cookie) \
    ((This)->lpVtbl->remove_PointerHovering(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerExiting(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerExiting(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerExiting(This, cookie) \
    ((This)->lpVtbl->remove_PointerExiting(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerPressing(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerPressing(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerPressing(This, cookie) \
    ((This)->lpVtbl->remove_PointerPressing(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerMoving(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerMoving(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerMoving(This, cookie) \
    ((This)->lpVtbl->remove_PointerMoving(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerReleasing(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerReleasing(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerReleasing(This, cookie) \
    ((This)->lpVtbl->remove_PointerReleasing(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_add_PointerLost(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerLost(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_remove_PointerLost(This, cookie) \
    ((This)->lpVtbl->remove_PointerLost(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_get_InkPresenter(This, value) \
    ((This)->lpVtbl->get_InkPresenter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkIndependentInputSource2[] = L"Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerCursor)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor** value);
    HRESULT (STDMETHODCALLTYPE* put_PointerCursor)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2* This,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_get_PointerCursor(This, value) \
    ((This)->lpVtbl->get_PointerCursor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_put_PointerCursor(This, value) \
    ((This)->lpVtbl->put_PointerCursor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkIndependentInputSourceStatics[] = L"Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* inkPresenter,
        __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSource** inkIndependentInputSource);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_Create(This, inkPresenter, inkIndependentInputSource) \
    ((This)->lpVtbl->Create(This, inkPresenter, inkIndependentInputSource))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkIndependentInputSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreInkPresenterHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreInkPresenterHost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreInkPresenterHost[] = L"Windows.UI.Input.Inking.Core.ICoreInkPresenterHost";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHostVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InkPresenter)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter** value);
    HRESULT (STDMETHODCALLTYPE* get_RootVisual)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        __x_ABI_CWindows_CUI_CComposition_CIContainerVisual** value);
    HRESULT (STDMETHODCALLTYPE* put_RootVisual)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost* This,
        __x_ABI_CWindows_CUI_CComposition_CIContainerVisual* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHostVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHostVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_get_InkPresenter(This, value) \
    ((This)->lpVtbl->get_InkPresenter(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_get_RootVisual(This, value) \
    ((This)->lpVtbl->get_RootVisual(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_put_RootVisual(This, value) \
    ((This)->lpVtbl->put_RootVisual(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreInkPresenterHost_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreWetStrokeUpdateEventArgs[] = L"Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NewInkPoints)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        __FIVector_1_Windows__CUI__CInput__CInking__CInkPoint** value);
    HRESULT (STDMETHODCALLTYPE* get_PointerId)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Disposition)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CCore_CCoreWetStrokeDisposition* value);
    HRESULT (STDMETHODCALLTYPE* put_Disposition)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CCore_CCoreWetStrokeDisposition value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_get_NewInkPoints(This, value) \
    ((This)->lpVtbl->get_NewInkPoints(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_get_PointerId(This, value) \
    ((This)->lpVtbl->get_PointerId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_get_Disposition(This, value) \
    ((This)->lpVtbl->get_Disposition(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_put_Disposition(This, value) \
    ((This)->lpVtbl->put_Disposition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreWetStrokeUpdateSource[] = L"Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_WetStrokeStarting)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_WetStrokeStarting)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_WetStrokeContinuing)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_WetStrokeContinuing)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_WetStrokeStopping)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_WetStrokeStopping)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_WetStrokeCompleted)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_WetStrokeCompleted)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_WetStrokeCanceled)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateSource_Windows__CUI__CInput__CInking__CCore__CCoreWetStrokeUpdateEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_WetStrokeCanceled)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_InkPresenter)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_add_WetStrokeStarting(This, handler, cookie) \
    ((This)->lpVtbl->add_WetStrokeStarting(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_remove_WetStrokeStarting(This, cookie) \
    ((This)->lpVtbl->remove_WetStrokeStarting(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_add_WetStrokeContinuing(This, handler, cookie) \
    ((This)->lpVtbl->add_WetStrokeContinuing(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_remove_WetStrokeContinuing(This, cookie) \
    ((This)->lpVtbl->remove_WetStrokeContinuing(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_add_WetStrokeStopping(This, handler, cookie) \
    ((This)->lpVtbl->add_WetStrokeStopping(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_remove_WetStrokeStopping(This, cookie) \
    ((This)->lpVtbl->remove_WetStrokeStopping(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_add_WetStrokeCompleted(This, handler, cookie) \
    ((This)->lpVtbl->add_WetStrokeCompleted(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_remove_WetStrokeCompleted(This, cookie) \
    ((This)->lpVtbl->remove_WetStrokeCompleted(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_add_WetStrokeCanceled(This, handler, cookie) \
    ((This)->lpVtbl->add_WetStrokeCanceled(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_remove_WetStrokeCanceled(This, cookie) \
    ((This)->lpVtbl->remove_WetStrokeCanceled(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_get_InkPresenter(This, value) \
    ((This)->lpVtbl->get_InkPresenter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Core_ICoreWetStrokeUpdateSourceStatics[] = L"Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* inkPresenter,
        __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSource** WetStrokeUpdateSource);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_Create(This, inkPresenter, WetStrokeUpdateSource) \
    ((This)->lpVtbl->Create(This, inkPresenter, WetStrokeUpdateSource))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CCore_CICoreWetStrokeUpdateSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreIncrementalInkStroke_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreIncrementalInkStroke_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreIncrementalInkStroke[] = L"Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource ** Default Interface **
 *    Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkIndependentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkIndependentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreInkIndependentInputSource[] = L"Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreInkPresenterHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreInkPresenterHost ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkPresenterHost_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreInkPresenterHost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreInkPresenterHost[] = L"Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateEventArgs[] = L"Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Core_CoreWetStrokeUpdateSource[] = L"Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
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
#endif // __windows2Eui2Einput2Einking2Ecore_p_h__

#endif // __windows2Eui2Einput2Einking2Ecore_h__
