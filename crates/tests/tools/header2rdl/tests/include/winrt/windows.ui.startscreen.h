
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
#ifndef __windows2Eui2Estartscreen_h__
#define __windows2Eui2Estartscreen_h__
#ifndef __windows2Eui2Estartscreen_p_h__
#define __windows2Eui2Estartscreen_p_h__


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
#include "Windows.ApplicationModel.Core.h"
#include "Windows.Perception.Spatial.h"
#include "Windows.System.h"
#include "Windows.UI.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IJumpList;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList ABI::Windows::UI::StartScreen::IJumpList

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IJumpListItem;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem ABI::Windows::UI::StartScreen::IJumpListItem

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IJumpListItemStatics;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics ABI::Windows::UI::StartScreen::IJumpListItemStatics

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IJumpListStatics;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics ABI::Windows::UI::StartScreen::IJumpListStatics

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTile;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile ABI::Windows::UI::StartScreen::ISecondaryTile

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTile2;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2 ABI::Windows::UI::StartScreen::ISecondaryTile2

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileFactory;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory ABI::Windows::UI::StartScreen::ISecondaryTileFactory

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileFactory2;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2 ABI::Windows::UI::StartScreen::ISecondaryTileFactory2

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileStatics;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics ABI::Windows::UI::StartScreen::ISecondaryTileStatics

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileVisualElements;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileVisualElements2;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2 ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements2

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileVisualElements3;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3 ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements3

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ISecondaryTileVisualElements4;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4 ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements4

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IStartScreenManager;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager ABI::Windows::UI::StartScreen::IStartScreenManager

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IStartScreenManager2;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2 ABI::Windows::UI::StartScreen::IStartScreenManager2

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IStartScreenManagerStatics;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics ABI::Windows::UI::StartScreen::IStartScreenManagerStatics

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ITileMixedRealityModel;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel ABI::Windows::UI::StartScreen::ITileMixedRealityModel

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface ITileMixedRealityModel2;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2 ABI::Windows::UI::StartScreen::ITileMixedRealityModel2

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IVisualElementsRequest;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest ABI::Windows::UI::StartScreen::IVisualElementsRequest

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IVisualElementsRequestDeferral;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral ABI::Windows::UI::StartScreen::IVisualElementsRequestDeferral

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                interface IVisualElementsRequestedEventArgs;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs ABI::Windows::UI::StartScreen::IVisualElementsRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */


namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class SecondaryTile;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#define DEF___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("391f7579-a90e-5352-9d01-fda995d7912f"))
IIterator<ABI::Windows::UI::StartScreen::SecondaryTile*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTile*, ABI::Windows::UI::StartScreen::ISecondaryTile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.StartScreen.SecondaryTile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::StartScreen::SecondaryTile*> __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_t;
#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#define DEF___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("75651af0-014a-5593-bc48-836ba3d1d5d4"))
IIterable<ABI::Windows::UI::StartScreen::SecondaryTile*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTile*, ABI::Windows::UI::StartScreen::ISecondaryTile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.StartScreen.SecondaryTile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::StartScreen::SecondaryTile*> __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_t;
#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#define DEF___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("16f89727-d811-5051-9ab5-0cb86a0f0ac3"))
IVectorView<ABI::Windows::UI::StartScreen::SecondaryTile*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTile*, ABI::Windows::UI::StartScreen::ISecondaryTile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.StartScreen.SecondaryTile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::StartScreen::SecondaryTile*> __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_t;
#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f72d55b2-b004-5e35-b5c4-22e87619b30a"))
IAsyncOperation<__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.StartScreen.SecondaryTile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile*> __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b9d6d973-a089-550a-83b7-f659ea0dea04"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.StartScreen.SecondaryTile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class JumpList;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1c008c58-733b-5b42-962a-b33328236cd3"))
IAsyncOperation<ABI::Windows::UI::StartScreen::JumpList*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::JumpList*, ABI::Windows::UI::StartScreen::IJumpList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.StartScreen.JumpList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::StartScreen::JumpList*> __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_t;
#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c047c6a-c55b-5485-b673-8d4bd7c342e2"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::StartScreen::JumpList*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::JumpList*, ABI::Windows::UI::StartScreen::IJumpList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.StartScreen.JumpList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::StartScreen::JumpList*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class JumpListItem;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#define DEF___FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f69f5cc4-004f-53eb-89e6-786e460588a4"))
IIterator<ABI::Windows::UI::StartScreen::JumpListItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::JumpListItem*, ABI::Windows::UI::StartScreen::IJumpListItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.StartScreen.JumpListItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::StartScreen::JumpListItem*> __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_t;
#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#define DEF___FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("130a7274-1afb-5c10-abea-61d81692a496"))
IIterable<ABI::Windows::UI::StartScreen::JumpListItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::JumpListItem*, ABI::Windows::UI::StartScreen::IJumpListItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.StartScreen.JumpListItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::StartScreen::JumpListItem*> __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_t;
#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class SecondaryTileVisualElements;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE
#define DEF___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bbc6e16c-cace-5230-8804-2298375168ac"))
IIterator<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*, ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.StartScreen.SecondaryTileVisualElements>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*> __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_t;
#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE
#define DEF___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6ef7c354-f153-5b53-99c2-e045c78cce08"))
IIterable<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*, ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.StartScreen.SecondaryTileVisualElements>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*> __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_t;
#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#define DEF___FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("be418be9-ab72-56b0-b6d3-ec70ef11f663"))
IVectorView<ABI::Windows::UI::StartScreen::JumpListItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::JumpListItem*, ABI::Windows::UI::StartScreen::IJumpListItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.StartScreen.JumpListItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::StartScreen::JumpListItem*> __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_t;
#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE
#define DEF___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1cd2cc9b-a41c-5dc7-9d95-4cef69a293f4"))
IVectorView<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*, ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.StartScreen.SecondaryTileVisualElements>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::StartScreen::SecondaryTileVisualElements*> __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_t;
#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#define DEF___FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af8f3fb2-f179-5f0a-aa09-28942eedf625"))
IVector<ABI::Windows::UI::StartScreen::JumpListItem*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::JumpListItem*, ABI::Windows::UI::StartScreen::IJumpListItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.StartScreen.JumpListItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::StartScreen::JumpListItem*> __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_t;
#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingBox SpatialBoundingBox;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_USE
#define DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab3274d9-9b82-5396-bb00-d70c539796b3"))
IReference<struct ABI::Windows::Perception::Spatial::SpatialBoundingBox> : IReference_impl<struct ABI::Windows::Perception::Spatial::SpatialBoundingBox>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Perception.Spatial.SpatialBoundingBox>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Perception::Spatial::SpatialBoundingBox> __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_t;
#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox ABI::Windows::Foundation::__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class VisualElementsRequestedEventArgs;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("02d9f968-fa76-5d77-934b-665e7c3be7cf"))
ITypedEventHandler<ABI::Windows::UI::StartScreen::SecondaryTile*, ABI::Windows::UI::StartScreen::VisualElementsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::SecondaryTile*, ABI::Windows::UI::StartScreen::ISecondaryTile*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::StartScreen::VisualElementsRequestedEventArgs*, ABI::Windows::UI::StartScreen::IVisualElementsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.StartScreen.SecondaryTile, Windows.UI.StartScreen.VisualElementsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::StartScreen::SecondaryTile*, ABI::Windows::UI::StartScreen::VisualElementsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class AppListEntry;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IAppListEntry;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry ABI::Windows::ApplicationModel::Core::IAppListEntry

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                typedef enum Placement : int Placement;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                typedef enum ForegroundText : int ForegroundText;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                typedef enum JumpListItemKind : int JumpListItemKind;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                typedef enum JumpListSystemGroupKind : int JumpListSystemGroupKind;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                typedef enum TileMixedRealityModelActivationBehavior : int TileMixedRealityModelActivationBehavior;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                typedef enum TileOptions : unsigned int TileOptions;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                typedef enum TileSize : int TileSize;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class StartScreenManager;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class TileMixedRealityModel;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class VisualElementsRequest;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                class VisualElementsRequestDeferral;
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.StartScreen.ForegroundText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                enum ForegroundText : int
                {
                    ForegroundText_Dark = 0,
                    ForegroundText_Light = 1,
                };
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.StartScreen.JumpListItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                enum JumpListItemKind : int
                {
                    JumpListItemKind_Arguments = 0,
                    JumpListItemKind_Separator = 1,
                };
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.StartScreen.JumpListSystemGroupKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                enum JumpListSystemGroupKind : int
                {
                    JumpListSystemGroupKind_None = 0,
                    JumpListSystemGroupKind_Frequent = 1,
                    JumpListSystemGroupKind_Recent = 2,
                };
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.StartScreen.TileMixedRealityModelActivationBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                enum TileMixedRealityModelActivationBehavior : int
                {
                    TileMixedRealityModelActivationBehavior_Default = 0,
                    TileMixedRealityModelActivationBehavior_None = 1,
                };
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.UI.StartScreen.TileOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                enum TileOptions : unsigned int
                {
                    TileOptions_None
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileOptions.None may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 0,
                    TileOptions_ShowNameOnLogo
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileOptions.ShowNameOnLogo may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile.VisualElements.ShowNameOnSquare150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 0x1,
                    TileOptions_ShowNameOnWideLogo
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileOptions.ShowNameWideOnLogo may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile.VisualElements.ShowNameOnWide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 0x2,
                    TileOptions_CopyOnDeployment
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileOptions.CopyOnDeployment may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile.RoamingEnabled to control roaming behavior.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(TileOptions)
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.StartScreen.TileSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                enum TileSize : int
                {
                    TileSize_Default = 0,
                    TileSize_Square30x30
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSize.Square30x30 may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 1,
                    TileSize_Square70x70
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSize.Square70x70 may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 2,
                    TileSize_Square150x150 = 3,
                    TileSize_Wide310x150 = 4,
                    TileSize_Square310x310 = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileSize_Square71x71 = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileSize_Square44x44 = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpList
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpList[] = L"Windows.UI.StartScreen.IJumpList";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("b0234c3e-cd6f-4cb6-a611-61fd505f3ed1")
                IJumpList : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Items(
                        __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemGroupKind(
                        ABI::Windows::UI::StartScreen::JumpListSystemGroupKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SystemGroupKind(
                        ABI::Windows::UI::StartScreen::JumpListSystemGroupKind value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IJumpList = __uuidof(IJumpList);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpList;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpListItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpListItem[] = L"Windows.UI.StartScreen.IJumpListItem";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("7adb6717-8b5d-4820-995b-9b418dbe48b0")
                IJumpListItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::UI::StartScreen::JumpListItemKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemovedByUser(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Description(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GroupName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_GroupName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IJumpListItem = __uuidof(IJumpListItem);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpListItemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpListItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpListItemStatics[] = L"Windows.UI.StartScreen.IJumpListItemStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("f1bfc4e8-c7aa-49cb-8dde-ecfccd7ad7e4")
                IJumpListItemStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithArguments(
                        HSTRING arguments,
                        HSTRING displayName,
                        ABI::Windows::UI::StartScreen::IJumpListItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSeparator(
                        ABI::Windows::UI::StartScreen::IJumpListItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IJumpListItemStatics = __uuidof(IJumpListItemStatics);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpListStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpList
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpListStatics[] = L"Windows.UI.StartScreen.IJumpListStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("a7e0c681-e67e-4b74-8250-3f322c4d92c3")
                IJumpListStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadCurrentAsync(
                        __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IJumpListStatics = __uuidof(IJumpListStatics);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTile[] = L"Windows.UI.StartScreen.ISecondaryTile";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("9e9e51e0-2bb5-4bc0-bb8d-42b23abcc88d")
                ISecondaryTile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_TileId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TileId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Arguments(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ShortName may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayName.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_ShortName(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ShortName may be altered or unavailable for releases after Windows 8.1. Instead, use DisplayName.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_ShortName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Logo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Logo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmallLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square30x30Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_SmallLogo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmallLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square30x30Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SmallLogo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("WideLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Wide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_WideLogo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("WideLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Wide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_WideLogo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LockScreenBadgeLogo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LockScreenBadgeLogo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LockScreenDisplayBadgeAndTileText(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LockScreenDisplayBadgeAndTileText(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_TileOptions(
                        ABI::Windows::UI::StartScreen::TileOptions value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_TileOptions(
                        ABI::Windows::UI::StartScreen::TileOptions* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_ForegroundText(
                        ABI::Windows::UI::StartScreen::ForegroundText value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ForegroundText may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ForegroundText.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_ForegroundText(
                        ABI::Windows::UI::StartScreen::ForegroundText* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("BackgroundColor may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.BackgroundColor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("BackgroundColor may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.BackgroundColor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateAsyncWithPoint(
                        ABI::Windows::Foundation::Point invocationPoint,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateAsyncWithRect(
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateAsyncWithRectAndPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestDeleteAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestDeleteAsyncWithPoint(
                        ABI::Windows::Foundation::Point invocationPoint,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestDeleteAsyncWithRect(
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestDeleteAsyncWithRectAndPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTile = __uuidof(ISecondaryTile);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.StartScreen.ISecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTile2[] = L"Windows.UI.StartScreen.ISecondaryTile2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("b2f6cc35-3250-4990-923c-294ab4b694dd")
                ISecondaryTile2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PhoneticName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhoneticName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VisualElements(
                        ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoamingEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoamingEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_VisualElementsRequested(
                        __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_VisualElementsRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTile2 = __uuidof(ISecondaryTile2);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileFactory[] = L"Windows.UI.StartScreen.ISecondaryTileFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("57f52ca0-51bc-4abf-8ebf-627a0398b05a")
                ISecondaryTileFactory : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SecondaryTile(string, string, string, string, Windows.UI.StartScreen.TileOptions, Windows.Foundation.Uri) may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile(string, string, string, Windows.Foundation.Uri, Windows.UI.StartScreen.TileSize).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE CreateTile(
                        HSTRING tileId,
                        HSTRING shortName,
                        HSTRING displayName,
                        HSTRING arguments,
                        ABI::Windows::UI::StartScreen::TileOptions tileOptions,
                        ABI::Windows::Foundation::IUriRuntimeClass* logoReference,
                        ABI::Windows::UI::StartScreen::ISecondaryTile** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SecondaryTile(string, string, string, string, Windows.UI.StartScreen.TileOptions, Windows.Foundation.Uri, Windows.Foundation.Uri) may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile(string, string, string, Windows.Foundation.Uri, Windows.UI.StartScreen.TileSize).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE CreateWideTile(
                        HSTRING tileId,
                        HSTRING shortName,
                        HSTRING displayName,
                        HSTRING arguments,
                        ABI::Windows::UI::StartScreen::TileOptions tileOptions,
                        ABI::Windows::Foundation::IUriRuntimeClass* logoReference,
                        ABI::Windows::Foundation::IUriRuntimeClass* wideLogoReference,
                        ABI::Windows::UI::StartScreen::ISecondaryTile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithId(
                        HSTRING tileId,
                        ABI::Windows::UI::StartScreen::ISecondaryTile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileFactory = __uuidof(ISecondaryTileFactory);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.StartScreen.ISecondaryTileFactory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileFactory2[] = L"Windows.UI.StartScreen.ISecondaryTileFactory2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("274b8a3b-522d-448e-9eb2-d0672ab345c8")
                ISecondaryTileFactory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateMinimalTile(
                        HSTRING tileId,
                        HSTRING displayName,
                        HSTRING arguments,
                        ABI::Windows::Foundation::IUriRuntimeClass* square150x150Logo,
                        ABI::Windows::UI::StartScreen::TileSize desiredSize,
                        ABI::Windows::UI::StartScreen::ISecondaryTile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileFactory2 = __uuidof(ISecondaryTileFactory2);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileStatics[] = L"Windows.UI.StartScreen.ISecondaryTileStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("99908dae-d051-4676-87fe-9ec242d83c74")
                ISecondaryTileStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Exists(
                        HSTRING tileId,
                        boolean* exists
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllForApplicationAsync(
                        HSTRING applicationId,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllForPackageAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileStatics = __uuidof(ISecondaryTileStatics);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("1d8df333-815e-413f-9f50-a81da70a96b2")
                ISecondaryTileVisualElements : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SecondaryTileVisualElements.Square30x30Logo may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Square30x30Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SecondaryTileVisualElements.Square30x30Logo may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Square30x30Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SecondaryTileVisualElements.Square70x70Logo may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Square70x70Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SecondaryTileVisualElements.Square70x70Logo may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Square70x70Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Square150x150Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Square150x150Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Wide310x150Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wide310x150Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Square310x310Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Square310x310Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ForegroundText(
                        ABI::Windows::UI::StartScreen::ForegroundText value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForegroundText(
                        ABI::Windows::UI::StartScreen::ForegroundText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShowNameOnSquare150x150Logo(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShowNameOnSquare150x150Logo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShowNameOnWide310x150Logo(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShowNameOnWide310x150Logo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShowNameOnSquare310x310Logo(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShowNameOnSquare310x310Logo(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileVisualElements = __uuidof(ISecondaryTileVisualElements);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements2[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("fd2e31d0-57dc-4794-8ecf-5682f5f3e6ef")
                ISecondaryTileVisualElements2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Square71x71Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Square71x71Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileVisualElements2 = __uuidof(ISecondaryTileVisualElements2);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements3[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("56b55ad6-d15c-40f4-81e7-57ffd8f8a4e9")
                ISecondaryTileVisualElements3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Square44x44Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Square44x44Logo(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileVisualElements3 = __uuidof(ISecondaryTileVisualElements3);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements4[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("66566117-b544-40d2-8d12-74d4ec24d04c")
                ISecondaryTileVisualElements4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MixedRealityModel(
                        ABI::Windows::UI::StartScreen::ITileMixedRealityModel** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISecondaryTileVisualElements4 = __uuidof(ISecondaryTileVisualElements4);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.StartScreen.IStartScreenManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.StartScreenManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IStartScreenManager[] = L"Windows.UI.StartScreen.IStartScreenManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("4a1dcbcb-26e9-4eb4-8933-859eb6ecdb29")
                IStartScreenManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SupportsAppListEntry(
                        ABI::Windows::ApplicationModel::Core::IAppListEntry* appListEntry,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ContainsAppListEntryAsync(
                        ABI::Windows::ApplicationModel::Core::IAppListEntry* appListEntry,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAddAppListEntryAsync(
                        ABI::Windows::ApplicationModel::Core::IAppListEntry* appListEntry,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartScreenManager = __uuidof(IStartScreenManager);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.StartScreen.IStartScreenManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.StartScreenManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.StartScreen.IStartScreenManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IStartScreenManager2[] = L"Windows.UI.StartScreen.IStartScreenManager2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("08a716b6-316b-4ad9-acb8-fe9cf00bd608")
                IStartScreenManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ContainsSecondaryTileAsync(
                        HSTRING tileId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryRemoveSecondaryTileAsync(
                        HSTRING tileId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartScreenManager2 = __uuidof(IStartScreenManager2);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.StartScreen.IStartScreenManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.StartScreenManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IStartScreenManagerStatics[] = L"Windows.UI.StartScreen.IStartScreenManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("7865ef0f-b585-464e-8993-34e8f8738d48")
                IStartScreenManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::UI::StartScreen::IStartScreenManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::UI::StartScreen::IStartScreenManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartScreenManagerStatics = __uuidof(IStartScreenManagerStatics);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.StartScreen.ITileMixedRealityModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.TileMixedRealityModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ITileMixedRealityModel[] = L"Windows.UI.StartScreen.ITileMixedRealityModel";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("b0764e5b-887d-4242-9a19-3d0a4ea78031")
                ITileMixedRealityModel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BoundingBox(
                        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BoundingBox(
                        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileMixedRealityModel = __uuidof(ITileMixedRealityModel);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.StartScreen.ITileMixedRealityModel2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.TileMixedRealityModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ITileMixedRealityModel2[] = L"Windows.UI.StartScreen.ITileMixedRealityModel2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("439470b2-d7c5-410b-8319-9486a27b6c67")
                ITileMixedRealityModel2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_ActivationBehavior(
                        ABI::Windows::UI::StartScreen::TileMixedRealityModelActivationBehavior value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActivationBehavior(
                        ABI::Windows::UI::StartScreen::TileMixedRealityModelActivationBehavior* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileMixedRealityModel2 = __uuidof(ITileMixedRealityModel2);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.StartScreen.IVisualElementsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.VisualElementsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IVisualElementsRequest[] = L"Windows.UI.StartScreen.IVisualElementsRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("c138333a-9308-4072-88cc-d068db347c68")
                IVisualElementsRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VisualElements(
                        ABI::Windows::UI::StartScreen::ISecondaryTileVisualElements** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlternateVisualElements(
                        __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::UI::StartScreen::IVisualElementsRequestDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVisualElementsRequest = __uuidof(IVisualElementsRequest);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.IVisualElementsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.VisualElementsRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IVisualElementsRequestDeferral[] = L"Windows.UI.StartScreen.IVisualElementsRequestDeferral";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("a1656eb0-0126-4357-8204-bd82bb2a046d")
                IVisualElementsRequestDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IVisualElementsRequestDeferral = __uuidof(IVisualElementsRequestDeferral);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.IVisualElementsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.VisualElementsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IVisualElementsRequestedEventArgs[] = L"Windows.UI.StartScreen.IVisualElementsRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace StartScreen {
                MIDL_INTERFACE("7b6fc982-3a0d-4ece-af96-cd17e1b00b2d")
                IVisualElementsRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::UI::StartScreen::IVisualElementsRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVisualElementsRequestedEventArgs = __uuidof(IVisualElementsRequestedEventArgs);
            } /* StartScreen */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.JumpList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.IJumpListStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IJumpList ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_JumpList_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_JumpList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_JumpList[] = L"Windows.UI.StartScreen.JumpList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.StartScreen.JumpListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.IJumpListItemStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IJumpListItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_JumpListItem_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_JumpListItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_JumpListItem[] = L"Windows.UI.StartScreen.JumpListItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.StartScreen.SecondaryTile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.StartScreen.ISecondaryTileFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.StartScreen.ISecondaryTileFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.ISecondaryTileStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.ISecondaryTile ** Default Interface **
 *    Windows.UI.StartScreen.ISecondaryTile2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTile_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_SecondaryTile[] = L"Windows.UI.StartScreen.SecondaryTile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements ** Default Interface **
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements2
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements3
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTileVisualElements_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTileVisualElements_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_SecondaryTileVisualElements[] = L"Windows.UI.StartScreen.SecondaryTileVisualElements";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.StartScreenManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.IStartScreenManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IStartScreenManager ** Default Interface **
 *    Windows.UI.StartScreen.IStartScreenManager2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_StartScreenManager_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_StartScreenManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_StartScreenManager[] = L"Windows.UI.StartScreen.StartScreenManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.StartScreen.TileMixedRealityModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.ITileMixedRealityModel ** Default Interface **
 *    Windows.UI.StartScreen.ITileMixedRealityModel2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_TileMixedRealityModel_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_TileMixedRealityModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_TileMixedRealityModel[] = L"Windows.UI.StartScreen.TileMixedRealityModel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.StartScreen.VisualElementsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IVisualElementsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_VisualElementsRequest[] = L"Windows.UI.StartScreen.VisualElementsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.VisualElementsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IVisualElementsRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_VisualElementsRequestDeferral[] = L"Windows.UI.StartScreen.VisualElementsRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.VisualElementsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IVisualElementsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_VisualElementsRequestedEventArgs[] = L"Windows.UI.StartScreen.VisualElementsRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpList __x_ABI_CWindows_CUI_CStartScreen_CIJumpList;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2 __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2 __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2 __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3 __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4 __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2 __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2 __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile;

typedef struct __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl;

interface __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile;

typedef struct __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl;

interface __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

typedef struct __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl;

interface __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList;

typedef struct __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpList** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpListVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList* This,
        __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpListVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CStartScreen__CJumpList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem;

typedef struct __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItemVtbl;

interface __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem;

typedef struct __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        __FIIterator_1_Windows__CUI__CStartScreen__CJumpListItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItemVtbl;

interface __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements;

typedef struct __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl;

interface __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements;

typedef struct __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        __FIIterator_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl;

interface __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem;

typedef struct __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItemVtbl;

interface __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements;

typedef struct __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl;

interface __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElementsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CStartScreen__CJumpListItem;

typedef struct __FIVector_1_Windows__CUI__CStartScreen__CJumpListItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        __FIVectorView_1_Windows__CUI__CStartScreen__CJumpListItem** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CStartScreen__CJumpListItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CStartScreen__CJumpListItemVtbl;

interface __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CStartScreen__CJumpListItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CStartScreen__CJumpListItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox;

typedef struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* This,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingBox* result);

    END_INTERFACE
} __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBoxVtbl;

interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox
{
    CONST_VTBL struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* sender,
        __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

typedef enum __x_ABI_CWindows_CUI_CStartScreen_CForegroundText __x_ABI_CWindows_CUI_CStartScreen_CForegroundText;

typedef enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListItemKind __x_ABI_CWindows_CUI_CStartScreen_CJumpListItemKind;

typedef enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListSystemGroupKind __x_ABI_CWindows_CUI_CStartScreen_CJumpListSystemGroupKind;

typedef enum __x_ABI_CWindows_CUI_CStartScreen_CTileMixedRealityModelActivationBehavior __x_ABI_CWindows_CUI_CStartScreen_CTileMixedRealityModelActivationBehavior;

typedef enum __x_ABI_CWindows_CUI_CStartScreen_CTileOptions __x_ABI_CWindows_CUI_CStartScreen_CTileOptions;

typedef enum __x_ABI_CWindows_CUI_CStartScreen_CTileSize __x_ABI_CWindows_CUI_CStartScreen_CTileSize;

/*
 *
 * Struct Windows.UI.StartScreen.ForegroundText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CStartScreen_CForegroundText
{
    ForegroundText_Dark = 0,
    ForegroundText_Light = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.StartScreen.JumpListItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListItemKind
{
    JumpListItemKind_Arguments = 0,
    JumpListItemKind_Separator = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.StartScreen.JumpListSystemGroupKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListSystemGroupKind
{
    JumpListSystemGroupKind_None = 0,
    JumpListSystemGroupKind_Frequent = 1,
    JumpListSystemGroupKind_Recent = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.StartScreen.TileMixedRealityModelActivationBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CUI_CStartScreen_CTileMixedRealityModelActivationBehavior
{
    TileMixedRealityModelActivationBehavior_Default = 0,
    TileMixedRealityModelActivationBehavior_None = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.UI.StartScreen.TileOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CStartScreen_CTileOptions
{
    TileOptions_None
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileOptions.None may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 0,
    TileOptions_ShowNameOnLogo
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileOptions.ShowNameOnLogo may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile.VisualElements.ShowNameOnSquare150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 0x1,
    TileOptions_ShowNameOnWideLogo
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileOptions.ShowNameWideOnLogo may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile.VisualElements.ShowNameOnWide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 0x2,
    TileOptions_CopyOnDeployment
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileOptions.CopyOnDeployment may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile.RoamingEnabled to control roaming behavior.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.StartScreen.TileSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CStartScreen_CTileSize
{
    TileSize_Default = 0,
    TileSize_Square30x30
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSize.Square30x30 may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 1,
    TileSize_Square70x70
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSize.Square70x70 may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 2,
    TileSize_Square150x150 = 3,
    TileSize_Wide310x150 = 4,
    TileSize_Square310x310 = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileSize_Square71x71 = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileSize_Square44x44 = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpList
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpList[] = L"Windows.UI.StartScreen.IJumpList";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        __FIVector_1_Windows__CUI__CStartScreen__CJumpListItem** value);
    HRESULT (STDMETHODCALLTYPE* get_SystemGroupKind)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListSystemGroupKind* value);
    HRESULT (STDMETHODCALLTYPE* put_SystemGroupKind)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListSystemGroupKind value);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpList* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIJumpListVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpList
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_get_SystemGroupKind(This, value) \
    ((This)->lpVtbl->get_SystemGroupKind(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_put_SystemGroupKind(This, value) \
    ((This)->lpVtbl->put_SystemGroupKind(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpList_SaveAsync(This, result) \
    ((This)->lpVtbl->SaveAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpList;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpListItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpListItem[] = L"Windows.UI.StartScreen.IJumpListItem";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CJumpListItemKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemovedByUser)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_GroupName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_GroupName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Logo)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Logo)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_RemovedByUser(This, value) \
    ((This)->lpVtbl->get_RemovedByUser(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_GroupName(This, value) \
    ((This)->lpVtbl->get_GroupName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_put_GroupName(This, value) \
    ((This)->lpVtbl->put_GroupName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_get_Logo(This, value) \
    ((This)->lpVtbl->get_Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_put_Logo(This, value) \
    ((This)->lpVtbl->put_Logo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpListItemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpListItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpListItemStatics[] = L"Windows.UI.StartScreen.IJumpListItemStatics";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithArguments)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This,
        HSTRING arguments,
        HSTRING displayName,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** result);
    HRESULT (STDMETHODCALLTYPE* CreateSeparator)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStaticsVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_CreateWithArguments(This, arguments, displayName, result) \
    ((This)->lpVtbl->CreateWithArguments(This, arguments, displayName, result))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_CreateSeparator(This, result) \
    ((This)->lpVtbl->CreateSeparator(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListItemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.IJumpListStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.JumpList
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IJumpListStatics[] = L"Windows.UI.StartScreen.IJumpListStatics";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadCurrentAsync)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This,
        __FIAsyncOperation_1_Windows__CUI__CStartScreen__CJumpList** result);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStaticsVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_LoadCurrentAsync(This, result) \
    ((This)->lpVtbl->LoadCurrentAsync(This, result))

#define __x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIJumpListStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTile[] = L"Windows.UI.StartScreen.ISecondaryTile";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_TileId)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_TileId)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Arguments)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ShortName may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayName.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_ShortName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ShortName may be altered or unavailable for releases after Windows 8.1. Instead, use DisplayName.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ShortName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Logo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Logo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmallLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square30x30Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_SmallLogo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmallLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square30x30Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SmallLogo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("WideLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Wide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_WideLogo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("WideLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Wide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_WideLogo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_LockScreenBadgeLogo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_LockScreenBadgeLogo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_LockScreenDisplayBadgeAndTileText)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_LockScreenDisplayBadgeAndTileText)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_TileOptions)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileOptions value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_TileOptions)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileOptions* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_ForegroundText)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CForegroundText value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ForegroundText may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ForegroundText.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ForegroundText)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CForegroundText* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("BackgroundColor may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.BackgroundColor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CUI_CColor value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("BackgroundColor may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.BackgroundColor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* RequestCreateAsync)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestCreateAsyncWithPoint)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CFoundation_CPoint invocationPoint,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestCreateAsyncWithRect)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestCreateAsyncWithRectAndPlacement)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestDeleteAsync)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestDeleteAsyncWithPoint)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CFoundation_CPoint invocationPoint,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestDeleteAsyncWithRect)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestDeleteAsyncWithRectAndPlacement)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateAsync)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_TileId(This, value) \
    ((This)->lpVtbl->put_TileId(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_TileId(This, value) \
    ((This)->lpVtbl->get_TileId(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_Arguments(This, value) \
    ((This)->lpVtbl->put_Arguments(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ShortName may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayName.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_ShortName(This, value) \
    ((This)->lpVtbl->put_ShortName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ShortName may be altered or unavailable for releases after Windows 8.1. Instead, use DisplayName.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_ShortName(This, value) \
    ((This)->lpVtbl->get_ShortName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Logo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_Logo(This, value) \
    ((This)->lpVtbl->put_Logo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Logo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square150x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_Logo(This, value) \
    ((This)->lpVtbl->get_Logo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmallLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square30x30Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_SmallLogo(This, value) \
    ((This)->lpVtbl->put_SmallLogo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmallLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Square30x30Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_SmallLogo(This, value) \
    ((This)->lpVtbl->get_SmallLogo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("WideLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Wide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_WideLogo(This, value) \
    ((This)->lpVtbl->put_WideLogo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("WideLogo may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.Wide310x150Logo.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_WideLogo(This, value) \
    ((This)->lpVtbl->get_WideLogo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_LockScreenBadgeLogo(This, value) \
    ((This)->lpVtbl->put_LockScreenBadgeLogo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_LockScreenBadgeLogo(This, value) \
    ((This)->lpVtbl->get_LockScreenBadgeLogo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_LockScreenDisplayBadgeAndTileText(This, value) \
    ((This)->lpVtbl->put_LockScreenDisplayBadgeAndTileText(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_LockScreenDisplayBadgeAndTileText(This, value) \
    ((This)->lpVtbl->get_LockScreenDisplayBadgeAndTileText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_TileOptions(This, value) \
    ((This)->lpVtbl->put_TileOptions(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_TileOptions(This, value) \
    ((This)->lpVtbl->get_TileOptions(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TileOptions may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ShowNameOnSquare150x150Logo, VisualElements.ShowNameOnWide310x150Logo, and RoamingEnabled.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_ForegroundText(This, value) \
    ((This)->lpVtbl->put_ForegroundText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ForegroundText may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.ForegroundText.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_ForegroundText(This, value) \
    ((This)->lpVtbl->get_ForegroundText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("BackgroundColor may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.BackgroundColor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("BackgroundColor may be altered or unavailable for releases after Windows 8.1. Instead, use VisualElements.BackgroundColor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestCreateAsync(This, operation) \
    ((This)->lpVtbl->RequestCreateAsync(This, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestCreateAsyncWithPoint(This, invocationPoint, operation) \
    ((This)->lpVtbl->RequestCreateAsyncWithPoint(This, invocationPoint, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestCreateAsyncWithRect(This, selection, operation) \
    ((This)->lpVtbl->RequestCreateAsyncWithRect(This, selection, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestCreateAsyncWithRectAndPlacement(This, selection, preferredPlacement, operation) \
    ((This)->lpVtbl->RequestCreateAsyncWithRectAndPlacement(This, selection, preferredPlacement, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestDeleteAsync(This, operation) \
    ((This)->lpVtbl->RequestDeleteAsync(This, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestDeleteAsyncWithPoint(This, invocationPoint, operation) \
    ((This)->lpVtbl->RequestDeleteAsyncWithPoint(This, invocationPoint, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestDeleteAsyncWithRect(This, selection, operation) \
    ((This)->lpVtbl->RequestDeleteAsyncWithRect(This, selection, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_RequestDeleteAsyncWithRectAndPlacement(This, selection, preferredPlacement, operation) \
    ((This)->lpVtbl->RequestDeleteAsyncWithRectAndPlacement(This, selection, preferredPlacement, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_UpdateAsync(This, operation) \
    ((This)->lpVtbl->UpdateAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.StartScreen.ISecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTile2[] = L"Windows.UI.StartScreen.ISecondaryTile2";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PhoneticName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PhoneticName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VisualElements)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements** value);
    HRESULT (STDMETHODCALLTYPE* put_RoamingEnabled)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingEnabled)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_VisualElementsRequested)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        __FITypedEventHandler_2_Windows__CUI__CStartScreen__CSecondaryTile_Windows__CUI__CStartScreen__CVisualElementsRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_VisualElementsRequested)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_put_PhoneticName(This, value) \
    ((This)->lpVtbl->put_PhoneticName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_get_PhoneticName(This, value) \
    ((This)->lpVtbl->get_PhoneticName(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_get_VisualElements(This, value) \
    ((This)->lpVtbl->get_VisualElements(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_put_RoamingEnabled(This, value) \
    ((This)->lpVtbl->put_RoamingEnabled(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_get_RoamingEnabled(This, value) \
    ((This)->lpVtbl->get_RoamingEnabled(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_add_VisualElementsRequested(This, handler, token) \
    ((This)->lpVtbl->add_VisualElementsRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_remove_VisualElementsRequested(This, token) \
    ((This)->lpVtbl->remove_VisualElementsRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileFactory[] = L"Windows.UI.StartScreen.ISecondaryTileFactory";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTile(string, string, string, string, Windows.UI.StartScreen.TileOptions, Windows.Foundation.Uri) may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile(string, string, string, Windows.Foundation.Uri, Windows.UI.StartScreen.TileSize).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* CreateTile)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        HSTRING tileId,
        HSTRING shortName,
        HSTRING displayName,
        HSTRING arguments,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileOptions tileOptions,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* logoReference,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTile(string, string, string, string, Windows.UI.StartScreen.TileOptions, Windows.Foundation.Uri, Windows.Foundation.Uri) may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile(string, string, string, Windows.Foundation.Uri, Windows.UI.StartScreen.TileSize).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* CreateWideTile)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        HSTRING tileId,
        HSTRING shortName,
        HSTRING displayName,
        HSTRING arguments,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileOptions tileOptions,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* logoReference,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* wideLogoReference,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithId)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactoryVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTile(string, string, string, string, Windows.UI.StartScreen.TileOptions, Windows.Foundation.Uri) may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile(string, string, string, Windows.Foundation.Uri, Windows.UI.StartScreen.TileSize).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_CreateTile(This, tileId, shortName, displayName, arguments, tileOptions, logoReference, value) \
    ((This)->lpVtbl->CreateTile(This, tileId, shortName, displayName, arguments, tileOptions, logoReference, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTile(string, string, string, string, Windows.UI.StartScreen.TileOptions, Windows.Foundation.Uri, Windows.Foundation.Uri) may be altered or unavailable for releases after Windows Phone 8.1. Instead, use SecondaryTile(string, string, string, Windows.Foundation.Uri, Windows.UI.StartScreen.TileSize).")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_CreateWideTile(This, tileId, shortName, displayName, arguments, tileOptions, logoReference, wideLogoReference, value) \
    ((This)->lpVtbl->CreateWideTile(This, tileId, shortName, displayName, arguments, tileOptions, logoReference, wideLogoReference, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_CreateWithId(This, tileId, value) \
    ((This)->lpVtbl->CreateWithId(This, tileId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.StartScreen.ISecondaryTileFactory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileFactory2[] = L"Windows.UI.StartScreen.ISecondaryTileFactory2";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateMinimalTile)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2* This,
        HSTRING tileId,
        HSTRING displayName,
        HSTRING arguments,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* square150x150Logo,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileSize desiredSize,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTile** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_CreateMinimalTile(This, tileId, displayName, arguments, square150x150Logo, desiredSize, value) \
    ((This)->lpVtbl->CreateMinimalTile(This, tileId, displayName, arguments, square150x150Logo, desiredSize, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileStatics[] = L"Windows.UI.StartScreen.ISecondaryTileStatics";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Exists)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        HSTRING tileId,
        boolean* exists);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllForApplicationAsync)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        HSTRING applicationId,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllForPackageAsync)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStaticsVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_Exists(This, tileId, exists) \
    ((This)->lpVtbl->Exists(This, tileId, exists))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FindAllAsync(This, operation) \
    ((This)->lpVtbl->FindAllAsync(This, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FindAllForApplicationAsync(This, applicationId, operation) \
    ((This)->lpVtbl->FindAllForApplicationAsync(This, applicationId, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_FindAllForPackageAsync(This, operation) \
    ((This)->lpVtbl->FindAllForPackageAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElementsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square30x30Logo may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Square30x30Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square30x30Logo may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Square30x30Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square70x70Logo may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Square70x70Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square70x70Logo may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Square70x70Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Square150x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Square150x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Wide310x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Wide310x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Square310x310Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Square310x310Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ForegroundText)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CForegroundText value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundText)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CForegroundText* value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_ShowNameOnSquare150x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShowNameOnSquare150x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShowNameOnWide310x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShowNameOnWide310x150Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShowNameOnSquare310x310Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShowNameOnSquare310x310Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElementsVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElementsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square30x30Logo may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_Square30x30Logo(This, value) \
    ((This)->lpVtbl->put_Square30x30Logo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square30x30Logo may be altered or unavailable for release after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_Square30x30Logo(This, value) \
    ((This)->lpVtbl->get_Square30x30Logo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square70x70Logo may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_Square70x70Logo(This, value) \
    ((This)->lpVtbl->put_Square70x70Logo(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SecondaryTileVisualElements.Square70x70Logo may be altered or unavailable for release after Windows Phone 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_Square70x70Logo(This, value) \
    ((This)->lpVtbl->get_Square70x70Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_Square150x150Logo(This, value) \
    ((This)->lpVtbl->put_Square150x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_Square150x150Logo(This, value) \
    ((This)->lpVtbl->get_Square150x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_Wide310x150Logo(This, value) \
    ((This)->lpVtbl->put_Wide310x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_Wide310x150Logo(This, value) \
    ((This)->lpVtbl->get_Wide310x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_Square310x310Logo(This, value) \
    ((This)->lpVtbl->put_Square310x310Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_Square310x310Logo(This, value) \
    ((This)->lpVtbl->get_Square310x310Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_ForegroundText(This, value) \
    ((This)->lpVtbl->put_ForegroundText(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_ForegroundText(This, value) \
    ((This)->lpVtbl->get_ForegroundText(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_ShowNameOnSquare150x150Logo(This, value) \
    ((This)->lpVtbl->put_ShowNameOnSquare150x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_ShowNameOnSquare150x150Logo(This, value) \
    ((This)->lpVtbl->get_ShowNameOnSquare150x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_ShowNameOnWide310x150Logo(This, value) \
    ((This)->lpVtbl->put_ShowNameOnWide310x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_ShowNameOnWide310x150Logo(This, value) \
    ((This)->lpVtbl->get_ShowNameOnWide310x150Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_put_ShowNameOnSquare310x310Logo(This, value) \
    ((This)->lpVtbl->put_ShowNameOnSquare310x310Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_get_ShowNameOnSquare310x310Logo(This, value) \
    ((This)->lpVtbl->get_ShowNameOnSquare310x310Logo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements2[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements2";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Square71x71Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Square71x71Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_put_Square71x71Logo(This, value) \
    ((This)->lpVtbl->put_Square71x71Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_get_Square71x71Logo(This, value) \
    ((This)->lpVtbl->get_Square71x71Logo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements3[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements3";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Square44x44Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Square44x44Logo)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_put_Square44x44Logo(This, value) \
    ((This)->lpVtbl->put_Square44x44Logo(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_get_Square44x44Logo(This, value) \
    ((This)->lpVtbl->get_Square44x44Logo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.ISecondaryTileVisualElements4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ISecondaryTileVisualElements4[] = L"Windows.UI.StartScreen.ISecondaryTileVisualElements4";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MixedRealityModel)(__x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4* This,
        __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_get_MixedRealityModel(This, value) \
    ((This)->lpVtbl->get_MixedRealityModel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.StartScreen.IStartScreenManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.StartScreenManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IStartScreenManager[] = L"Windows.UI.StartScreen.IStartScreenManager";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* SupportsAppListEntry)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* appListEntry,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* ContainsAppListEntryAsync)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* appListEntry,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestAddAppListEntryAsync)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* appListEntry,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_SupportsAppListEntry(This, appListEntry, result) \
    ((This)->lpVtbl->SupportsAppListEntry(This, appListEntry, result))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_ContainsAppListEntryAsync(This, appListEntry, operation) \
    ((This)->lpVtbl->ContainsAppListEntryAsync(This, appListEntry, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_RequestAddAppListEntryAsync(This, appListEntry, operation) \
    ((This)->lpVtbl->RequestAddAppListEntryAsync(This, appListEntry, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.StartScreen.IStartScreenManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.StartScreenManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.StartScreen.IStartScreenManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IStartScreenManager2[] = L"Windows.UI.StartScreen.IStartScreenManager2";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ContainsSecondaryTileAsync)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This,
        HSTRING tileId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryRemoveSecondaryTileAsync)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2* This,
        HSTRING tileId,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_ContainsSecondaryTileAsync(This, tileId, operation) \
    ((This)->lpVtbl->ContainsSecondaryTileAsync(This, tileId, operation))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_TryRemoveSecondaryTileAsync(This, tileId, operation) \
    ((This)->lpVtbl->TryRemoveSecondaryTileAsync(This, tileId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.StartScreen.IStartScreenManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.StartScreenManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IStartScreenManagerStatics[] = L"Windows.UI.StartScreen.IStartScreenManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager** value);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIStartScreenManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.StartScreen.ITileMixedRealityModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.TileMixedRealityModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ITileMixedRealityModel[] = L"Windows.UI.StartScreen.ITileMixedRealityModel";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_BoundingBox)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox* value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingBox)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel* This,
        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingBox** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModelVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_put_BoundingBox(This, value) \
    ((This)->lpVtbl->put_BoundingBox(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_get_BoundingBox(This, value) \
    ((This)->lpVtbl->get_BoundingBox(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.StartScreen.ITileMixedRealityModel2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.TileMixedRealityModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_ITileMixedRealityModel2[] = L"Windows.UI.StartScreen.ITileMixedRealityModel2";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ActivationBehavior)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileMixedRealityModelActivationBehavior value);
    HRESULT (STDMETHODCALLTYPE* get_ActivationBehavior)(__x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2* This,
        enum __x_ABI_CWindows_CUI_CStartScreen_CTileMixedRealityModelActivationBehavior* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2Vtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_put_ActivationBehavior(This, value) \
    ((This)->lpVtbl->put_ActivationBehavior(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_get_ActivationBehavior(This, value) \
    ((This)->lpVtbl->get_ActivationBehavior(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CITileMixedRealityModel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.StartScreen.IVisualElementsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.VisualElementsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IVisualElementsRequest[] = L"Windows.UI.StartScreen.IVisualElementsRequest";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VisualElements)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        __x_ABI_CWindows_CUI_CStartScreen_CISecondaryTileVisualElements** value);
    HRESULT (STDMETHODCALLTYPE* get_AlternateVisualElements)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        __FIVectorView_1_Windows__CUI__CStartScreen__CSecondaryTileVisualElements** value);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_get_VisualElements(This, value) \
    ((This)->lpVtbl->get_VisualElements(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_get_AlternateVisualElements(This, value) \
    ((This)->lpVtbl->get_AlternateVisualElements(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.IVisualElementsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.VisualElementsRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IVisualElementsRequestDeferral[] = L"Windows.UI.StartScreen.IVisualElementsRequestDeferral";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferralVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.StartScreen.IVisualElementsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.StartScreen.VisualElementsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_StartScreen_IVisualElementsRequestedEventArgs[] = L"Windows.UI.StartScreen.IVisualElementsRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CStartScreen_CIVisualElementsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.JumpList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.IJumpListStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IJumpList ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_JumpList_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_JumpList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_JumpList[] = L"Windows.UI.StartScreen.JumpList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.StartScreen.JumpListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.IJumpListItemStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IJumpListItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_JumpListItem_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_JumpListItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_JumpListItem[] = L"Windows.UI.StartScreen.JumpListItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.StartScreen.SecondaryTile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.StartScreen.ISecondaryTileFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.StartScreen.ISecondaryTileFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.ISecondaryTileStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.ISecondaryTile ** Default Interface **
 *    Windows.UI.StartScreen.ISecondaryTile2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTile_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_SecondaryTile[] = L"Windows.UI.StartScreen.SecondaryTile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.SecondaryTileVisualElements
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements ** Default Interface **
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements2
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements3
 *    Windows.UI.StartScreen.ISecondaryTileVisualElements4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTileVisualElements_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_SecondaryTileVisualElements_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_SecondaryTileVisualElements[] = L"Windows.UI.StartScreen.SecondaryTileVisualElements";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.StartScreenManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.StartScreen.IStartScreenManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IStartScreenManager ** Default Interface **
 *    Windows.UI.StartScreen.IStartScreenManager2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_StartScreenManager_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_StartScreenManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_StartScreenManager[] = L"Windows.UI.StartScreen.StartScreenManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.StartScreen.TileMixedRealityModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.ITileMixedRealityModel ** Default Interface **
 *    Windows.UI.StartScreen.ITileMixedRealityModel2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_TileMixedRealityModel_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_TileMixedRealityModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_TileMixedRealityModel[] = L"Windows.UI.StartScreen.TileMixedRealityModel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.StartScreen.VisualElementsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IVisualElementsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_VisualElementsRequest[] = L"Windows.UI.StartScreen.VisualElementsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.VisualElementsRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IVisualElementsRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_VisualElementsRequestDeferral[] = L"Windows.UI.StartScreen.VisualElementsRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.StartScreen.VisualElementsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.StartScreen.IVisualElementsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_StartScreen_VisualElementsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_StartScreen_VisualElementsRequestedEventArgs[] = L"Windows.UI.StartScreen.VisualElementsRequestedEventArgs";
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
#endif // __windows2Eui2Estartscreen_p_h__

#endif // __windows2Eui2Estartscreen_h__
