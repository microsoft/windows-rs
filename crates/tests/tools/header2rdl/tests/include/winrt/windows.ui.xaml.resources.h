
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
#ifndef __windows2Eui2Examl2Eresources_h__
#define __windows2Eui2Examl2Eresources_h__
#ifndef __windows2Eui2Examl2Eresources_p_h__
#define __windows2Eui2Examl2Eresources_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    interface ICustomXamlResourceLoader;
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoader

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    interface ICustomXamlResourceLoaderFactory;
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoaderFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    interface ICustomXamlResourceLoaderOverrides;
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoaderOverrides

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    interface ICustomXamlResourceLoaderStatics;
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoaderStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    class CustomXamlResourceLoader;
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoader[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoader";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    MIDL_INTERFACE("511a84ab-4a88-419f-852e-54083b90b078")
                    ICustomXamlResourceLoader : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ICustomXamlResourceLoader = __uuidof(ICustomXamlResourceLoader);
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoaderFactory[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    MIDL_INTERFACE("5bfd7e49-7886-44f3-8ed3-6fec0463ed69")
                    ICustomXamlResourceLoaderFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoader** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomXamlResourceLoaderFactory = __uuidof(ICustomXamlResourceLoaderFactory);
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoaderOverrides[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    MIDL_INTERFACE("f851e991-af02-46e8-9af8-427b7ebfe9f8")
                    ICustomXamlResourceLoaderOverrides : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetResource(
                            HSTRING resourceId,
                            HSTRING objectType,
                            HSTRING propertyName,
                            HSTRING propertyType,
                            IInspectable** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomXamlResourceLoaderOverrides = __uuidof(ICustomXamlResourceLoaderOverrides);
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoaderStatics[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Resources {
                    MIDL_INTERFACE("224ff617-e4dc-4c27-ad32-db93d5d0e5da")
                    ICustomXamlResourceLoaderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoader** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Current(
                            ABI::Windows::UI::Xaml::Resources::ICustomXamlResourceLoader* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomXamlResourceLoaderStatics = __uuidof(ICustomXamlResourceLoaderStatics);
                } /* Resources */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Resources.ICustomXamlResourceLoader ** Default Interface **
 *    Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Resources_CustomXamlResourceLoader_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Resources_CustomXamlResourceLoader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Resources_CustomXamlResourceLoader[] = L"Windows.UI.Xaml.Resources.CustomXamlResourceLoader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader;

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides;

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoader[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoader";
typedef struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoaderFactory[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoaderOverrides[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides";
typedef struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverridesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetResource)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides* This,
        HSTRING resourceId,
        HSTRING objectType,
        HSTRING propertyName,
        HSTRING propertyType,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverridesVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverridesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_GetResource(This, resourceId, objectType, propertyName, propertyType, result) \
    ((This)->lpVtbl->GetResource(This, resourceId, objectType, propertyName, propertyType, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Resources_ICustomXamlResourceLoaderStatics[] = L"Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader** value);
    HRESULT (STDMETHODCALLTYPE* put_Current)(__x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoader* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_put_Current(This, value) \
    ((This)->lpVtbl->put_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CResources_CICustomXamlResourceLoaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Resources.CustomXamlResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Resources.ICustomXamlResourceLoader ** Default Interface **
 *    Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Resources_CustomXamlResourceLoader_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Resources_CustomXamlResourceLoader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Resources_CustomXamlResourceLoader[] = L"Windows.UI.Xaml.Resources.CustomXamlResourceLoader";
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
#endif // __windows2Eui2Examl2Eresources_p_h__

#endif // __windows2Eui2Examl2Eresources_h__
