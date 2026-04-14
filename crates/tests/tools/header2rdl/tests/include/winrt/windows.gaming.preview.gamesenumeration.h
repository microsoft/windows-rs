
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
#ifndef __windows2Egaming2Epreview2Egamesenumeration_h__
#define __windows2Egaming2Epreview2Egamesenumeration_h__
#ifndef __windows2Egaming2Epreview2Egamesenumeration_p_h__
#define __windows2Egaming2Epreview2Egamesenumeration_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION)
#define WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.h"
#include "Windows.Gaming.Preview.h"
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameListChangedEventHandler;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListChangedEventHandler

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameListRemovedEventHandler;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListRemovedEventHandler

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameListEntry;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameListEntry2;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2 ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry2

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameListStatics;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListStatics

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameListStatics2;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2 ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListStatics2

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameModeConfiguration;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration ABI::Windows::Gaming::Preview::GamesEnumeration::IGameModeConfiguration

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameModeUserConfiguration;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration ABI::Windows::Gaming::Preview::GamesEnumeration::IGameModeUserConfiguration

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    interface IGameModeUserConfigurationStatics;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics ABI::Windows::Gaming::Preview::GamesEnumeration::IGameModeUserConfigurationStatics

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_FWD_DEFINED__

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
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    class GameListEntry;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ed5b903e-5aeb-5d8c-9538-8306f02926c3"))
IIterator<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*, ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("42b8c8a0-3d03-5d5f-817e-4405c850f646"))
IIterable<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*, ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0eea1ad9-03e2-5ba9-ae02-daca432f362a"))
IVectorView<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*, ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("721a94fb-a733-5e19-8abf-03249c29870e"))
IAsyncOperation<__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry*> __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eadac44b-7fdd-5589-b093-1bb73cc64f02"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("37bcb2e2-9c6f-5658-a43b-ed28fe0c8458"))
IAsyncOperation<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*, ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a77293e0-d508-5582-ac76-8c9605fa1de9"))
IAsyncOperationCompletedHandler<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*, ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.Preview.GamesEnumeration.GameListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntry*> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_USE */

