
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
#ifndef __windows2Eui2Epopups_h__
#define __windows2Eui2Epopups_h__
#ifndef __windows2Eui2Epopups_p_h__
#define __windows2Eui2Epopups_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommandInvokedHandler;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler ABI::Windows::UI::Popups::IUICommandInvokedHandler

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IMessageDialog;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog ABI::Windows::UI::Popups::IMessageDialog

#endif // ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IMessageDialogFactory;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory ABI::Windows::UI::Popups::IMessageDialogFactory

#endif // ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IPopupMenu;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu ABI::Windows::UI::Popups::IPopupMenu

#endif // ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommand;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommand ABI::Windows::UI::Popups::IUICommand

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommandFactory;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory ABI::Windows::UI::Popups::IUICommandFactory

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8770535-6a4b-52b1-b578-f3cdc5007a1f"))
IAsyncOperation<ABI::Windows::UI::Popups::IUICommand*> : IAsyncOperation_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Popups::IUICommand*> __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dd33fd5b-a24d-5a44-91fe-dd6441770103"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Popups::IUICommand*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Popups::IUICommand*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIIterator_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f45db3d3-7299-57ce-a73e-297cf0af3083"))
IIterator<ABI::Windows::UI::Popups::IUICommand*> : IIterator_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Popups::IUICommand*> __FIIterator_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIIterable_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e63de42b-53c3-5e07-90d3-98172d545412"))
IIterable<ABI::Windows::UI::Popups::IUICommand*> : IIterable_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Popups::IUICommand*> __FIIterable_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIVectorView_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ed1165e6-f377-5b00-8172-93c1bd04deb4"))
IVectorView<ABI::Windows::UI::Popups::IUICommand*> : IVectorView_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Popups::IUICommand*> __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIVector_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("105139a1-dcb8-5f65-97ef-cb1bf0b75f9d"))
IVector<ABI::Windows::UI::Popups::IUICommand*> : IVector_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Popups::IUICommand*> __FIVector_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIVector_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace UI {
            namespace Popups {
                typedef enum MessageDialogOptions : unsigned int MessageDialogOptions;
            } /* Popups */
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
            namespace Popups {
                class MessageDialog;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                class UICommand;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Popups.MessageDialogOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                enum MessageDialogOptions : unsigned int
                {
                    MessageDialogOptions_None = 0,
                    MessageDialogOptions_AcceptUserInputAfterDelay = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(MessageDialogOptions)
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Popups.Placement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                enum Placement : int
                {
                    Placement_Default = 0,
                    Placement_Above = 1,
                    Placement_Below = 2,
                    Placement_Left = 3,
                    Placement_Right = 4,
                };
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Popups.UICommandInvokedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                MIDL_INTERFACE("daf77a4f-c27a-4298-9ac6-2922c45e7da6")
                IUICommandInvokedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::UI::Popups::IUICommand* command
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUICommandInvokedHandler = __uuidof(IUICommandInvokedHandler);
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IMessageDialog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.MessageDialog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IMessageDialog[] = L"Windows.UI.Popups.IMessageDialog";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                MIDL_INTERFACE("33f59b01-5325-43ab-9ab3-bdae440e4121")
                IMessageDialog : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Commands(
                        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultCommandIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultCommandIndex(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CancelCommandIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CancelCommandIndex(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Content(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** messageDialogAsyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Options(
                        ABI::Windows::UI::Popups::MessageDialogOptions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Options(
                        ABI::Windows::UI::Popups::MessageDialogOptions value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageDialog = __uuidof(IMessageDialog);
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIMessageDialog;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IMessageDialogFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.MessageDialog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IMessageDialogFactory[] = L"Windows.UI.Popups.IMessageDialogFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                MIDL_INTERFACE("2d161777-a66f-4ea5-bb87-793ffa4941f2")
                IMessageDialogFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING content,
                        ABI::Windows::UI::Popups::IMessageDialog** messageDialog
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTitle(
                        HSTRING content,
                        HSTRING title,
                        ABI::Windows::UI::Popups::IMessageDialog** messageDialog
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageDialogFactory = __uuidof(IMessageDialogFactory);
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IPopupMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.PopupMenu
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IPopupMenu[] = L"Windows.UI.Popups.IPopupMenu";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                MIDL_INTERFACE("4e9bc6dc-880d-47fc-a0a1-72b639e62559")
                IPopupMenu : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Commands(
                        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        ABI::Windows::Foundation::Point invocationPoint,
                        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsyncWithRect(
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsyncWithRectAndPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPopupMenu = __uuidof(IPopupMenu);
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIPopupMenu;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IUICommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IUICommand[] = L"Windows.UI.Popups.IUICommand";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                MIDL_INTERFACE("4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f")
                IUICommand : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Label(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Invoked(
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Invoked(
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Id(
                        IInspectable* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUICommand = __uuidof(IUICommand);
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIUICommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IUICommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.UICommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IUICommandFactory[] = L"Windows.UI.Popups.IUICommandFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                MIDL_INTERFACE("a21a8189-26b0-4676-ae94-54041bc125e8")
                IUICommandFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING label,
                        ABI::Windows::UI::Popups::IUICommand** instance
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithHandler(
                        HSTRING label,
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler* action,
                        ABI::Windows::UI::Popups::IUICommand** instance
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithHandlerAndId(
                        HSTRING label,
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler* action,
                        IInspectable* commandId,
                        ABI::Windows::UI::Popups::IUICommand** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUICommandFactory = __uuidof(IUICommandFactory);
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIUICommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.MessageDialog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Popups.IMessageDialogFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IMessageDialog ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_MessageDialog_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_MessageDialog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_MessageDialog[] = L"Windows.UI.Popups.MessageDialog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.PopupMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IPopupMenu ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_PopupMenu_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_PopupMenu_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_PopupMenu[] = L"Windows.UI.Popups.PopupMenu";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.UICommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Popups.IUICommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IUICommand ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_UICommand_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_UICommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_UICommand[] = L"Windows.UI.Popups.UICommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.UICommandSeparator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IUICommand ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_UICommandSeparator_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_UICommandSeparator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_UICommandSeparator[] = L"Windows.UI.Popups.UICommandSeparator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIMessageDialog __x_ABI_CWindows_CUI_CPopups_CIMessageDialog;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIPopupMenu __x_ABI_CWindows_CUI_CPopups_CIPopupMenu;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommand __x_ABI_CWindows_CUI_CPopups_CIUICommand;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CPopups__CIUICommand __FIIterator_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIIterator_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIIterator_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CPopups__CIUICommand __FIIterable_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIIterable_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIIterator_1_Windows__CUI__CPopups__CIUICommand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIIterable_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CPopups__CIUICommand __FIVectorView_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIVectorView_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIVectorView_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CPopups__CIUICommand __FIVector_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIVector_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIVectorView_1_Windows__CUI__CPopups__CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIVector_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef enum __x_ABI_CWindows_CUI_CPopups_CMessageDialogOptions __x_ABI_CWindows_CUI_CPopups_CMessageDialogOptions;

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

/*
 *
 * Struct Windows.UI.Popups.MessageDialogOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CPopups_CMessageDialogOptions
{
    MessageDialogOptions_None = 0,
    MessageDialogOptions_AcceptUserInputAfterDelay = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Popups.Placement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CPopups_CPlacement
{
    Placement_Default = 0,
    Placement_Above = 1,
    Placement_Below = 2,
    Placement_Left = 3,
    Placement_Right = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Popups.UICommandInvokedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* command);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandlerVtbl;

interface __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_Invoke(This, command) \
    ((This)->lpVtbl->Invoke(This, command))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IMessageDialog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.MessageDialog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IMessageDialog[] = L"Windows.UI.Popups.IMessageDialog";
typedef struct __x_ABI_CWindows_CUI_CPopups_CIMessageDialogVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Commands)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultCommandIndex)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultCommandIndex)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_CancelCommandIndex)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_CancelCommandIndex)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Content)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** messageDialogAsyncOperation);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        enum __x_ABI_CWindows_CUI_CPopups_CMessageDialogOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_Options)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialog* This,
        enum __x_ABI_CWindows_CUI_CPopups_CMessageDialogOptions value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CPopups_CIMessageDialogVtbl;

interface __x_ABI_CWindows_CUI_CPopups_CIMessageDialog
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CPopups_CIMessageDialogVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_get_Commands(This, value) \
    ((This)->lpVtbl->get_Commands(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_get_DefaultCommandIndex(This, value) \
    ((This)->lpVtbl->get_DefaultCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_put_DefaultCommandIndex(This, value) \
    ((This)->lpVtbl->put_DefaultCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_get_CancelCommandIndex(This, value) \
    ((This)->lpVtbl->get_CancelCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_put_CancelCommandIndex(This, value) \
    ((This)->lpVtbl->put_CancelCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_put_Content(This, value) \
    ((This)->lpVtbl->put_Content(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_ShowAsync(This, messageDialogAsyncOperation) \
    ((This)->lpVtbl->ShowAsync(This, messageDialogAsyncOperation))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialog_put_Options(This, value) \
    ((This)->lpVtbl->put_Options(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIMessageDialog;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IMessageDialogFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.MessageDialog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IMessageDialogFactory[] = L"Windows.UI.Popups.IMessageDialogFactory";
typedef struct __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This,
        HSTRING content,
        __x_ABI_CWindows_CUI_CPopups_CIMessageDialog** messageDialog);
    HRESULT (STDMETHODCALLTYPE* CreateWithTitle)(__x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory* This,
        HSTRING content,
        HSTRING title,
        __x_ABI_CWindows_CUI_CPopups_CIMessageDialog** messageDialog);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactoryVtbl;

interface __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_Create(This, content, messageDialog) \
    ((This)->lpVtbl->Create(This, content, messageDialog))

#define __x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_CreateWithTitle(This, content, title, messageDialog) \
    ((This)->lpVtbl->CreateWithTitle(This, content, title, messageDialog))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIMessageDialogFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IPopupMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.PopupMenu
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IPopupMenu[] = L"Windows.UI.Popups.IPopupMenu";
typedef struct __x_ABI_CWindows_CUI_CPopups_CIPopupMenuVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Commands)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value);
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        struct __x_ABI_CWindows_CFoundation_CPoint invocationPoint,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* ShowAsyncWithRect)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* ShowAsyncWithRectAndPlacement)(__x_ABI_CWindows_CUI_CPopups_CIPopupMenu* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncOperation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CPopups_CIPopupMenuVtbl;

interface __x_ABI_CWindows_CUI_CPopups_CIPopupMenu
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CPopups_CIPopupMenuVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_get_Commands(This, value) \
    ((This)->lpVtbl->get_Commands(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_ShowAsync(This, invocationPoint, asyncOperation) \
    ((This)->lpVtbl->ShowAsync(This, invocationPoint, asyncOperation))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_ShowAsyncWithRect(This, selection, asyncOperation) \
    ((This)->lpVtbl->ShowAsyncWithRect(This, selection, asyncOperation))

#define __x_ABI_CWindows_CUI_CPopups_CIPopupMenu_ShowAsyncWithRectAndPlacement(This, selection, preferredPlacement, asyncOperation) \
    ((This)->lpVtbl->ShowAsyncWithRectAndPlacement(This, selection, preferredPlacement, asyncOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIPopupMenu;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIPopupMenu_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IUICommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IUICommand[] = L"Windows.UI.Popups.IUICommand";
typedef struct __x_ABI_CWindows_CUI_CPopups_CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Label)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Invoked)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler** value);
    HRESULT (STDMETHODCALLTYPE* put_Invoked)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CUI_CPopups_CIUICommand* This,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CPopups_CIUICommandVtbl;

interface __x_ABI_CWindows_CUI_CPopups_CIUICommand
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CPopups_CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_put_Label(This, value) \
    ((This)->lpVtbl->put_Label(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_get_Invoked(This, value) \
    ((This)->lpVtbl->get_Invoked(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_put_Invoked(This, value) \
    ((This)->lpVtbl->put_Invoked(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommand_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIUICommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Popups.IUICommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Popups.UICommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Popups_IUICommandFactory[] = L"Windows.UI.Popups.IUICommandFactory";
typedef struct __x_ABI_CWindows_CUI_CPopups_CIUICommandFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** instance);
    HRESULT (STDMETHODCALLTYPE* CreateWithHandler)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* action,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** instance);
    HRESULT (STDMETHODCALLTYPE* CreateWithHandlerAndId)(__x_ABI_CWindows_CUI_CPopups_CIUICommandFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* action,
        IInspectable* commandId,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CPopups_CIUICommandFactoryVtbl;

interface __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CPopups_CIUICommandFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_Create(This, label, instance) \
    ((This)->lpVtbl->Create(This, label, instance))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_CreateWithHandler(This, label, action, instance) \
    ((This)->lpVtbl->CreateWithHandler(This, label, action, instance))

#define __x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_CreateWithHandlerAndId(This, label, action, commandId, instance) \
    ((This)->lpVtbl->CreateWithHandlerAndId(This, label, action, commandId, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CPopups_CIUICommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CPopups_CIUICommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.MessageDialog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Popups.IMessageDialogFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IMessageDialog ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_MessageDialog_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_MessageDialog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_MessageDialog[] = L"Windows.UI.Popups.MessageDialog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.PopupMenu
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IPopupMenu ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_PopupMenu_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_PopupMenu_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_PopupMenu[] = L"Windows.UI.Popups.PopupMenu";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.UICommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Popups.IUICommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IUICommand ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_UICommand_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_UICommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_UICommand[] = L"Windows.UI.Popups.UICommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Popups.UICommandSeparator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IUICommand ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Popups_UICommandSeparator_DEFINED
#define RUNTIMECLASS_Windows_UI_Popups_UICommandSeparator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Popups_UICommandSeparator[] = L"Windows.UI.Popups.UICommandSeparator";
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
#endif // __windows2Eui2Epopups_p_h__

#endif // __windows2Eui2Epopups_h__
