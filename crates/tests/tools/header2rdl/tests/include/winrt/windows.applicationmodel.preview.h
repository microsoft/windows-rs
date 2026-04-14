
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
#ifndef __windows2Eapplicationmodel2Epreview_h__
#define __windows2Eapplicationmodel2Epreview_h__
#ifndef __windows2Eapplicationmodel2Epreview_p_h__
#define __windows2Eapplicationmodel2Epreview_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION)

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
#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                interface IStartupAppInfoPreview;
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview ABI::Windows::ApplicationModel::Preview::IStartupAppInfoPreview

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                interface IStartupAppsManagerPreview;
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview ABI::Windows::ApplicationModel::Preview::IStartupAppsManagerPreview

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                interface IStartupAppsManagerPreviewStatics;
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics ABI::Windows::ApplicationModel::Preview::IStartupAppsManagerPreviewStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                class StartupAppInfoPreview;
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9bac9a7b-2678-5ebb-bc0d-90737a7b8f8f"))
IIterator<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*, ABI::Windows::ApplicationModel::Preview::IStartupAppInfoPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Preview.StartupAppInfoPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*> __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_t;
#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE */

#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3ee76ee0-7bfe-5c7a-9518-1bed0ecd62ad"))
IIterable<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*, ABI::Windows::ApplicationModel::Preview::IStartupAppInfoPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Preview.StartupAppInfoPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*> __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_t;
#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE */

#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bae0507d-eac4-5501-a77a-583c2e45e735"))
IVectorView<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*, ABI::Windows::ApplicationModel::Preview::IStartupAppInfoPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Preview.StartupAppInfoPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Preview::StartupAppInfoPreview*> __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_USE */

#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                typedef enum StartupAppImpactPreview : int StartupAppImpactPreview;
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                class StartupAppsManagerPreview;
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Preview.StartupAppImpactPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                enum StartupAppImpactPreview : int
                {
                    StartupAppImpactPreview_Unknown = 0,
                    StartupAppImpactPreview_None = 1,
                    StartupAppImpactPreview_Low = 2,
                    StartupAppImpactPreview_Medium = 3,
                    StartupAppImpactPreview_High = 4,
                };
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.IStartupAppInfoPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.StartupAppInfoPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_IStartupAppInfoPreview[] = L"Windows.ApplicationModel.Preview.IStartupAppInfoPreview";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                MIDL_INTERFACE("c3a147db-09fa-5aa5-b3bd-119a09963d58")
                IStartupAppInfoPreview : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Publisher(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Impact(
                        ABI::Windows::ApplicationModel::Preview::StartupAppImpactPreview* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExecutablePath(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartupAppInfoPreview = __uuidof(IStartupAppInfoPreview);
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.IStartupAppsManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.StartupAppsManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_IStartupAppsManagerPreview[] = L"Windows.ApplicationModel.Preview.IStartupAppsManagerPreview";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                MIDL_INTERFACE("7197b9c1-03bb-5693-87c3-6f983cc70fb3")
                IStartupAppsManagerPreview : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStartupAppInfos(
                        __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartupAppsManagerPreview = __uuidof(IStartupAppsManagerPreview);
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.IStartupAppsManagerPreviewStatics
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.StartupAppsManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_IStartupAppsManagerPreviewStatics[] = L"Windows.ApplicationModel.Preview.IStartupAppsManagerPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                MIDL_INTERFACE("9d0331f5-343f-5cd7-9d66-762cfa2c0380")
                IStartupAppsManagerPreviewStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::ApplicationModel::Preview::IStartupAppsManagerPreview** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartupAppsManagerPreviewStatics = __uuidof(IStartupAppsManagerPreviewStatics);
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.StartupAppInfoPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.IStartupAppInfoPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppInfoPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppInfoPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_StartupAppInfoPreview[] = L"Windows.ApplicationModel.Preview.StartupAppInfoPreview";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.StartupAppsManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.IStartupAppsManagerPreviewStatics interface starting with version 1.0 of the Windows.ApplicationModel.Preview.StartupAppsPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.IStartupAppsManagerPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppsManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppsManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_StartupAppsManagerPreview[] = L"Windows.ApplicationModel.Preview.StartupAppsManagerPreview";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        __FIIterator_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CApplicationModel_CPreview_CStartupAppImpactPreview __x_ABI_CWindows_CApplicationModel_CPreview_CStartupAppImpactPreview;

/*
 *
 * Struct Windows.ApplicationModel.Preview.StartupAppImpactPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CPreview_CStartupAppImpactPreview
{
    StartupAppImpactPreview_Unknown = 0,
    StartupAppImpactPreview_None = 1,
    StartupAppImpactPreview_Low = 2,
    StartupAppImpactPreview_Medium = 3,
    StartupAppImpactPreview_High = 4,
};
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.IStartupAppInfoPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.StartupAppInfoPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_IStartupAppInfoPreview[] = L"Windows.ApplicationModel.Preview.IStartupAppInfoPreview";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Publisher)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Impact)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        enum __x_ABI_CWindows_CApplicationModel_CPreview_CStartupAppImpactPreview* value);
    HRESULT (STDMETHODCALLTYPE* get_ExecutablePath)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreviewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_get_Publisher(This, value) \
    ((This)->lpVtbl->get_Publisher(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_get_Impact(This, value) \
    ((This)->lpVtbl->get_Impact(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_get_ExecutablePath(This, value) \
    ((This)->lpVtbl->get_ExecutablePath(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppInfoPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.IStartupAppsManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.StartupAppsManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_IStartupAppsManagerPreview[] = L"Windows.ApplicationModel.Preview.IStartupAppsManagerPreview";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStartupAppInfos)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview* This,
        __FIVectorView_1_Windows__CApplicationModel__CPreview__CStartupAppInfoPreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_GetStartupAppInfos(This, result) \
    ((This)->lpVtbl->GetStartupAppInfos(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.IStartupAppsManagerPreviewStatics
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.StartupAppsManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_IStartupAppsManagerPreviewStatics[] = L"Windows.ApplicationModel.Preview.IStartupAppsManagerPreviewStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CIStartupAppsManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.StartupAppInfoPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.IStartupAppInfoPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppInfoPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppInfoPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_StartupAppInfoPreview[] = L"Windows.ApplicationModel.Preview.StartupAppInfoPreview";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.StartupAppsManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.StartupAppsPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.IStartupAppsManagerPreviewStatics interface starting with version 1.0 of the Windows.ApplicationModel.Preview.StartupAppsPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.IStartupAppsManagerPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppsManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_StartupAppsManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_StartupAppsManagerPreview[] = L"Windows.ApplicationModel.Preview.StartupAppsManagerPreview";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_STARTUPAPPSPREVIEWCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Epreview_p_h__

#endif // __windows2Eapplicationmodel2Epreview_h__