#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIReference_1_int_USE
#define DEF___FIReference_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("548cefbd-bc8a-5fa0-8df2-957440fc8bf4"))
IReference<int> : IReference_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<int> __FIReference_1_int_t;
#define __FIReference_1_int ABI::Windows::Foundation::__FIReference_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_int_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppDisplayInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppDisplayInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo ABI::Windows::ApplicationModel::IAppDisplayInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    typedef enum GameListCategory : int GameListCategory;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    typedef enum GameListEntryLaunchableState : int GameListEntryLaunchableState;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    class GameModeConfiguration;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    class GameModeUserConfiguration;
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.Preview.GamesEnumeration.GameListCategory
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    enum GameListCategory : int
                    {
                        GameListCategory_Candidate = 0,
                        GameListCategory_ConfirmedBySystem = 1,
                        GameListCategory_ConfirmedByUser = 2,
                    };
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Preview.GamesEnumeration.GameListEntryLaunchableState
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    enum GameListEntryLaunchableState : int
                    {
                        GameListEntryLaunchableState_NotLaunchable = 0,
                        GameListEntryLaunchableState_ByLastRunningFullPath = 1,
                        GameListEntryLaunchableState_ByUserProvidedPath = 2,
                        GameListEntryLaunchableState_ByTile = 3,
                    };
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Delegate Windows.Gaming.Preview.GamesEnumeration.GameListChangedEventHandler
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("25f6a421-d8f5-4d91-b40e-53d5e86fde64")
                    IGameListChangedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry* game
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameListChangedEventHandler = __uuidof(IGameListChangedEventHandler);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Gaming.Preview.GamesEnumeration.GameListRemovedEventHandler
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("10c5648f-6c8f-4712-9b38-474bc22e76d8")
                    IGameListRemovedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            HSTRING identifier
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameListRemovedEventHandler = __uuidof(IGameListRemovedEventHandler);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListEntry
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListEntry[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("735924d3-811f-4494-b69c-c641a0c61543")
                    IGameListEntry : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayInfo(
                            ABI::Windows::ApplicationModel::IAppDisplayInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LaunchAsync(
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Category(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::GameListCategory* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_HSTRING_IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetCategoryAsync(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::GameListCategory value,
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameListEntry = __uuidof(IGameListEntry);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameListEntry
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Preview.GamesEnumeration.IGameListEntry
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListEntry2[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("d84a8f8b-8749-4a25-90d3-f6c5a427886d")
                    IGameListEntry2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LaunchableState(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::GameListEntryLaunchableState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LauncherExecutable(
                            ABI::Windows::Storage::IStorageFile** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LaunchParameters(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLauncherExecutableFileAsync(
                            ABI::Windows::Storage::IStorageFile* executableFile,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLauncherExecutableFileWithParamsAsync(
                            ABI::Windows::Storage::IStorageFile* executableFile,
                            HSTRING launchParams,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TitleId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetTitleIdAsync(
                            HSTRING id,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_GameModeConfiguration(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameModeConfiguration** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameListEntry2 = __uuidof(IGameListEntry2);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListStatics
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameList
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListStatics[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("2ddd0f6f-9c66-4b05-945c-d6ed78491b8c")
                    IGameListStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindAllAsyncPackageFamilyName(
                            HSTRING packageFamilyName,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GameAdded(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListChangedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GameAdded(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GameRemoved(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListRemovedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GameRemoved(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GameUpdated(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListChangedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GameUpdated(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameListStatics = __uuidof(IGameListStatics);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameList
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListStatics2[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("395f2098-ea1a-45aa-9268-a83905686f27")
                    IGameListStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE MergeEntriesAsync(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry* left,
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry* right,
                            __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnmergeEntryAsync(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameListEntry* mergedEntry,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameListStatics2 = __uuidof(IGameListStatics2);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameModeConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("78e591af-b142-4ef0-8830-55bc2be4f5ea")
                    IGameModeConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RelatedProcessNames(
                            __FIVector_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PercentGpuTimeAllocatedToGame(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PercentGpuTimeAllocatedToGame(
                            __FIReference_1_int* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PercentGpuMemoryAllocatedToGame(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PercentGpuMemoryAllocatedToGame(
                            __FIReference_1_int* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PercentGpuMemoryAllocatedToSystemCompositor(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PercentGpuMemoryAllocatedToSystemCompositor(
                            __FIReference_1_int* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxCpuCount(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MaxCpuCount(
                            __FIReference_1_int* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CpuExclusivityMaskLow(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CpuExclusivityMaskLow(
                            __FIReference_1_int* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CpuExclusivityMaskHigh(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CpuExclusivityMaskHigh(
                            __FIReference_1_int* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AffinitizeToExclusiveCpus(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AffinitizeToExclusiveCpus(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameModeConfiguration = __uuidof(IGameModeConfiguration);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameModeUserConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("72d34af4-756b-470f-a0c2-ba62a90795db")
                    IGameModeUserConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_GamingRelatedProcessNames(
                            __FIVector_1_HSTRING** processNames
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameModeUserConfiguration = __uuidof(IGameModeUserConfiguration);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameModeUserConfigurationStatics[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Preview {
                namespace GamesEnumeration {
                    MIDL_INTERFACE("6e50d97c-66ea-478e-a4a1-f57c0e8d00e7")
                    IGameModeUserConfigurationStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDefault(
                            ABI::Windows::Gaming::Preview::GamesEnumeration::IGameModeUserConfiguration** userConfiguration
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameModeUserConfigurationStatics = __uuidof(IGameModeUserConfigurationStatics);
                } /* GamesEnumeration */
            } /* Preview */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameList
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Preview.GamesEnumeration.IGameListStatics interface starting with version 1.0 of the Windows.Gaming.Preview.GamesEnumerationContract API contract
 *   Static Methods exist on the Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2 interface starting with version 2.0 of the Windows.Gaming.Preview.GamesEnumerationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameList_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameList[] = L"Windows.Gaming.Preview.GamesEnumeration.GameList";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameListEntry
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Preview.GamesEnumeration.IGameListEntry ** Default Interface **
 *    Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameListEntry_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameListEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameListEntry[] = L"Windows.Gaming.Preview.GamesEnumeration.GameListEntry";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameModeConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics interface starting with version 2.0 of the Windows.Gaming.Preview.GamesEnumerationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeUserConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeUserConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameModeUserConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2 __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2 __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics;

#endif // ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_FWD_DEFINED__

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

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIIterator_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* This,
        __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

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

#if !defined(____FIReference_1_int_INTERFACE_DEFINED__)
#define ____FIReference_1_int_INTERFACE_DEFINED__

typedef interface __FIReference_1_int __FIReference_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_int;

typedef struct __FIReference_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIReference_1_intVtbl;

interface __FIReference_1_int
{
    CONST_VTBL struct __FIReference_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_int_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_int_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListCategory __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListCategory;

typedef enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListEntryLaunchableState __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListEntryLaunchableState;

/*
 *
 * Struct Windows.Gaming.Preview.GamesEnumeration.GameListCategory
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListCategory
{
    GameListCategory_Candidate = 0,
    GameListCategory_ConfirmedBySystem = 1,
    GameListCategory_ConfirmedByUser = 2,
};
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Gaming.Preview.GamesEnumeration.GameListEntryLaunchableState
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListEntryLaunchableState
{
    GameListEntryLaunchableState_NotLaunchable = 0,
    GameListEntryLaunchableState_ByLastRunningFullPath = 1,
    GameListEntryLaunchableState_ByUserProvidedPath = 2,
    GameListEntryLaunchableState_ByTile = 3,
};
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Delegate Windows.Gaming.Preview.GamesEnumeration.GameListChangedEventHandler
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* game);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandlerVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_Invoke(This, game) \
    ((This)->lpVtbl->Invoke(This, game))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Gaming.Preview.GamesEnumeration.GameListRemovedEventHandler
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler* This,
        HSTRING identifier);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandlerVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_Invoke(This, identifier) \
    ((This)->lpVtbl->Invoke(This, identifier))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListEntry
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListEntry[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayInfo)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo** value);
    HRESULT (STDMETHODCALLTYPE* LaunchAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* get_Category)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListCategory* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        __FIMapView_2_HSTRING_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* SetCategoryAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* This,
        enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListCategory value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntryVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_get_DisplayInfo(This, value) \
    ((This)->lpVtbl->get_DisplayInfo(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_LaunchAsync(This, operation) \
    ((This)->lpVtbl->LaunchAsync(This, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_get_Category(This, value) \
    ((This)->lpVtbl->get_Category(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_SetCategoryAsync(This, value, action) \
    ((This)->lpVtbl->SetCategoryAsync(This, value, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameListEntry
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Preview.GamesEnumeration.IGameListEntry
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListEntry2[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LaunchableState)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        enum __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CGameListEntryLaunchableState* value);
    HRESULT (STDMETHODCALLTYPE* get_LauncherExecutable)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* get_LaunchParameters)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* SetLauncherExecutableFileAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* executableFile,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SetLauncherExecutableFileWithParamsAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* executableFile,
        HSTRING launchParams,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_TitleId)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* SetTitleIdAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        HSTRING id,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_GameModeConfiguration)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2Vtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_get_LaunchableState(This, value) \
    ((This)->lpVtbl->get_LaunchableState(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_get_LauncherExecutable(This, value) \
    ((This)->lpVtbl->get_LauncherExecutable(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_get_LaunchParameters(This, value) \
    ((This)->lpVtbl->get_LaunchParameters(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_SetLauncherExecutableFileAsync(This, executableFile, operation) \
    ((This)->lpVtbl->SetLauncherExecutableFileAsync(This, executableFile, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_SetLauncherExecutableFileWithParamsAsync(This, executableFile, launchParams, operation) \
    ((This)->lpVtbl->SetLauncherExecutableFileWithParamsAsync(This, executableFile, launchParams, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_get_TitleId(This, value) \
    ((This)->lpVtbl->get_TitleId(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_SetTitleIdAsync(This, id, operation) \
    ((This)->lpVtbl->SetTitleIdAsync(This, id, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_get_GameModeConfiguration(This, value) \
    ((This)->lpVtbl->get_GameModeConfiguration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListStatics
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameList
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListStatics[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListStatics";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllAsyncPackageFamilyName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        HSTRING packageFamilyName,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation);
    HRESULT (STDMETHODCALLTYPE* add_GameAdded)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GameAdded)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GameRemoved)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListRemovedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GameRemoved)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GameUpdated)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListChangedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GameUpdated)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FindAllAsync(This, operation) \
    ((This)->lpVtbl->FindAllAsync(This, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_FindAllAsyncPackageFamilyName(This, packageFamilyName, operation) \
    ((This)->lpVtbl->FindAllAsyncPackageFamilyName(This, packageFamilyName, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_add_GameAdded(This, handler, token) \
    ((This)->lpVtbl->add_GameAdded(This, handler, token))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_remove_GameAdded(This, token) \
    ((This)->lpVtbl->remove_GameAdded(This, token))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_add_GameRemoved(This, handler, token) \
    ((This)->lpVtbl->add_GameRemoved(This, handler, token))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_remove_GameRemoved(This, token) \
    ((This)->lpVtbl->remove_GameRemoved(This, token))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_add_GameUpdated(This, handler, token) \
    ((This)->lpVtbl->add_GameUpdated(This, handler, token))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_remove_GameUpdated(This, token) \
    ((This)->lpVtbl->remove_GameUpdated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameList
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameListStatics2[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* MergeEntriesAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* left,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* right,
        __FIAsyncOperation_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation);
    HRESULT (STDMETHODCALLTYPE* UnmergeEntryAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListEntry* mergedEntry,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CGaming__CPreview__CGamesEnumeration__CGameListEntry** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2Vtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_MergeEntriesAsync(This, left, right, operation) \
    ((This)->lpVtbl->MergeEntriesAsync(This, left, right, operation))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_UnmergeEntryAsync(This, mergedEntry, operation) \
    ((This)->lpVtbl->UnmergeEntryAsync(This, mergedEntry, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameListStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameModeConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RelatedProcessNames)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_PercentGpuTimeAllocatedToGame)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_PercentGpuTimeAllocatedToGame)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_PercentGpuMemoryAllocatedToGame)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_PercentGpuMemoryAllocatedToGame)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_PercentGpuMemoryAllocatedToSystemCompositor)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_PercentGpuMemoryAllocatedToSystemCompositor)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxCpuCount)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_MaxCpuCount)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_CpuExclusivityMaskLow)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_CpuExclusivityMaskLow)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_CpuExclusivityMaskHigh)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_CpuExclusivityMaskHigh)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_AffinitizeToExclusiveCpus)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AffinitizeToExclusiveCpus)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfigurationVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_RelatedProcessNames(This, value) \
    ((This)->lpVtbl->get_RelatedProcessNames(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_PercentGpuTimeAllocatedToGame(This, value) \
    ((This)->lpVtbl->get_PercentGpuTimeAllocatedToGame(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_PercentGpuTimeAllocatedToGame(This, value) \
    ((This)->lpVtbl->put_PercentGpuTimeAllocatedToGame(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_PercentGpuMemoryAllocatedToGame(This, value) \
    ((This)->lpVtbl->get_PercentGpuMemoryAllocatedToGame(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_PercentGpuMemoryAllocatedToGame(This, value) \
    ((This)->lpVtbl->put_PercentGpuMemoryAllocatedToGame(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_PercentGpuMemoryAllocatedToSystemCompositor(This, value) \
    ((This)->lpVtbl->get_PercentGpuMemoryAllocatedToSystemCompositor(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_PercentGpuMemoryAllocatedToSystemCompositor(This, value) \
    ((This)->lpVtbl->put_PercentGpuMemoryAllocatedToSystemCompositor(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_MaxCpuCount(This, value) \
    ((This)->lpVtbl->get_MaxCpuCount(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_MaxCpuCount(This, value) \
    ((This)->lpVtbl->put_MaxCpuCount(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_CpuExclusivityMaskLow(This, value) \
    ((This)->lpVtbl->get_CpuExclusivityMaskLow(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_CpuExclusivityMaskLow(This, value) \
    ((This)->lpVtbl->put_CpuExclusivityMaskLow(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_CpuExclusivityMaskHigh(This, value) \
    ((This)->lpVtbl->get_CpuExclusivityMaskHigh(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_CpuExclusivityMaskHigh(This, value) \
    ((This)->lpVtbl->put_CpuExclusivityMaskHigh(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_get_AffinitizeToExclusiveCpus(This, value) \
    ((This)->lpVtbl->get_AffinitizeToExclusiveCpus(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_put_AffinitizeToExclusiveCpus(This, value) \
    ((This)->lpVtbl->put_AffinitizeToExclusiveCpus(This, value))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_SaveAsync(This, operation) \
    ((This)->lpVtbl->SaveAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameModeUserConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GamingRelatedProcessNames)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This,
        __FIVector_1_HSTRING** processNames);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_get_GamingRelatedProcessNames(This, processNames) \
    ((This)->lpVtbl->get_GamingRelatedProcessNames(This, processNames))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_SaveAsync(This, operation) \
    ((This)->lpVtbl->SaveAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Preview_GamesEnumeration_IGameModeUserConfigurationStatics[] = L"Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics";
typedef struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics* This,
        __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfiguration** userConfiguration);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_GetDefault(This, userConfiguration) \
    ((This)->lpVtbl->GetDefault(This, userConfiguration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CPreview_CGamesEnumeration_CIGameModeUserConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameList
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Preview.GamesEnumeration.IGameListStatics interface starting with version 1.0 of the Windows.Gaming.Preview.GamesEnumerationContract API contract
 *   Static Methods exist on the Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2 interface starting with version 2.0 of the Windows.Gaming.Preview.GamesEnumerationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameList_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameList[] = L"Windows.Gaming.Preview.GamesEnumeration.GameList";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameListEntry
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Preview.GamesEnumeration.IGameListEntry ** Default Interface **
 *    Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameListEntry_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameListEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameListEntry[] = L"Windows.Gaming.Preview.GamesEnumeration.GameListEntry";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameModeConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration
 *
 * Introduced to Windows.Gaming.Preview.GamesEnumerationContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics interface starting with version 2.0 of the Windows.Gaming.Preview.GamesEnumerationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeUserConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Preview_GamesEnumeration_GameModeUserConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Preview_GamesEnumeration_GameModeUserConfiguration[] = L"Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration";
#endif
#endif // WINDOWS_GAMING_PREVIEW_GAMESENUMERATIONCONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egaming2Epreview2Egamesenumeration_p_h__

#endif // __windows2Egaming2Epreview2Egamesenumeration_h__
