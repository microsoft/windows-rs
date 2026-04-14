
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
#ifndef __windows2Eapplicationmodel2Esocialinfo_h__
#define __windows2Eapplicationmodel2Esocialinfo_h__
#ifndef __windows2Eapplicationmodel2Esocialinfo_p_h__
#define __windows2Eapplicationmodel2Esocialinfo_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION)

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
#include "Windows.Graphics.Imaging.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialFeedChildItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedChildItem

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialFeedContent;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialFeedItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedItem

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialFeedSharedItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedSharedItem

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialItemThumbnail;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                interface ISocialUserInfo;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo ABI::Windows::ApplicationModel::SocialInfo::ISocialUserInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialItemThumbnail;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fabcf39f-fd48-5550-8f47-a0f1573e1f53"))
IIterator<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*, ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.SocialInfo.SocialItemThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t;
#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5d102c6d-92c3-59f3-b1dc-5986c56445a5"))
IIterable<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*, ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.SocialInfo.SocialItemThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t;
#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9a3e6d46-e880-5deb-9006-92fe5c43ace1"))
IVectorView<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*, ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.SocialInfo.SocialItemThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t;
#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c210bbd7-2f56-5076-bb0e-b7497726cf95"))
IVector<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*, ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.SocialInfo.SocialItemThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::SocialInfo::SocialItemThumbnail*> __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t;
#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_USE */

