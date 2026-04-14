
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
#ifndef __windows2Eapplicationmodel2Eresources_h__
#define __windows2Eapplicationmodel2Eresources_h__
#ifndef __windows2Eapplicationmodel2Eresources_p_h__
#define __windows2Eapplicationmodel2Eresources_p_h__


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
#include "Windows.UI.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoader;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader ABI::Windows::ApplicationModel::Resources::IResourceLoader

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoader2;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2 ABI::Windows::ApplicationModel::Resources::IResourceLoader2

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoaderFactory;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory ABI::Windows::ApplicationModel::Resources::IResourceLoaderFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoaderStatics;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics ABI::Windows::ApplicationModel::Resources::IResourceLoaderStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoaderStatics2;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2 ABI::Windows::ApplicationModel::Resources::IResourceLoaderStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoaderStatics3;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3 ABI::Windows::ApplicationModel::Resources::IResourceLoaderStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                interface IResourceLoaderStatics4;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4 ABI::Windows::ApplicationModel::Resources::IResourceLoaderStatics4

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
            class UIContext;
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IUIContext;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIUIContext ABI::Windows::UI::IUIContext

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                class ResourceLoader;
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoader[] = L"Windows.ApplicationModel.Resources.IResourceLoader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("08524908-16ef-45ad-a602-293637d7e61a")
                IResourceLoader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetString(
                        HSTRING resource,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoader = __uuidof(IResourceLoader);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoader2[] = L"Windows.ApplicationModel.Resources.IResourceLoader2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("10eb6ec6-8138-48c1-bc65-e1f14207367c")
                IResourceLoader2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStringForUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoader2 = __uuidof(IResourceLoader2);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderFactory[] = L"Windows.ApplicationModel.Resources.IResourceLoaderFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("c33a3603-69dc-4285-a077-d5c0e47ccbe8")
                IResourceLoaderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateResourceLoaderByName(
                        HSTRING name,
                        ABI::Windows::ApplicationModel::Resources::IResourceLoader** loader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoaderFactory = __uuidof(IResourceLoaderFactory);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("bf777ce1-19c8-49c2-953c-47e9227b334e")
                IResourceLoaderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStringForReference(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoaderStatics = __uuidof(IResourceLoaderStatics);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics2[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("0cc04141-6466-4989-9494-0b82dfc53f1f")
                IResourceLoaderStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::ApplicationModel::Resources::IResourceLoader** loader
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentViewWithName(
                        HSTRING name,
                        ABI::Windows::ApplicationModel::Resources::IResourceLoader** loader
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForViewIndependentUse(
                        ABI::Windows::ApplicationModel::Resources::IResourceLoader** loader
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForViewIndependentUseWithName(
                        HSTRING name,
                        ABI::Windows::ApplicationModel::Resources::IResourceLoader** loader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoaderStatics2 = __uuidof(IResourceLoaderStatics2);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics3[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("64609dfb-64ac-491b-8100-0e558d61c1d0")
                IResourceLoaderStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUIContext(
                        ABI::Windows::UI::IUIContext* context,
                        ABI::Windows::ApplicationModel::Resources::IResourceLoader** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoaderStatics3 = __uuidof(IResourceLoaderStatics3);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics4[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                MIDL_INTERFACE("9fb36c32-6c8c-4316-962e-909539b5c259")
                IResourceLoaderStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultPriPath(
                        HSTRING packageFullName,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResourceLoaderStatics4 = __uuidof(IResourceLoaderStatics4);
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.ApplicationModel.Resources.ResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Resources.IResourceLoaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics3 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics4 interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.IResourceLoader ** Default Interface **
 *    Windows.ApplicationModel.Resources.IResourceLoader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_ResourceLoader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_ResourceLoader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_ResourceLoader[] = L"Windows.ApplicationModel.Resources.ResourceLoader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2 __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2 __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3 __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4 __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoader[] = L"Windows.ApplicationModel.Resources.IResourceLoader";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetString)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader* This,
        HSTRING resource,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_GetString(This, resource, value) \
    ((This)->lpVtbl->GetString(This, resource, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoader2[] = L"Windows.ApplicationModel.Resources.IResourceLoader2";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStringForUri)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_GetStringForUri(This, uri, value) \
    ((This)->lpVtbl->GetStringForUri(This, uri, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderFactory[] = L"Windows.ApplicationModel.Resources.IResourceLoaderFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateResourceLoaderByName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader** loader);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_CreateResourceLoaderByName(This, name, loader) \
    ((This)->lpVtbl->CreateResourceLoaderByName(This, name, loader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStringForReference)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_GetStringForReference(This, uri, value) \
    ((This)->lpVtbl->GetStringForReference(This, uri, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics2[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader** loader);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentViewWithName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        HSTRING name,
        __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader** loader);
    HRESULT (STDMETHODCALLTYPE* GetForViewIndependentUse)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader** loader);
    HRESULT (STDMETHODCALLTYPE* GetForViewIndependentUseWithName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2* This,
        HSTRING name,
        __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader** loader);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetForCurrentView(This, loader) \
    ((This)->lpVtbl->GetForCurrentView(This, loader))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetForCurrentViewWithName(This, name, loader) \
    ((This)->lpVtbl->GetForCurrentViewWithName(This, name, loader))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetForViewIndependentUse(This, loader) \
    ((This)->lpVtbl->GetForViewIndependentUse(This, loader))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_GetForViewIndependentUseWithName(This, name, loader) \
    ((This)->lpVtbl->GetForViewIndependentUseWithName(This, name, loader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics3[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUIContext)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3* This,
        __x_ABI_CWindows_CUI_CIUIContext* context,
        __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoader** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_GetForUIContext(This, context, result) \
    ((This)->lpVtbl->GetForUIContext(This, context, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Resources.IResourceLoaderStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.ResourceLoader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_IResourceLoaderStatics4[] = L"Windows.ApplicationModel.Resources.IResourceLoaderStatics4";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefaultPriPath)(__x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4* This,
        HSTRING packageFullName,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_GetDefaultPriPath(This, packageFullName, value) \
    ((This)->lpVtbl->GetDefaultPriPath(This, packageFullName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CIResourceLoaderStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.ApplicationModel.Resources.ResourceLoader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Resources.IResourceLoaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics3 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.IResourceLoaderStatics4 interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.IResourceLoader ** Default Interface **
 *    Windows.ApplicationModel.Resources.IResourceLoader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_ResourceLoader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_ResourceLoader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_ResourceLoader[] = L"Windows.ApplicationModel.Resources.ResourceLoader";
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
#endif // __windows2Eapplicationmodel2Eresources_p_h__

#endif // __windows2Eapplicationmodel2Eresources_h__
