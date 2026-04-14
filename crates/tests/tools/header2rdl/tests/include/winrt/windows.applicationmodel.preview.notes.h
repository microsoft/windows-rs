
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
#ifndef __windows2Eapplicationmodel2Epreview2Enotes_h__
#define __windows2Eapplicationmodel2Epreview2Enotes_h__
#ifndef __windows2Eapplicationmodel2Epreview2Enotes_p_h__
#define __windows2Eapplicationmodel2Epreview2Enotes_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION)

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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    interface INotePlacementChangedPreviewEventArgs;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs ABI::Windows::ApplicationModel::Preview::Notes::INotePlacementChangedPreviewEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    interface INoteVisibilityChangedPreviewEventArgs;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs ABI::Windows::ApplicationModel::Preview::Notes::INoteVisibilityChangedPreviewEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    interface INotesWindowManagerPreview;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreview

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    interface INotesWindowManagerPreview2;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2 ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreview2

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    interface INotesWindowManagerPreviewShowNoteOptions;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreviewShowNoteOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    interface INotesWindowManagerPreviewStatics;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreviewStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    class NotesWindowManagerPreview;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6f2755fb-6c33-543c-9ab4-de486bc7bfe2"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreview*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_USE */

#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    class NotePlacementChangedPreviewEventArgs;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a28af2c7-4012-5999-a322-5236b30d995f"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::NotePlacementChangedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::Notes::NotePlacementChangedPreviewEventArgs*, ABI::Windows::ApplicationModel::Preview::Notes::INotePlacementChangedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview, Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::NotePlacementChangedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    class NoteVisibilityChangedPreviewEventArgs;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("16d825c8-5271-51c8-a00f-0cfb1b029ab6"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::NoteVisibilityChangedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Preview::Notes::NoteVisibilityChangedPreviewEventArgs*, ABI::Windows::ApplicationModel::Preview::Notes::INoteVisibilityChangedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview, Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Preview::Notes::NotesWindowManagerPreview*, ABI::Windows::ApplicationModel::Preview::Notes::NoteVisibilityChangedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

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
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap ABI::Windows::Graphics::Imaging::ISoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    class NotesWindowManagerPreviewShowNoteOptions;
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotePlacementChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    MIDL_INTERFACE("491d57b7-f780-4e7f-a939-9a4caf965214")
                    INotePlacementChangedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ViewId(
                            INT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotePlacementChangedPreviewEventArgs = __uuidof(INotePlacementChangedPreviewEventArgs);
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INoteVisibilityChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    MIDL_INTERFACE("0e34649e-3815-4ff6-83b3-a14d17120e24")
                    INoteVisibilityChangedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ViewId(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsVisible(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INoteVisibilityChangedPreviewEventArgs = __uuidof(INoteVisibilityChangedPreviewEventArgs);
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreview[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    MIDL_INTERFACE("dc2ac23e-4850-4f13-9cc7-ff487efdfcde")
                    INotesWindowManagerPreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsScreenLocked(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ShowNote(
                            INT32 noteViewId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ShowNoteRelativeTo(
                            INT32 noteViewId,
                            INT32 anchorNoteViewId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ShowNoteWithPlacement(
                            INT32 noteViewId,
                            ABI::Windows::Storage::Streams::IBuffer* data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE HideNote(
                            INT32 noteViewId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNotePlacement(
                            INT32 noteViewId,
                            ABI::Windows::Storage::Streams::IBuffer** data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TrySetNoteSize(
                            INT32 noteViewId,
                            ABI::Windows::Foundation::Size size,
                            boolean* succeeded
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetFocusToNextView(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetNotesThumbnailAsync(
                            ABI::Windows::Storage::Streams::IBuffer* thumbnail,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SystemLockStateChanged(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SystemLockStateChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NotePlacementChanged(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NotePlacementChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NoteVisibilityChanged(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NoteVisibilityChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotesWindowManagerPreview = __uuidof(INotesWindowManagerPreview);
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreview2[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    MIDL_INTERFACE("edfe864a-1f54-4b09-9823-ff477f6fa3bc")
                    INotesWindowManagerPreview2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ShowNoteRelativeToWithOptions(
                            INT32 noteViewId,
                            INT32 anchorNoteViewId,
                            ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreviewShowNoteOptions* options
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ShowNoteWithPlacementWithOptions(
                            INT32 noteViewId,
                            ABI::Windows::Storage::Streams::IBuffer* data,
                            ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreviewShowNoteOptions* options
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetFocusToPreviousView(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetThumbnailImageForTaskSwitcherAsync(
                            ABI::Windows::Graphics::Imaging::ISoftwareBitmap* bitmap,
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotesWindowManagerPreview2 = __uuidof(INotesWindowManagerPreview2);
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreviewShowNoteOptions[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    MIDL_INTERFACE("886b09d6-a6ae-4007-a56d-1ca70c84c0d2")
                    INotesWindowManagerPreviewShowNoteOptions : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ShowWithFocus(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ShowWithFocus(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotesWindowManagerPreviewShowNoteOptions = __uuidof(INotesWindowManagerPreviewShowNoteOptions);
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreviewStatics[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Notes {
                    MIDL_INTERFACE("6668cc88-0a8e-4127-a38e-995445868a78")
                    INotesWindowManagerPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentApp(
                            ABI::Windows::ApplicationModel::Preview::Notes::INotesWindowManagerPreview** current
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INotesWindowManagerPreviewStatics = __uuidof(INotesWindowManagerPreviewStatics);
                } /* Notes */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotePlacementChangedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotePlacementChangedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NotePlacementChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NoteVisibilityChangedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NoteVisibilityChangedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NoteVisibilityChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics interface starting with version 1.0 of the Windows.ApplicationModel.Preview.Notes.PreviewNotesContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview ** Default Interface **
 *    Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreview[] = L"Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.ApplicationModel.Preview.Notes.PreviewNotesContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreviewShowNoteOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreviewShowNoteOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreviewShowNoteOptions[] = L"Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2 __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* sender,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* sender,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotePlacementChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewId)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_get_ViewId(This, value) \
    ((This)->lpVtbl->get_ViewId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotePlacementChangedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INoteVisibilityChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewId)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVisible)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_get_ViewId(This, value) \
    ((This)->lpVtbl->get_ViewId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_get_IsVisible(This, value) \
    ((This)->lpVtbl->get_IsVisible(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINoteVisibilityChangedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreview[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsScreenLocked)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ShowNote)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        INT32 noteViewId);
    HRESULT (STDMETHODCALLTYPE* ShowNoteRelativeTo)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        INT32 noteViewId,
        INT32 anchorNoteViewId);
    HRESULT (STDMETHODCALLTYPE* ShowNoteWithPlacement)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        INT32 noteViewId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data);
    HRESULT (STDMETHODCALLTYPE* HideNote)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        INT32 noteViewId);
    HRESULT (STDMETHODCALLTYPE* GetNotePlacement)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        INT32 noteViewId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** data);
    HRESULT (STDMETHODCALLTYPE* TrySetNoteSize)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        INT32 noteViewId,
        struct __x_ABI_CWindows_CFoundation_CSize size,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* SetFocusToNextView)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This);
    HRESULT (STDMETHODCALLTYPE* SetNotesThumbnailAsync)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* thumbnail,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* add_SystemLockStateChanged)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SystemLockStateChanged)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NotePlacementChanged)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNotePlacementChangedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NotePlacementChanged)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NoteVisibilityChanged)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPreview__CNotes__CNotesWindowManagerPreview_Windows__CApplicationModel__CPreview__CNotes__CNoteVisibilityChangedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NoteVisibilityChanged)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_get_IsScreenLocked(This, value) \
    ((This)->lpVtbl->get_IsScreenLocked(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_ShowNote(This, noteViewId) \
    ((This)->lpVtbl->ShowNote(This, noteViewId))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_ShowNoteRelativeTo(This, noteViewId, anchorNoteViewId) \
    ((This)->lpVtbl->ShowNoteRelativeTo(This, noteViewId, anchorNoteViewId))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_ShowNoteWithPlacement(This, noteViewId, data) \
    ((This)->lpVtbl->ShowNoteWithPlacement(This, noteViewId, data))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_HideNote(This, noteViewId) \
    ((This)->lpVtbl->HideNote(This, noteViewId))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_GetNotePlacement(This, noteViewId, data) \
    ((This)->lpVtbl->GetNotePlacement(This, noteViewId, data))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_TrySetNoteSize(This, noteViewId, size, succeeded) \
    ((This)->lpVtbl->TrySetNoteSize(This, noteViewId, size, succeeded))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_SetFocusToNextView(This) \
    ((This)->lpVtbl->SetFocusToNextView(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_SetNotesThumbnailAsync(This, thumbnail, operation) \
    ((This)->lpVtbl->SetNotesThumbnailAsync(This, thumbnail, operation))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_add_SystemLockStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_SystemLockStateChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_remove_SystemLockStateChanged(This, token) \
    ((This)->lpVtbl->remove_SystemLockStateChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_add_NotePlacementChanged(This, handler, token) \
    ((This)->lpVtbl->add_NotePlacementChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_remove_NotePlacementChanged(This, token) \
    ((This)->lpVtbl->remove_NotePlacementChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_add_NoteVisibilityChanged(This, handler, token) \
    ((This)->lpVtbl->add_NoteVisibilityChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_remove_NoteVisibilityChanged(This, token) \
    ((This)->lpVtbl->remove_NoteVisibilityChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreview2[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowNoteRelativeToWithOptions)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        INT32 noteViewId,
        INT32 anchorNoteViewId,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* options);
    HRESULT (STDMETHODCALLTYPE* ShowNoteWithPlacementWithOptions)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        INT32 noteViewId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* options);
    HRESULT (STDMETHODCALLTYPE* SetFocusToPreviousView)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This);
    HRESULT (STDMETHODCALLTYPE* SetThumbnailImageForTaskSwitcherAsync)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* bitmap,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_ShowNoteRelativeToWithOptions(This, noteViewId, anchorNoteViewId, options) \
    ((This)->lpVtbl->ShowNoteRelativeToWithOptions(This, noteViewId, anchorNoteViewId, options))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_ShowNoteWithPlacementWithOptions(This, noteViewId, data, options) \
    ((This)->lpVtbl->ShowNoteWithPlacementWithOptions(This, noteViewId, data, options))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_SetFocusToPreviousView(This) \
    ((This)->lpVtbl->SetFocusToPreviousView(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_SetThumbnailImageForTaskSwitcherAsync(This, bitmap, action) \
    ((This)->lpVtbl->SetThumbnailImageForTaskSwitcherAsync(This, bitmap, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreviewShowNoteOptions[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ShowWithFocus)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShowWithFocus)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_get_ShowWithFocus(This, value) \
    ((This)->lpVtbl->get_ShowWithFocus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_put_ShowWithFocus(This, value) \
    ((This)->lpVtbl->put_ShowWithFocus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewShowNoteOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Notes_INotesWindowManagerPreviewStatics[] = L"Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentApp)(__x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreview** current);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_GetForCurrentApp(This, current) \
    ((This)->lpVtbl->GetForCurrentApp(This, current))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CNotes_CINotesWindowManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotePlacementChangedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotePlacementChangedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NotePlacementChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NoteVisibilityChangedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NoteVisibilityChangedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NoteVisibilityChangedPreviewEventArgs[] = L"Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics interface starting with version 1.0 of the Windows.ApplicationModel.Preview.Notes.PreviewNotesContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview ** Default Interface **
 *    Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreview[] = L"Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions
 *
 * Introduced to Windows.ApplicationModel.Preview.Notes.PreviewNotesContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.ApplicationModel.Preview.Notes.PreviewNotesContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreviewShowNoteOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreviewShowNoteOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Notes_NotesWindowManagerPreviewShowNoteOptions[] = L"Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_PREVIEW_NOTES_PREVIEWNOTESCONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Epreview2Enotes_p_h__

#endif // __windows2Eapplicationmodel2Epreview2Enotes_h__