#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

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
        namespace Graphics {
            namespace Imaging {
                typedef struct BitmapSize BitmapSize;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                typedef enum SocialFeedItemStyle : int SocialFeedItemStyle;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                typedef enum SocialItemBadgeStyle : int SocialItemBadgeStyle;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialFeedChildItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialFeedContent;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialFeedSharedItem;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                class SocialUserInfo;
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialFeedItemStyle
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("SocialFeedItemStyle is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                SocialFeedItemStyle : int
                {
                    SocialFeedItemStyle_Default = 0,
                    SocialFeedItemStyle_Photo = 1,
                };
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialFeedKind
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("SocialFeedKind is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                SocialFeedKind : int
                {
                    SocialFeedKind_HomeFeed = 0,
                    SocialFeedKind_ContactFeed = 1,
                    SocialFeedKind_Dashboard = 2,
                };
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialFeedUpdateMode
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("SocialFeedUpdateMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                SocialFeedUpdateMode : int
                {
                    SocialFeedUpdateMode_Append = 0,
                    SocialFeedUpdateMode_Replace = 1,
                };
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialItemBadgeStyle
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("SocialItemBadgeStyle is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                SocialItemBadgeStyle : int
                {
                    SocialItemBadgeStyle_Hidden = 0,
                    SocialItemBadgeStyle_Visible = 1,
                    SocialItemBadgeStyle_VisibleWithCount = 2,
                };
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedChildItem
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedChildItem[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                MIDL_INTERFACE("0b6a985a-d59d-40be-980c-488a2ab30a83")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                ISocialFeedChildItem : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Author(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialUserInfo** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_PrimaryContent(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_SecondaryContent(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Timestamp(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnails(
                        __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_SharedItem(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedSharedItem** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_SharedItem(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedSharedItem* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocialFeedChildItem = __uuidof(ISocialFeedChildItem);
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedContent
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedContent
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedContent[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedContent";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                MIDL_INTERFACE("a234e429-3e39-494d-a37c-f462a2494514")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                ISocialFeedContent : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Message(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocialFeedContent = __uuidof(ISocialFeedContent);
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedItem
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedItem[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                MIDL_INTERFACE("4f1392ab-1f72-4d33-b695-de3e1db60317")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                ISocialFeedItem : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Author(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialUserInfo** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_PrimaryContent(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_SecondaryContent(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Timestamp(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnails(
                        __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_SharedItem(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedSharedItem** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_SharedItem(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedSharedItem* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_BadgeStyle(
                        ABI::Windows::ApplicationModel::SocialInfo::SocialItemBadgeStyle* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_BadgeStyle(
                        ABI::Windows::ApplicationModel::SocialInfo::SocialItemBadgeStyle value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_BadgeCountValue(
                        INT32* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_BadgeCountValue(
                        INT32 value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_ChildItem(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedChildItem** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_ChildItem(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedChildItem* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Style(
                        ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItemStyle* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Style(
                        ABI::Windows::ApplicationModel::SocialInfo::SocialFeedItemStyle value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocialFeedItem = __uuidof(ISocialFeedItem);
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedSharedItem[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                MIDL_INTERFACE("7bfb9e40-a6aa-45a7-9ff6-54c42105dd1f")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                ISocialFeedSharedItem : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalSource(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_OriginalSource(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialFeedContent** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Timestamp(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::ApplicationModel::SocialInfo::ISocialItemThumbnail** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocialFeedSharedItem = __uuidof(ISocialFeedSharedItem);
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialItemThumbnail
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialItemThumbnail[] = L"Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                MIDL_INTERFACE("5cbf831a-3f08-497f-917f-57e09d84b141")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                ISocialItemThumbnail : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_ImageUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_ImageUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_BitmapSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE SetImageAsync(
                        ABI::Windows::Storage::Streams::IInputStream* image,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocialItemThumbnail = __uuidof(ISocialItemThumbnail);
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialUserInfo
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialUserInfo
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialUserInfo[] = L"Windows.ApplicationModel.SocialInfo.ISocialUserInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace SocialInfo {
                MIDL_INTERFACE("9e5e1bd1-90d0-4e1d-9554-844d46607f61")
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                ISocialUserInfo : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_UserName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_UserName(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE put_TargetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISocialUserInfo = __uuidof(ISocialUserInfo);
            } /* SocialInfo */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedChildItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedChildItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedChildItem_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedChildItem[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedChildItem";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedContent
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedContent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedContent_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedContent_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedContent[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedContent";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedItem_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedItem[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedItem";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedSharedItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedSharedItem_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedSharedItem[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialItemThumbnail
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialItemThumbnail_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialItemThumbnail_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialItemThumbnail[] = L"Windows.ApplicationModel.SocialInfo.SocialItemThumbnail";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialUserInfo
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialUserInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialUserInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialUserInfo_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialUserInfo[] = L"Windows.ApplicationModel.SocialInfo.SocialUserInfo";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

typedef struct __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

typedef struct __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        __FIIterator_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail;

typedef struct __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        __FIVectorView_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl;

interface __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedItemStyle __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedItemStyle;

typedef enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialItemBadgeStyle __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialItemBadgeStyle;

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialFeedItemStyle
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedItemStyle is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedItemStyle
{
    SocialFeedItemStyle_Default = 0,
    SocialFeedItemStyle_Photo = 1,
};
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialFeedKind
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedKind is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedKind
{
    SocialFeedKind_HomeFeed = 0,
    SocialFeedKind_ContactFeed = 1,
    SocialFeedKind_Dashboard = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialFeedUpdateMode
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedUpdateMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedUpdateMode
{
    SocialFeedUpdateMode_Append = 0,
    SocialFeedUpdateMode_Replace = 1,
};
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.SocialInfo.SocialItemBadgeStyle
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialItemBadgeStyle is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialItemBadgeStyle
{
    SocialItemBadgeStyle_Hidden = 0,
    SocialItemBadgeStyle_Visible = 1,
    SocialItemBadgeStyle_VisibleWithCount = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedChildItem
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedChildItem[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_PrimaryContent)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_SecondaryContent)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Thumbnails)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_SharedItem)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_SharedItem)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_PrimaryContent(This, value) \
    ((This)->lpVtbl->get_PrimaryContent(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_SecondaryContent(This, value) \
    ((This)->lpVtbl->get_SecondaryContent(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_put_Timestamp(This, value) \
    ((This)->lpVtbl->put_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_Thumbnails(This, value) \
    ((This)->lpVtbl->get_Thumbnails(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_get_SharedItem(This, value) \
    ((This)->lpVtbl->get_SharedItem(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_put_SharedItem(This, value) \
    ((This)->lpVtbl->put_SharedItem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedContent
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedContent
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedContent[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedContent";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Message)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContentVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_put_Message(This, value) \
    ((This)->lpVtbl->put_Message(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedItem
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedItem[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedItem";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_PrimaryContent)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_SecondaryContent)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Thumbnails)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __FIVector_1_Windows__CApplicationModel__CSocialInfo__CSocialItemThumbnail** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_SharedItem)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_SharedItem)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_BadgeStyle)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialItemBadgeStyle* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_BadgeStyle)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialItemBadgeStyle value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_BadgeCountValue)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        INT32* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_BadgeCountValue)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        INT32 value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ChildItem)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_ChildItem)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedChildItem* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Style)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedItemStyle* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Style)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CSocialInfo_CSocialFeedItemStyle value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_PrimaryContent(This, value) \
    ((This)->lpVtbl->get_PrimaryContent(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_SecondaryContent(This, value) \
    ((This)->lpVtbl->get_SecondaryContent(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_Timestamp(This, value) \
    ((This)->lpVtbl->put_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_Thumbnails(This, value) \
    ((This)->lpVtbl->get_Thumbnails(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_SharedItem(This, value) \
    ((This)->lpVtbl->get_SharedItem(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_SharedItem(This, value) \
    ((This)->lpVtbl->put_SharedItem(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_BadgeStyle(This, value) \
    ((This)->lpVtbl->get_BadgeStyle(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_BadgeStyle(This, value) \
    ((This)->lpVtbl->put_BadgeStyle(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_BadgeCountValue(This, value) \
    ((This)->lpVtbl->get_BadgeCountValue(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_BadgeCountValue(This, value) \
    ((This)->lpVtbl->put_BadgeCountValue(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_ChildItem(This, value) \
    ((This)->lpVtbl->get_ChildItem(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_ChildItem(This, value) \
    ((This)->lpVtbl->put_ChildItem(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_get_Style(This, value) \
    ((This)->lpVtbl->get_Style(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_put_Style(This, value) \
    ((This)->lpVtbl->put_Style(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialFeedSharedItem[] = L"Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_OriginalSource)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_OriginalSource)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedContent** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Timestamp)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem* This,
        __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_get_OriginalSource(This, value) \
    ((This)->lpVtbl->get_OriginalSource(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_put_OriginalSource(This, value) \
    ((This)->lpVtbl->put_OriginalSource(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_put_Timestamp(This, value) \
    ((This)->lpVtbl->put_Timestamp(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialFeedSharedItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialItemThumbnail
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialItemThumbnail[] = L"Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ImageUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_ImageUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_BitmapSize)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_BitmapSize)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* SetImageAsync)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* image,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnailVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_get_ImageUri(This, value) \
    ((This)->lpVtbl->get_ImageUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_put_ImageUri(This, value) \
    ((This)->lpVtbl->put_ImageUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_get_BitmapSize(This, value) \
    ((This)->lpVtbl->get_BitmapSize(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_put_BitmapSize(This, value) \
    ((This)->lpVtbl->put_BitmapSize(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_SetImageAsync(This, image, operation) \
    ((This)->lpVtbl->SetImageAsync(This, image, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialItemThumbnail_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.SocialInfo.ISocialUserInfo
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.SocialInfo.SocialUserInfo
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_SocialInfo_ISocialUserInfo[] = L"Windows.ApplicationModel.SocialInfo.ISocialUserInfo";
typedef struct
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_UserName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_UserName)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* put_TargetUri)(__x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_get_UserName(This, value) \
    ((This)->lpVtbl->get_UserName(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_put_UserName(This, value) \
    ((This)->lpVtbl->put_UserName(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_get_TargetUri(This, value) \
    ((This)->lpVtbl->get_TargetUri(This, value))

#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
    DEPRECATED("ISocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_put_TargetUri(This, value) \
    ((This)->lpVtbl->put_TargetUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CSocialInfo_CISocialUserInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedChildItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedChildItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedChildItem_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedChildItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedChildItem[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedChildItem";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedContent
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedContent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedContent_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedContent_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedContent is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedContent[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedContent";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedItem_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedItem[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedItem";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedSharedItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialFeedSharedItem_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialFeedSharedItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialFeedSharedItem[] = L"Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialItemThumbnail
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.SocialInfo.SocialInfoContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialItemThumbnail_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialItemThumbnail_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialItemThumbnail is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialItemThumbnail[] = L"Windows.ApplicationModel.SocialInfo.SocialItemThumbnail";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SocialInfo.SocialUserInfo
 *
 * Introduced to Windows.ApplicationModel.SocialInfo.SocialInfoContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.SocialInfo.ISocialUserInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialUserInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SocialInfo_SocialUserInfo_DEFINED
#if WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
DEPRECATED("SocialUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SocialInfo_SocialUserInfo[] = L"Windows.ApplicationModel.SocialInfo.SocialUserInfo";
#endif
#endif // WINDOWS_APPLICATIONMODEL_SOCIALINFO_SOCIALINFOCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Esocialinfo_p_h__

#endif // __windows2Eapplicationmodel2Esocialinfo_h__
