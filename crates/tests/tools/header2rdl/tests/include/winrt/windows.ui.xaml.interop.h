
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
#ifndef __windows2Eui2Examl2Einterop_h__
#define __windows2Eui2Examl2Einterop_h__
#ifndef __windows2Eui2Examl2Einterop_p_h__
#define __windows2Eui2Examl2Einterop_p_h__


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
#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface IBindableVectorChangedEventHandler;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler ABI::Windows::UI::Xaml::Interop::IBindableVectorChangedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface INotifyCollectionChangedEventHandler;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler ABI::Windows::UI::Xaml::Interop::INotifyCollectionChangedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface IBindableIterable;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable ABI::Windows::UI::Xaml::Interop::IBindableIterable

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface IBindableIterator;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator ABI::Windows::UI::Xaml::Interop::IBindableIterator

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface IBindableObservableVector;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector ABI::Windows::UI::Xaml::Interop::IBindableObservableVector

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface IBindableVector;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector ABI::Windows::UI::Xaml::Interop::IBindableVector

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface IBindableVectorView;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView ABI::Windows::UI::Xaml::Interop::IBindableVectorView

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface INotifyCollectionChanged;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged ABI::Windows::UI::Xaml::Interop::INotifyCollectionChanged

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface INotifyCollectionChangedEventArgs;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs ABI::Windows::UI::Xaml::Interop::INotifyCollectionChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    interface INotifyCollectionChangedEventArgsFactory;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory ABI::Windows::UI::Xaml::Interop::INotifyCollectionChangedEventArgsFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    typedef enum NotifyCollectionChangedAction : int NotifyCollectionChangedAction;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    typedef enum TypeKind : int TypeKind;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    class NotifyCollectionChangedEventArgs;
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Interop.NotifyCollectionChangedAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    enum NotifyCollectionChangedAction : int
                    {
                        NotifyCollectionChangedAction_Add = 0,
                        NotifyCollectionChangedAction_Remove = 1,
                        NotifyCollectionChangedAction_Replace = 2,
                        NotifyCollectionChangedAction_Move = 3,
                        NotifyCollectionChangedAction_Reset = 4,
                    };
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Interop.TypeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    enum TypeKind : int
                    {
                        TypeKind_Primitive = 0,
                        TypeKind_Metadata = 1,
                        TypeKind_Custom = 2,
                    };
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Interop.TypeName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    struct TypeName
                    {
                        HSTRING Name;
                        ABI::Windows::UI::Xaml::Interop::TypeKind Kind;
                    };
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Interop.BindableVectorChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("624cd4e1-d007-43b1-9c03-af4d3e6258c4")
                    IBindableVectorChangedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            ABI::Windows::UI::Xaml::Interop::IBindableObservableVector* vector,
                            IInspectable* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBindableVectorChangedEventHandler = __uuidof(IBindableVectorChangedEventHandler);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Interop.NotifyCollectionChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("ca10b37c-f382-4591-8557-5e24965279b0")
                    INotifyCollectionChangedEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Interop::INotifyCollectionChangedEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotifyCollectionChangedEventHandler = __uuidof(INotifyCollectionChangedEventHandler);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableIterable[] = L"Windows.UI.Xaml.Interop.IBindableIterable";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("036d2c08-df29-41af-8aa2-d774be62ba6f")
                    IBindableIterable : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE First(
                            ABI::Windows::UI::Xaml::Interop::IBindableIterator** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBindableIterable = __uuidof(IBindableIterable);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableIterator[] = L"Windows.UI.Xaml.Interop.IBindableIterator";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("6a1d6c07-076d-49f2-8314-f52c9c9a8331")
                    IBindableIterator : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HasCurrent(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE MoveNext(
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBindableIterator = __uuidof(IBindableIterator);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableObservableVector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Interop.IBindableVector
 *     Windows.UI.Xaml.Interop.IBindableIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableObservableVector[] = L"Windows.UI.Xaml.Interop.IBindableObservableVector";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("fe1eb536-7e7f-4f90-ac9a-474984aae512")
                    IBindableObservableVector : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_VectorChanged(
                            ABI::Windows::UI::Xaml::Interop::IBindableVectorChangedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_VectorChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBindableObservableVector = __uuidof(IBindableObservableVector);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableVector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Interop.IBindableIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableVector[] = L"Windows.UI.Xaml.Interop.IBindableVector";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("393de7de-6fd0-4c0d-bb71-47244a113e93")
                    IBindableVector : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetAt(
                            UINT32 index,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Size(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetView(
                            ABI::Windows::UI::Xaml::Interop::IBindableVectorView** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IndexOf(
                            IInspectable* value,
                            UINT32* index,
                            boolean* returnValue
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAt(
                            UINT32 index,
                            IInspectable* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE InsertAt(
                            UINT32 index,
                            IInspectable* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveAt(
                            UINT32 index
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Append(
                            IInspectable* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveAtEnd(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBindableVector = __uuidof(IBindableVector);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Interop.IBindableIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableVectorView[] = L"Windows.UI.Xaml.Interop.IBindableVectorView";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("346dd6e7-976e-4bc3-815d-ece243bc0f33")
                    IBindableVectorView : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetAt(
                            UINT32 index,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Size(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IndexOf(
                            IInspectable* value,
                            UINT32* index,
                            boolean* returnValue
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBindableVectorView = __uuidof(IBindableVectorView);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.INotifyCollectionChanged
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_INotifyCollectionChanged[] = L"Windows.UI.Xaml.Interop.INotifyCollectionChanged";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("28b167d5-1a31-465b-9b25-d5c3ae686c40")
                    INotifyCollectionChanged : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_CollectionChanged(
                            ABI::Windows::UI::Xaml::Interop::INotifyCollectionChangedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CollectionChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotifyCollectionChanged = __uuidof(INotifyCollectionChanged);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_INotifyCollectionChangedEventArgs[] = L"Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("4cf68d33-e3f2-4964-b85e-945b4f7e2f21")
                    INotifyCollectionChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Action(
                            ABI::Windows::UI::Xaml::Interop::NotifyCollectionChangedAction* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewItems(
                            ABI::Windows::UI::Xaml::Interop::IBindableVector** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OldItems(
                            ABI::Windows::UI::Xaml::Interop::IBindableVector** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewStartingIndex(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OldStartingIndex(
                            INT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotifyCollectionChangedEventArgs = __uuidof(INotifyCollectionChangedEventArgs);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_INotifyCollectionChangedEventArgsFactory[] = L"Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgsFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Interop {
                    MIDL_INTERFACE("b30c3e3a-df8d-44a5-9a38-7ac0d08ce63d")
                    INotifyCollectionChangedEventArgsFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithAllParameters(
                            ABI::Windows::UI::Xaml::Interop::NotifyCollectionChangedAction action,
                            ABI::Windows::UI::Xaml::Interop::IBindableVector* newItems,
                            ABI::Windows::UI::Xaml::Interop::IBindableVector* oldItems,
                            INT32 newIndex,
                            INT32 oldIndex,
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Interop::INotifyCollectionChangedEventArgs** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotifyCollectionChangedEventArgsFactory = __uuidof(INotifyCollectionChangedEventArgsFactory);
                } /* Interop */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Interop_NotifyCollectionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Interop_NotifyCollectionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Interop_NotifyCollectionChangedEventArgs[] = L"Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CUI_CXaml_CInterop_CNotifyCollectionChangedAction __x_ABI_CWindows_CUI_CXaml_CInterop_CNotifyCollectionChangedAction;

typedef enum __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeKind __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeKind;

/*
 *
 * Struct Windows.UI.Xaml.Interop.NotifyCollectionChangedAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CInterop_CNotifyCollectionChangedAction
{
    NotifyCollectionChangedAction_Add = 0,
    NotifyCollectionChangedAction_Remove = 1,
    NotifyCollectionChangedAction_Replace = 2,
    NotifyCollectionChangedAction_Move = 3,
    NotifyCollectionChangedAction_Reset = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Interop.TypeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeKind
{
    TypeKind_Primitive = 0,
    TypeKind_Metadata = 1,
    TypeKind_Custom = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Interop.TypeName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeName
{
    HSTRING Name;
    enum __x_ABI_CWindows_CUI_CXaml_CInterop_CTypeKind Kind;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Interop.BindableVectorChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* vector,
        IInspectable* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_Invoke(This, vector, e) \
    ((This)->lpVtbl->Invoke(This, vector, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Interop.NotifyCollectionChangedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableIterable[] = L"Windows.UI.Xaml.Interop.IBindableIterable";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterableVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableIterator[] = L"Windows.UI.Xaml.Interop.IBindableIterator";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIteratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIteratorVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIteratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_get_HasCurrent(This, value) \
    ((This)->lpVtbl->get_HasCurrent(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableIterator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableObservableVector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Interop.IBindableVector
 *     Windows.UI.Xaml.Interop.IBindableIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableObservableVector[] = L"Windows.UI.Xaml.Interop.IBindableObservableVector";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_VectorChanged)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorChangedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_VectorChanged)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVectorVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_add_VectorChanged(This, handler, token) \
    ((This)->lpVtbl->add_VectorChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_remove_VectorChanged(This, token) \
    ((This)->lpVtbl->remove_VectorChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableObservableVector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableVector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Interop.IBindableIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableVector[] = L"Windows.UI.Xaml.Interop.IBindableVector";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        UINT32 index,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetView)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        IInspectable* value,
        UINT32* index,
        boolean* returnValue);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        UINT32 index,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        UINT32 index,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_IndexOf(This, value, index, returnValue) \
    ((This)->lpVtbl->IndexOf(This, value, index, returnValue))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.IBindableVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Xaml.Interop.IBindableIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_IBindableVectorView[] = L"Windows.UI.Xaml.Interop.IBindableVectorView";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        UINT32 index,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView* This,
        IInspectable* value,
        UINT32* index,
        boolean* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorViewVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_IndexOf(This, value, index, returnValue) \
    ((This)->lpVtbl->IndexOf(This, value, index, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVectorView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.INotifyCollectionChanged
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_INotifyCollectionChanged[] = L"Windows.UI.Xaml.Interop.INotifyCollectionChanged";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CollectionChanged)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CollectionChanged)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_add_CollectionChanged(This, handler, token) \
    ((This)->lpVtbl->add_CollectionChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_remove_CollectionChanged(This, token) \
    ((This)->lpVtbl->remove_CollectionChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChanged_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_INotifyCollectionChangedEventArgs[] = L"Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInterop_CNotifyCollectionChangedAction* value);
    HRESULT (STDMETHODCALLTYPE* get_NewItems)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector** value);
    HRESULT (STDMETHODCALLTYPE* get_OldItems)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector** value);
    HRESULT (STDMETHODCALLTYPE* get_NewStartingIndex)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_OldStartingIndex)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_get_NewItems(This, value) \
    ((This)->lpVtbl->get_NewItems(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_get_OldItems(This, value) \
    ((This)->lpVtbl->get_OldItems(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_get_NewStartingIndex(This, value) \
    ((This)->lpVtbl->get_NewStartingIndex(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_get_OldStartingIndex(This, value) \
    ((This)->lpVtbl->get_OldStartingIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Interop_INotifyCollectionChangedEventArgsFactory[] = L"Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgsFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithAllParameters)(__x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInterop_CNotifyCollectionChangedAction action,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* newItems,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CIBindableVector* oldItems,
        INT32 newIndex,
        INT32 oldIndex,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgs** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_CreateInstanceWithAllParameters(This, action, newItems, oldItems, newIndex, oldIndex, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstanceWithAllParameters(This, action, newItems, oldItems, newIndex, oldIndex, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CInterop_CINotifyCollectionChangedEventArgsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Interop_NotifyCollectionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Interop_NotifyCollectionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Interop_NotifyCollectionChangedEventArgs[] = L"Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
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
#endif // __windows2Eui2Examl2Einterop_p_h__

#endif // __windows2Eui2Examl2Einterop_h__
