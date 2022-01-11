#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IRichEditOleImpl: Sized {
    fn GetClientSite();
    fn GetObjectCount();
    fn GetLinkCount();
    fn GetObject();
    fn InsertObject();
    fn ConvertObject();
    fn ActivateAs();
    fn SetHostNames();
    fn SetLinkAvailable();
    fn SetDvaspect();
    fn HandsOffStorage();
    fn SaveCompleted();
    fn InPlaceDeactivate();
    fn ContextSensitiveHelp();
    fn GetClipboardData();
    fn ImportDataObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IRichEditOleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditOleVtbl {
        unsafe extern "system" fn GetClientSite<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpolesite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectCount<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinkCount<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpreobject: *mut REOBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActivateAs<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHostNames<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstrcontainerapp: super::super::super::Foundation::PSTR, lpstrcontainerobj: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLinkAvailable<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDvaspect<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, dvaspect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandsOffStorage<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceDeactivate<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipboardData<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportDataObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, cf: u16, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetClientSite::<Impl, IMPL_OFFSET>,
            GetObjectCount::<Impl, IMPL_OFFSET>,
            GetLinkCount::<Impl, IMPL_OFFSET>,
            GetObject::<Impl, IMPL_OFFSET>,
            InsertObject::<Impl, IMPL_OFFSET>,
            ConvertObject::<Impl, IMPL_OFFSET>,
            ActivateAs::<Impl, IMPL_OFFSET>,
            SetHostNames::<Impl, IMPL_OFFSET>,
            SetLinkAvailable::<Impl, IMPL_OFFSET>,
            SetDvaspect::<Impl, IMPL_OFFSET>,
            HandsOffStorage::<Impl, IMPL_OFFSET>,
            SaveCompleted::<Impl, IMPL_OFFSET>,
            InPlaceDeactivate::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            GetClipboardData::<Impl, IMPL_OFFSET>,
            ImportDataObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditOle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IRichEditOleCallbackImpl: Sized {
    fn GetNewStorage();
    fn GetInPlaceContext();
    fn ShowContainerUI();
    fn QueryInsertObject();
    fn DeleteObject();
    fn QueryAcceptData();
    fn ContextSensitiveHelp();
    fn GetClipboardData();
    fn GetDragDropEffect();
    fn GetContextMenu();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IRichEditOleCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditOleCallbackVtbl {
        unsafe extern "system" fn GetNewStorage<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInPlaceContext<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpframe: *mut ::windows::core::RawPtr, lplpdoc: *mut ::windows::core::RawPtr, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowContainerUI<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryInsertObject<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID, lpstg: ::windows::core::RawPtr, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteObject<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpoleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAcceptData<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipboardData<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDragDropEffect<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContextMenu<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::windows::core::RawPtr, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetNewStorage::<Impl, IMPL_OFFSET>,
            GetInPlaceContext::<Impl, IMPL_OFFSET>,
            ShowContainerUI::<Impl, IMPL_OFFSET>,
            QueryInsertObject::<Impl, IMPL_OFFSET>,
            DeleteObject::<Impl, IMPL_OFFSET>,
            QueryAcceptData::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            GetClipboardData::<Impl, IMPL_OFFSET>,
            GetDragDropEffect::<Impl, IMPL_OFFSET>,
            GetContextMenu::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditOleCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRicheditUiaOverridesImpl: Sized {
    fn GetPropertyOverrideValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRicheditUiaOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditUiaOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRicheditUiaOverridesVtbl {
        unsafe extern "system" fn GetPropertyOverrideValue<Impl: IRicheditUiaOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPropertyOverrideValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDisplaysImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDisplaysVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDisplaysImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDisplaysVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDisplays as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocumentImpl: Sized + IDispatchImpl {
    fn GetName();
    fn GetSelection();
    fn GetStoryCount();
    fn GetStoryRanges();
    fn GetSaved();
    fn SetSaved();
    fn GetDefaultTabStop();
    fn SetDefaultTabStop();
    fn New();
    fn Open();
    fn Save();
    fn Freeze();
    fn Unfreeze();
    fn BeginEditCollection();
    fn EndEditCollection();
    fn Undo();
    fn Redo();
    fn Range();
    fn RangeFromPoint();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocumentVtbl {
        unsafe extern "system" fn GetName<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryCount<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryRanges<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSaved<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSaved<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn New<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Freeze<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unfreeze<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEditCollection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEditCollection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Undo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Redo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Range<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetSelection::<Impl, IMPL_OFFSET>,
            GetStoryCount::<Impl, IMPL_OFFSET>,
            GetStoryRanges::<Impl, IMPL_OFFSET>,
            GetSaved::<Impl, IMPL_OFFSET>,
            SetSaved::<Impl, IMPL_OFFSET>,
            GetDefaultTabStop::<Impl, IMPL_OFFSET>,
            SetDefaultTabStop::<Impl, IMPL_OFFSET>,
            New::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Freeze::<Impl, IMPL_OFFSET>,
            Unfreeze::<Impl, IMPL_OFFSET>,
            BeginEditCollection::<Impl, IMPL_OFFSET>,
            EndEditCollection::<Impl, IMPL_OFFSET>,
            Undo::<Impl, IMPL_OFFSET>,
            Redo::<Impl, IMPL_OFFSET>,
            Range::<Impl, IMPL_OFFSET>,
            RangeFromPoint::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument2Impl: Sized + ITextDocumentImpl + IDispatchImpl {
    fn GetCaretType();
    fn SetCaretType();
    fn GetDisplays();
    fn GetDocumentFont();
    fn SetDocumentFont();
    fn GetDocumentPara();
    fn SetDocumentPara();
    fn GetEastAsianFlags();
    fn GetGenerator();
    fn SetIMEInProgress();
    fn GetNotificationMode();
    fn SetNotificationMode();
    fn GetSelection2();
    fn GetStoryRanges2();
    fn GetTypographyOptions();
    fn GetVersion();
    fn GetWindow();
    fn AttachMsgFilter();
    fn CheckTextLimit();
    fn GetCallManager();
    fn GetClientRect();
    fn GetEffectColor();
    fn GetImmContext();
    fn GetPreferredFont();
    fn GetProperty();
    fn GetStrings();
    fn Notify();
    fn Range2();
    fn RangeFromPoint2();
    fn ReleaseCallManager();
    fn ReleaseImmContext();
    fn SetEffectColor();
    fn SetProperty();
    fn SetTypographyOptions();
    fn SysBeep();
    fn Update();
    fn UpdateWindow();
    fn GetMathProperties();
    fn SetMathProperties();
    fn GetActiveStory();
    fn SetActiveStory();
    fn GetMainStory();
    fn GetNewStory();
    fn GetStory();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument2Vtbl {
        unsafe extern "system" fn GetCaretType<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplays<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplays: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocumentFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentPara<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocumentPara<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEastAsianFlags<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGenerator<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIMEInProgress<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNotificationMode<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationMode<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelection2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryRanges2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypographyOptions<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindow<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachMsgFilter<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckTextLimit<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCallManager<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientRect<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectColor<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImmContext<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferredFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrings<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Range2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeFromPoint2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseCallManager<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseImmContext<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffectColor<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypographyOptions<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SysBeep<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateWindow<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMathProperties<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMathProperties<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMainStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNewStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetSelection::<Impl, IMPL_OFFSET>,
            GetStoryCount::<Impl, IMPL_OFFSET>,
            GetStoryRanges::<Impl, IMPL_OFFSET>,
            GetSaved::<Impl, IMPL_OFFSET>,
            SetSaved::<Impl, IMPL_OFFSET>,
            GetDefaultTabStop::<Impl, IMPL_OFFSET>,
            SetDefaultTabStop::<Impl, IMPL_OFFSET>,
            New::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Freeze::<Impl, IMPL_OFFSET>,
            Unfreeze::<Impl, IMPL_OFFSET>,
            BeginEditCollection::<Impl, IMPL_OFFSET>,
            EndEditCollection::<Impl, IMPL_OFFSET>,
            Undo::<Impl, IMPL_OFFSET>,
            Redo::<Impl, IMPL_OFFSET>,
            Range::<Impl, IMPL_OFFSET>,
            RangeFromPoint::<Impl, IMPL_OFFSET>,
            GetCaretType::<Impl, IMPL_OFFSET>,
            SetCaretType::<Impl, IMPL_OFFSET>,
            GetDisplays::<Impl, IMPL_OFFSET>,
            GetDocumentFont::<Impl, IMPL_OFFSET>,
            SetDocumentFont::<Impl, IMPL_OFFSET>,
            GetDocumentPara::<Impl, IMPL_OFFSET>,
            SetDocumentPara::<Impl, IMPL_OFFSET>,
            GetEastAsianFlags::<Impl, IMPL_OFFSET>,
            GetGenerator::<Impl, IMPL_OFFSET>,
            SetIMEInProgress::<Impl, IMPL_OFFSET>,
            GetNotificationMode::<Impl, IMPL_OFFSET>,
            SetNotificationMode::<Impl, IMPL_OFFSET>,
            GetSelection2::<Impl, IMPL_OFFSET>,
            GetStoryRanges2::<Impl, IMPL_OFFSET>,
            GetTypographyOptions::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            AttachMsgFilter::<Impl, IMPL_OFFSET>,
            CheckTextLimit::<Impl, IMPL_OFFSET>,
            GetCallManager::<Impl, IMPL_OFFSET>,
            GetClientRect::<Impl, IMPL_OFFSET>,
            GetEffectColor::<Impl, IMPL_OFFSET>,
            GetImmContext::<Impl, IMPL_OFFSET>,
            GetPreferredFont::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            GetStrings::<Impl, IMPL_OFFSET>,
            Notify::<Impl, IMPL_OFFSET>,
            Range2::<Impl, IMPL_OFFSET>,
            RangeFromPoint2::<Impl, IMPL_OFFSET>,
            ReleaseCallManager::<Impl, IMPL_OFFSET>,
            ReleaseImmContext::<Impl, IMPL_OFFSET>,
            SetEffectColor::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetTypographyOptions::<Impl, IMPL_OFFSET>,
            SysBeep::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
            UpdateWindow::<Impl, IMPL_OFFSET>,
            GetMathProperties::<Impl, IMPL_OFFSET>,
            SetMathProperties::<Impl, IMPL_OFFSET>,
            GetActiveStory::<Impl, IMPL_OFFSET>,
            SetActiveStory::<Impl, IMPL_OFFSET>,
            GetMainStory::<Impl, IMPL_OFFSET>,
            GetNewStory::<Impl, IMPL_OFFSET>,
            GetStory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument2OldImpl: Sized + ITextDocumentImpl + IDispatchImpl {
    fn AttachMsgFilter();
    fn SetEffectColor();
    fn GetEffectColor();
    fn GetCaretType();
    fn SetCaretType();
    fn GetImmContext();
    fn ReleaseImmContext();
    fn GetPreferredFont();
    fn GetNotificationMode();
    fn SetNotificationMode();
    fn GetClientRect();
    fn GetSelection2();
    fn GetWindow();
    fn GetFEFlags();
    fn UpdateWindow();
    fn CheckTextLimit();
    fn IMEInProgress();
    fn SysBeep();
    fn Update();
    fn Notify();
    fn GetDocumentFont();
    fn GetDocumentPara();
    fn GetCallManager();
    fn ReleaseCallManager();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument2OldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2OldImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument2OldVtbl {
        unsafe extern "system" fn AttachMsgFilter<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffectColor<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectColor<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pcr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaretType<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcarettype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, carettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImmContext<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseImmContext<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferredFont<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNotificationMode<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationMode<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientRect<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelection2<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindow<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFEFlags<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateWindow<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckTextLimit<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IMEInProgress<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SysBeep<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentFont<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentPara<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextpara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCallManager<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseCallManager<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetSelection::<Impl, IMPL_OFFSET>,
            GetStoryCount::<Impl, IMPL_OFFSET>,
            GetStoryRanges::<Impl, IMPL_OFFSET>,
            GetSaved::<Impl, IMPL_OFFSET>,
            SetSaved::<Impl, IMPL_OFFSET>,
            GetDefaultTabStop::<Impl, IMPL_OFFSET>,
            SetDefaultTabStop::<Impl, IMPL_OFFSET>,
            New::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Freeze::<Impl, IMPL_OFFSET>,
            Unfreeze::<Impl, IMPL_OFFSET>,
            BeginEditCollection::<Impl, IMPL_OFFSET>,
            EndEditCollection::<Impl, IMPL_OFFSET>,
            Undo::<Impl, IMPL_OFFSET>,
            Redo::<Impl, IMPL_OFFSET>,
            Range::<Impl, IMPL_OFFSET>,
            RangeFromPoint::<Impl, IMPL_OFFSET>,
            AttachMsgFilter::<Impl, IMPL_OFFSET>,
            SetEffectColor::<Impl, IMPL_OFFSET>,
            GetEffectColor::<Impl, IMPL_OFFSET>,
            GetCaretType::<Impl, IMPL_OFFSET>,
            SetCaretType::<Impl, IMPL_OFFSET>,
            GetImmContext::<Impl, IMPL_OFFSET>,
            ReleaseImmContext::<Impl, IMPL_OFFSET>,
            GetPreferredFont::<Impl, IMPL_OFFSET>,
            GetNotificationMode::<Impl, IMPL_OFFSET>,
            SetNotificationMode::<Impl, IMPL_OFFSET>,
            GetClientRect::<Impl, IMPL_OFFSET>,
            GetSelection2::<Impl, IMPL_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            GetFEFlags::<Impl, IMPL_OFFSET>,
            UpdateWindow::<Impl, IMPL_OFFSET>,
            CheckTextLimit::<Impl, IMPL_OFFSET>,
            IMEInProgress::<Impl, IMPL_OFFSET>,
            SysBeep::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
            Notify::<Impl, IMPL_OFFSET>,
            GetDocumentFont::<Impl, IMPL_OFFSET>,
            GetDocumentPara::<Impl, IMPL_OFFSET>,
            GetCallManager::<Impl, IMPL_OFFSET>,
            ReleaseCallManager::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2Old as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextFontImpl: Sized + IDispatchImpl {
    fn GetDuplicate();
    fn SetDuplicate();
    fn CanChange();
    fn IsEqual();
    fn Reset();
    fn GetStyle();
    fn SetStyle();
    fn GetAllCaps();
    fn SetAllCaps();
    fn GetAnimation();
    fn SetAnimation();
    fn GetBackColor();
    fn SetBackColor();
    fn GetBold();
    fn SetBold();
    fn GetEmboss();
    fn SetEmboss();
    fn GetForeColor();
    fn SetForeColor();
    fn GetHidden();
    fn SetHidden();
    fn GetEngrave();
    fn SetEngrave();
    fn GetItalic();
    fn SetItalic();
    fn GetKerning();
    fn SetKerning();
    fn GetLanguageID();
    fn SetLanguageID();
    fn GetName();
    fn SetName();
    fn GetOutline();
    fn SetOutline();
    fn GetPosition();
    fn SetPosition();
    fn GetProtected();
    fn SetProtected();
    fn GetShadow();
    fn SetShadow();
    fn GetSize();
    fn SetSize();
    fn GetSmallCaps();
    fn SetSmallCaps();
    fn GetSpacing();
    fn SetSpacing();
    fn GetStrikeThrough();
    fn SetStrikeThrough();
    fn GetSubscript();
    fn SetSubscript();
    fn GetSuperscript();
    fn SetSuperscript();
    fn GetUnderline();
    fn SetUnderline();
    fn GetWeight();
    fn SetWeight();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFontImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextFontVtbl {
        unsafe extern "system" fn GetDuplicate<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuplicate<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanChange<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyle<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStyle<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnimation<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAnimation<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBold<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBold<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEmboss<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEmboss<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForeColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetForeColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHidden<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHidden<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEngrave<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEngrave<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItalic<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetItalic<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKerning<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKerning<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguageID<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguageID<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosition<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPosition<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtected<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProtected<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShadow<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetShadow<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSmallCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSmallCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpacing<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpacing<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrikeThrough<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrikeThrough<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSuperscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSuperscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnderline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnderline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWeight<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWeight<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            SetDuplicate::<Impl, IMPL_OFFSET>,
            CanChange::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            SetStyle::<Impl, IMPL_OFFSET>,
            GetAllCaps::<Impl, IMPL_OFFSET>,
            SetAllCaps::<Impl, IMPL_OFFSET>,
            GetAnimation::<Impl, IMPL_OFFSET>,
            SetAnimation::<Impl, IMPL_OFFSET>,
            GetBackColor::<Impl, IMPL_OFFSET>,
            SetBackColor::<Impl, IMPL_OFFSET>,
            GetBold::<Impl, IMPL_OFFSET>,
            SetBold::<Impl, IMPL_OFFSET>,
            GetEmboss::<Impl, IMPL_OFFSET>,
            SetEmboss::<Impl, IMPL_OFFSET>,
            GetForeColor::<Impl, IMPL_OFFSET>,
            SetForeColor::<Impl, IMPL_OFFSET>,
            GetHidden::<Impl, IMPL_OFFSET>,
            SetHidden::<Impl, IMPL_OFFSET>,
            GetEngrave::<Impl, IMPL_OFFSET>,
            SetEngrave::<Impl, IMPL_OFFSET>,
            GetItalic::<Impl, IMPL_OFFSET>,
            SetItalic::<Impl, IMPL_OFFSET>,
            GetKerning::<Impl, IMPL_OFFSET>,
            SetKerning::<Impl, IMPL_OFFSET>,
            GetLanguageID::<Impl, IMPL_OFFSET>,
            SetLanguageID::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetOutline::<Impl, IMPL_OFFSET>,
            SetOutline::<Impl, IMPL_OFFSET>,
            GetPosition::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            GetProtected::<Impl, IMPL_OFFSET>,
            SetProtected::<Impl, IMPL_OFFSET>,
            GetShadow::<Impl, IMPL_OFFSET>,
            SetShadow::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            GetSmallCaps::<Impl, IMPL_OFFSET>,
            SetSmallCaps::<Impl, IMPL_OFFSET>,
            GetSpacing::<Impl, IMPL_OFFSET>,
            SetSpacing::<Impl, IMPL_OFFSET>,
            GetStrikeThrough::<Impl, IMPL_OFFSET>,
            SetStrikeThrough::<Impl, IMPL_OFFSET>,
            GetSubscript::<Impl, IMPL_OFFSET>,
            SetSubscript::<Impl, IMPL_OFFSET>,
            GetSuperscript::<Impl, IMPL_OFFSET>,
            SetSuperscript::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            SetWeight::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextFont as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextFont2Impl: Sized + ITextFontImpl + IDispatchImpl {
    fn GetCount();
    fn GetAutoLigatures();
    fn SetAutoLigatures();
    fn GetAutospaceAlpha();
    fn SetAutospaceAlpha();
    fn GetAutospaceNumeric();
    fn SetAutospaceNumeric();
    fn GetAutospaceParens();
    fn SetAutospaceParens();
    fn GetCharRep();
    fn SetCharRep();
    fn GetCompressionMode();
    fn SetCompressionMode();
    fn GetCookie();
    fn SetCookie();
    fn GetDoubleStrike();
    fn SetDoubleStrike();
    fn GetDuplicate2();
    fn SetDuplicate2();
    fn GetLinkType();
    fn GetMathZone();
    fn SetMathZone();
    fn GetModWidthPairs();
    fn SetModWidthPairs();
    fn GetModWidthSpace();
    fn SetModWidthSpace();
    fn GetOldNumbers();
    fn SetOldNumbers();
    fn GetOverlapping();
    fn SetOverlapping();
    fn GetPositionSubSuper();
    fn SetPositionSubSuper();
    fn GetScaling();
    fn SetScaling();
    fn GetSpaceExtension();
    fn SetSpaceExtension();
    fn GetUnderlinePositionMode();
    fn SetUnderlinePositionMode();
    fn GetEffects();
    fn GetEffects2();
    fn GetProperty();
    fn GetPropertyInfo();
    fn IsEqual2();
    fn SetEffects();
    fn SetEffects2();
    fn SetProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextFont2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextFont2Vtbl {
        unsafe extern "system" fn GetCount<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutoLigatures<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoLigatures<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutospaceAlpha<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutospaceAlpha<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutospaceNumeric<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutospaceNumeric<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutospaceParens<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutospaceParens<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCharRep<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCharRep<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompressionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompressionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCookie<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCookie<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDoubleStrike<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDoubleStrike<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuplicate2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLinkType<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMathZone<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMathZone<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetModWidthPairs<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModWidthPairs<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetModWidthSpace<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModWidthSpace<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOldNumbers<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOldNumbers<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverlapping<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverlapping<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPositionSubSuper<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPositionSubSuper<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScaling<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaling<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpaceExtension<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpaceExtension<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffects<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffects2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffects<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffects2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            SetDuplicate::<Impl, IMPL_OFFSET>,
            CanChange::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            SetStyle::<Impl, IMPL_OFFSET>,
            GetAllCaps::<Impl, IMPL_OFFSET>,
            SetAllCaps::<Impl, IMPL_OFFSET>,
            GetAnimation::<Impl, IMPL_OFFSET>,
            SetAnimation::<Impl, IMPL_OFFSET>,
            GetBackColor::<Impl, IMPL_OFFSET>,
            SetBackColor::<Impl, IMPL_OFFSET>,
            GetBold::<Impl, IMPL_OFFSET>,
            SetBold::<Impl, IMPL_OFFSET>,
            GetEmboss::<Impl, IMPL_OFFSET>,
            SetEmboss::<Impl, IMPL_OFFSET>,
            GetForeColor::<Impl, IMPL_OFFSET>,
            SetForeColor::<Impl, IMPL_OFFSET>,
            GetHidden::<Impl, IMPL_OFFSET>,
            SetHidden::<Impl, IMPL_OFFSET>,
            GetEngrave::<Impl, IMPL_OFFSET>,
            SetEngrave::<Impl, IMPL_OFFSET>,
            GetItalic::<Impl, IMPL_OFFSET>,
            SetItalic::<Impl, IMPL_OFFSET>,
            GetKerning::<Impl, IMPL_OFFSET>,
            SetKerning::<Impl, IMPL_OFFSET>,
            GetLanguageID::<Impl, IMPL_OFFSET>,
            SetLanguageID::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetOutline::<Impl, IMPL_OFFSET>,
            SetOutline::<Impl, IMPL_OFFSET>,
            GetPosition::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            GetProtected::<Impl, IMPL_OFFSET>,
            SetProtected::<Impl, IMPL_OFFSET>,
            GetShadow::<Impl, IMPL_OFFSET>,
            SetShadow::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            GetSmallCaps::<Impl, IMPL_OFFSET>,
            SetSmallCaps::<Impl, IMPL_OFFSET>,
            GetSpacing::<Impl, IMPL_OFFSET>,
            SetSpacing::<Impl, IMPL_OFFSET>,
            GetStrikeThrough::<Impl, IMPL_OFFSET>,
            SetStrikeThrough::<Impl, IMPL_OFFSET>,
            GetSubscript::<Impl, IMPL_OFFSET>,
            SetSubscript::<Impl, IMPL_OFFSET>,
            GetSuperscript::<Impl, IMPL_OFFSET>,
            SetSuperscript::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            SetWeight::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetAutoLigatures::<Impl, IMPL_OFFSET>,
            SetAutoLigatures::<Impl, IMPL_OFFSET>,
            GetAutospaceAlpha::<Impl, IMPL_OFFSET>,
            SetAutospaceAlpha::<Impl, IMPL_OFFSET>,
            GetAutospaceNumeric::<Impl, IMPL_OFFSET>,
            SetAutospaceNumeric::<Impl, IMPL_OFFSET>,
            GetAutospaceParens::<Impl, IMPL_OFFSET>,
            SetAutospaceParens::<Impl, IMPL_OFFSET>,
            GetCharRep::<Impl, IMPL_OFFSET>,
            SetCharRep::<Impl, IMPL_OFFSET>,
            GetCompressionMode::<Impl, IMPL_OFFSET>,
            SetCompressionMode::<Impl, IMPL_OFFSET>,
            GetCookie::<Impl, IMPL_OFFSET>,
            SetCookie::<Impl, IMPL_OFFSET>,
            GetDoubleStrike::<Impl, IMPL_OFFSET>,
            SetDoubleStrike::<Impl, IMPL_OFFSET>,
            GetDuplicate2::<Impl, IMPL_OFFSET>,
            SetDuplicate2::<Impl, IMPL_OFFSET>,
            GetLinkType::<Impl, IMPL_OFFSET>,
            GetMathZone::<Impl, IMPL_OFFSET>,
            SetMathZone::<Impl, IMPL_OFFSET>,
            GetModWidthPairs::<Impl, IMPL_OFFSET>,
            SetModWidthPairs::<Impl, IMPL_OFFSET>,
            GetModWidthSpace::<Impl, IMPL_OFFSET>,
            SetModWidthSpace::<Impl, IMPL_OFFSET>,
            GetOldNumbers::<Impl, IMPL_OFFSET>,
            SetOldNumbers::<Impl, IMPL_OFFSET>,
            GetOverlapping::<Impl, IMPL_OFFSET>,
            SetOverlapping::<Impl, IMPL_OFFSET>,
            GetPositionSubSuper::<Impl, IMPL_OFFSET>,
            SetPositionSubSuper::<Impl, IMPL_OFFSET>,
            GetScaling::<Impl, IMPL_OFFSET>,
            SetScaling::<Impl, IMPL_OFFSET>,
            GetSpaceExtension::<Impl, IMPL_OFFSET>,
            SetSpaceExtension::<Impl, IMPL_OFFSET>,
            GetUnderlinePositionMode::<Impl, IMPL_OFFSET>,
            SetUnderlinePositionMode::<Impl, IMPL_OFFSET>,
            GetEffects::<Impl, IMPL_OFFSET>,
            GetEffects2::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            GetPropertyInfo::<Impl, IMPL_OFFSET>,
            IsEqual2::<Impl, IMPL_OFFSET>,
            SetEffects::<Impl, IMPL_OFFSET>,
            SetEffects2::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextFont2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHostImpl: Sized {
    fn TxGetDC();
    fn TxReleaseDC();
    fn TxShowScrollBar();
    fn TxEnableScrollBar();
    fn TxSetScrollRange();
    fn TxSetScrollPos();
    fn TxInvalidateRect();
    fn TxViewChange();
    fn TxCreateCaret();
    fn TxShowCaret();
    fn TxSetCaretPos();
    fn TxSetTimer();
    fn TxKillTimer();
    fn TxScrollWindowEx();
    fn TxSetCapture();
    fn TxSetFocus();
    fn TxSetCursor();
    fn TxScreenToClient();
    fn TxClientToScreen();
    fn TxActivate();
    fn TxDeactivate();
    fn TxGetClientRect();
    fn TxGetViewInset();
    fn TxGetCharFormat();
    fn TxGetParaFormat();
    fn TxGetSysColor();
    fn TxGetBackStyle();
    fn TxGetMaxLength();
    fn TxGetScrollBars();
    fn TxGetPasswordChar();
    fn TxGetAcceleratorPos();
    fn TxGetExtent();
    fn OnTxCharFormatChange();
    fn OnTxParaFormatChange();
    fn TxGetPropertyBits();
    fn TxNotify();
    fn TxImmGetContext();
    fn TxImmReleaseContext();
    fn TxGetSelectionBarWidth();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextHostVtbl {
        unsafe extern "system" fn TxGetDC<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxReleaseDC<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxShowScrollBar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxEnableScrollBar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetScrollRange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetScrollPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxInvalidateRect<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxViewChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxCreateCaret<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxShowCaret<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetCaretPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetTimer<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxKillTimer<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxScrollWindowEx<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetCapture<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetFocus<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetCursor<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxScreenToClient<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxClientToScreen<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxActivate<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxDeactivate<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetClientRect<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetViewInset<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetCharFormat<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetParaFormat<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetSysColor<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetBackStyle<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetMaxLength<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetScrollBars<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetPasswordChar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetExtent<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxCharFormatChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxParaFormatChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetPropertyBits<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxNotify<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxImmGetContext<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxImmReleaseContext<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            TxGetDC::<Impl, IMPL_OFFSET>,
            TxReleaseDC::<Impl, IMPL_OFFSET>,
            TxShowScrollBar::<Impl, IMPL_OFFSET>,
            TxEnableScrollBar::<Impl, IMPL_OFFSET>,
            TxSetScrollRange::<Impl, IMPL_OFFSET>,
            TxSetScrollPos::<Impl, IMPL_OFFSET>,
            TxInvalidateRect::<Impl, IMPL_OFFSET>,
            TxViewChange::<Impl, IMPL_OFFSET>,
            TxCreateCaret::<Impl, IMPL_OFFSET>,
            TxShowCaret::<Impl, IMPL_OFFSET>,
            TxSetCaretPos::<Impl, IMPL_OFFSET>,
            TxSetTimer::<Impl, IMPL_OFFSET>,
            TxKillTimer::<Impl, IMPL_OFFSET>,
            TxScrollWindowEx::<Impl, IMPL_OFFSET>,
            TxSetCapture::<Impl, IMPL_OFFSET>,
            TxSetFocus::<Impl, IMPL_OFFSET>,
            TxSetCursor::<Impl, IMPL_OFFSET>,
            TxScreenToClient::<Impl, IMPL_OFFSET>,
            TxClientToScreen::<Impl, IMPL_OFFSET>,
            TxActivate::<Impl, IMPL_OFFSET>,
            TxDeactivate::<Impl, IMPL_OFFSET>,
            TxGetClientRect::<Impl, IMPL_OFFSET>,
            TxGetViewInset::<Impl, IMPL_OFFSET>,
            TxGetCharFormat::<Impl, IMPL_OFFSET>,
            TxGetParaFormat::<Impl, IMPL_OFFSET>,
            TxGetSysColor::<Impl, IMPL_OFFSET>,
            TxGetBackStyle::<Impl, IMPL_OFFSET>,
            TxGetMaxLength::<Impl, IMPL_OFFSET>,
            TxGetScrollBars::<Impl, IMPL_OFFSET>,
            TxGetPasswordChar::<Impl, IMPL_OFFSET>,
            TxGetAcceleratorPos::<Impl, IMPL_OFFSET>,
            TxGetExtent::<Impl, IMPL_OFFSET>,
            OnTxCharFormatChange::<Impl, IMPL_OFFSET>,
            OnTxParaFormatChange::<Impl, IMPL_OFFSET>,
            TxGetPropertyBits::<Impl, IMPL_OFFSET>,
            TxNotify::<Impl, IMPL_OFFSET>,
            TxImmGetContext::<Impl, IMPL_OFFSET>,
            TxImmReleaseContext::<Impl, IMPL_OFFSET>,
            TxGetSelectionBarWidth::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost2Impl: Sized + ITextHostImpl {
    fn TxIsDoubleClickPending();
    fn TxGetWindow();
    fn TxSetForegroundWindow();
    fn TxGetPalette();
    fn TxGetEastAsianFlags();
    fn TxSetCursor2();
    fn TxFreeTextServicesNotification();
    fn TxGetEditStyle();
    fn TxGetWindowStyles();
    fn TxShowDropCaret();
    fn TxDestroyCaret();
    fn TxGetHorzExtent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextHost2Vtbl {
        unsafe extern "system" fn TxIsDoubleClickPending<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetWindow<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetForegroundWindow<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetPalette<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetCursor2<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetEditStyle<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetWindowStyles<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxShowDropCaret<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxDestroyCaret<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetHorzExtent<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhorzextent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            TxGetDC::<Impl, IMPL_OFFSET>,
            TxReleaseDC::<Impl, IMPL_OFFSET>,
            TxShowScrollBar::<Impl, IMPL_OFFSET>,
            TxEnableScrollBar::<Impl, IMPL_OFFSET>,
            TxSetScrollRange::<Impl, IMPL_OFFSET>,
            TxSetScrollPos::<Impl, IMPL_OFFSET>,
            TxInvalidateRect::<Impl, IMPL_OFFSET>,
            TxViewChange::<Impl, IMPL_OFFSET>,
            TxCreateCaret::<Impl, IMPL_OFFSET>,
            TxShowCaret::<Impl, IMPL_OFFSET>,
            TxSetCaretPos::<Impl, IMPL_OFFSET>,
            TxSetTimer::<Impl, IMPL_OFFSET>,
            TxKillTimer::<Impl, IMPL_OFFSET>,
            TxScrollWindowEx::<Impl, IMPL_OFFSET>,
            TxSetCapture::<Impl, IMPL_OFFSET>,
            TxSetFocus::<Impl, IMPL_OFFSET>,
            TxSetCursor::<Impl, IMPL_OFFSET>,
            TxScreenToClient::<Impl, IMPL_OFFSET>,
            TxClientToScreen::<Impl, IMPL_OFFSET>,
            TxActivate::<Impl, IMPL_OFFSET>,
            TxDeactivate::<Impl, IMPL_OFFSET>,
            TxGetClientRect::<Impl, IMPL_OFFSET>,
            TxGetViewInset::<Impl, IMPL_OFFSET>,
            TxGetCharFormat::<Impl, IMPL_OFFSET>,
            TxGetParaFormat::<Impl, IMPL_OFFSET>,
            TxGetSysColor::<Impl, IMPL_OFFSET>,
            TxGetBackStyle::<Impl, IMPL_OFFSET>,
            TxGetMaxLength::<Impl, IMPL_OFFSET>,
            TxGetScrollBars::<Impl, IMPL_OFFSET>,
            TxGetPasswordChar::<Impl, IMPL_OFFSET>,
            TxGetAcceleratorPos::<Impl, IMPL_OFFSET>,
            TxGetExtent::<Impl, IMPL_OFFSET>,
            OnTxCharFormatChange::<Impl, IMPL_OFFSET>,
            OnTxParaFormatChange::<Impl, IMPL_OFFSET>,
            TxGetPropertyBits::<Impl, IMPL_OFFSET>,
            TxNotify::<Impl, IMPL_OFFSET>,
            TxImmGetContext::<Impl, IMPL_OFFSET>,
            TxImmReleaseContext::<Impl, IMPL_OFFSET>,
            TxGetSelectionBarWidth::<Impl, IMPL_OFFSET>,
            TxIsDoubleClickPending::<Impl, IMPL_OFFSET>,
            TxGetWindow::<Impl, IMPL_OFFSET>,
            TxSetForegroundWindow::<Impl, IMPL_OFFSET>,
            TxGetPalette::<Impl, IMPL_OFFSET>,
            TxGetEastAsianFlags::<Impl, IMPL_OFFSET>,
            TxSetCursor2::<Impl, IMPL_OFFSET>,
            TxFreeTextServicesNotification::<Impl, IMPL_OFFSET>,
            TxGetEditStyle::<Impl, IMPL_OFFSET>,
            TxGetWindowStyles::<Impl, IMPL_OFFSET>,
            TxShowDropCaret::<Impl, IMPL_OFFSET>,
            TxDestroyCaret::<Impl, IMPL_OFFSET>,
            TxGetHorzExtent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextHost2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextParaImpl: Sized + IDispatchImpl {
    fn GetDuplicate();
    fn SetDuplicate();
    fn CanChange();
    fn IsEqual();
    fn Reset();
    fn GetStyle();
    fn SetStyle();
    fn GetAlignment();
    fn SetAlignment();
    fn GetHyphenation();
    fn SetHyphenation();
    fn GetFirstLineIndent();
    fn GetKeepTogether();
    fn SetKeepTogether();
    fn GetKeepWithNext();
    fn SetKeepWithNext();
    fn GetLeftIndent();
    fn GetLineSpacing();
    fn GetLineSpacingRule();
    fn GetListAlignment();
    fn SetListAlignment();
    fn GetListLevelIndex();
    fn SetListLevelIndex();
    fn GetListStart();
    fn SetListStart();
    fn GetListTab();
    fn SetListTab();
    fn GetListType();
    fn SetListType();
    fn GetNoLineNumber();
    fn SetNoLineNumber();
    fn GetPageBreakBefore();
    fn SetPageBreakBefore();
    fn GetRightIndent();
    fn SetRightIndent();
    fn SetIndents();
    fn SetLineSpacing();
    fn GetSpaceAfter();
    fn SetSpaceAfter();
    fn GetSpaceBefore();
    fn SetSpaceBefore();
    fn GetWidowControl();
    fn SetWidowControl();
    fn GetTabCount();
    fn AddTab();
    fn ClearAllTabs();
    fn DeleteTab();
    fn GetTab();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextParaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextParaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextParaVtbl {
        unsafe extern "system" fn GetDuplicate<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuplicate<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanChange<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyle<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStyle<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHyphenation<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHyphenation<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstLineIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeepTogether<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeepWithNext<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLeftIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineSpacing<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineSpacingRule<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListLevelIndex<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListLevelIndex<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListStart<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListStart<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListType<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListType<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNoLineNumber<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNoLineNumber<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageBreakBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPageBreakBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRightIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRightIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndents<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpaceAfter<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpaceAfter<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpaceBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpaceBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWidowControl<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWidowControl<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTabCount<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearAllTabs<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            SetDuplicate::<Impl, IMPL_OFFSET>,
            CanChange::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            SetStyle::<Impl, IMPL_OFFSET>,
            GetAlignment::<Impl, IMPL_OFFSET>,
            SetAlignment::<Impl, IMPL_OFFSET>,
            GetHyphenation::<Impl, IMPL_OFFSET>,
            SetHyphenation::<Impl, IMPL_OFFSET>,
            GetFirstLineIndent::<Impl, IMPL_OFFSET>,
            GetKeepTogether::<Impl, IMPL_OFFSET>,
            SetKeepTogether::<Impl, IMPL_OFFSET>,
            GetKeepWithNext::<Impl, IMPL_OFFSET>,
            SetKeepWithNext::<Impl, IMPL_OFFSET>,
            GetLeftIndent::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacingRule::<Impl, IMPL_OFFSET>,
            GetListAlignment::<Impl, IMPL_OFFSET>,
            SetListAlignment::<Impl, IMPL_OFFSET>,
            GetListLevelIndex::<Impl, IMPL_OFFSET>,
            SetListLevelIndex::<Impl, IMPL_OFFSET>,
            GetListStart::<Impl, IMPL_OFFSET>,
            SetListStart::<Impl, IMPL_OFFSET>,
            GetListTab::<Impl, IMPL_OFFSET>,
            SetListTab::<Impl, IMPL_OFFSET>,
            GetListType::<Impl, IMPL_OFFSET>,
            SetListType::<Impl, IMPL_OFFSET>,
            GetNoLineNumber::<Impl, IMPL_OFFSET>,
            SetNoLineNumber::<Impl, IMPL_OFFSET>,
            GetPageBreakBefore::<Impl, IMPL_OFFSET>,
            SetPageBreakBefore::<Impl, IMPL_OFFSET>,
            GetRightIndent::<Impl, IMPL_OFFSET>,
            SetRightIndent::<Impl, IMPL_OFFSET>,
            SetIndents::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetSpaceAfter::<Impl, IMPL_OFFSET>,
            SetSpaceAfter::<Impl, IMPL_OFFSET>,
            GetSpaceBefore::<Impl, IMPL_OFFSET>,
            SetSpaceBefore::<Impl, IMPL_OFFSET>,
            GetWidowControl::<Impl, IMPL_OFFSET>,
            SetWidowControl::<Impl, IMPL_OFFSET>,
            GetTabCount::<Impl, IMPL_OFFSET>,
            AddTab::<Impl, IMPL_OFFSET>,
            ClearAllTabs::<Impl, IMPL_OFFSET>,
            DeleteTab::<Impl, IMPL_OFFSET>,
            GetTab::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPara as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextPara2Impl: Sized + ITextParaImpl + IDispatchImpl {
    fn GetBorders();
    fn GetDuplicate2();
    fn SetDuplicate2();
    fn GetFontAlignment();
    fn SetFontAlignment();
    fn GetHangingPunctuation();
    fn SetHangingPunctuation();
    fn GetSnapToGrid();
    fn SetSnapToGrid();
    fn GetTrimPunctuationAtStart();
    fn SetTrimPunctuationAtStart();
    fn GetEffects();
    fn GetProperty();
    fn IsEqual2();
    fn SetEffects();
    fn SetProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextPara2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPara2Vtbl {
        unsafe extern "system" fn GetBorders<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppborders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuplicate2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAlignment<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontAlignment<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHangingPunctuation<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHangingPunctuation<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapToGrid<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapToGrid<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffects<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffects<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            SetDuplicate::<Impl, IMPL_OFFSET>,
            CanChange::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            SetStyle::<Impl, IMPL_OFFSET>,
            GetAlignment::<Impl, IMPL_OFFSET>,
            SetAlignment::<Impl, IMPL_OFFSET>,
            GetHyphenation::<Impl, IMPL_OFFSET>,
            SetHyphenation::<Impl, IMPL_OFFSET>,
            GetFirstLineIndent::<Impl, IMPL_OFFSET>,
            GetKeepTogether::<Impl, IMPL_OFFSET>,
            SetKeepTogether::<Impl, IMPL_OFFSET>,
            GetKeepWithNext::<Impl, IMPL_OFFSET>,
            SetKeepWithNext::<Impl, IMPL_OFFSET>,
            GetLeftIndent::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacingRule::<Impl, IMPL_OFFSET>,
            GetListAlignment::<Impl, IMPL_OFFSET>,
            SetListAlignment::<Impl, IMPL_OFFSET>,
            GetListLevelIndex::<Impl, IMPL_OFFSET>,
            SetListLevelIndex::<Impl, IMPL_OFFSET>,
            GetListStart::<Impl, IMPL_OFFSET>,
            SetListStart::<Impl, IMPL_OFFSET>,
            GetListTab::<Impl, IMPL_OFFSET>,
            SetListTab::<Impl, IMPL_OFFSET>,
            GetListType::<Impl, IMPL_OFFSET>,
            SetListType::<Impl, IMPL_OFFSET>,
            GetNoLineNumber::<Impl, IMPL_OFFSET>,
            SetNoLineNumber::<Impl, IMPL_OFFSET>,
            GetPageBreakBefore::<Impl, IMPL_OFFSET>,
            SetPageBreakBefore::<Impl, IMPL_OFFSET>,
            GetRightIndent::<Impl, IMPL_OFFSET>,
            SetRightIndent::<Impl, IMPL_OFFSET>,
            SetIndents::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetSpaceAfter::<Impl, IMPL_OFFSET>,
            SetSpaceAfter::<Impl, IMPL_OFFSET>,
            GetSpaceBefore::<Impl, IMPL_OFFSET>,
            SetSpaceBefore::<Impl, IMPL_OFFSET>,
            GetWidowControl::<Impl, IMPL_OFFSET>,
            SetWidowControl::<Impl, IMPL_OFFSET>,
            GetTabCount::<Impl, IMPL_OFFSET>,
            AddTab::<Impl, IMPL_OFFSET>,
            ClearAllTabs::<Impl, IMPL_OFFSET>,
            DeleteTab::<Impl, IMPL_OFFSET>,
            GetTab::<Impl, IMPL_OFFSET>,
            GetBorders::<Impl, IMPL_OFFSET>,
            GetDuplicate2::<Impl, IMPL_OFFSET>,
            SetDuplicate2::<Impl, IMPL_OFFSET>,
            GetFontAlignment::<Impl, IMPL_OFFSET>,
            SetFontAlignment::<Impl, IMPL_OFFSET>,
            GetHangingPunctuation::<Impl, IMPL_OFFSET>,
            SetHangingPunctuation::<Impl, IMPL_OFFSET>,
            GetSnapToGrid::<Impl, IMPL_OFFSET>,
            SetSnapToGrid::<Impl, IMPL_OFFSET>,
            GetTrimPunctuationAtStart::<Impl, IMPL_OFFSET>,
            SetTrimPunctuationAtStart::<Impl, IMPL_OFFSET>,
            GetEffects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            IsEqual2::<Impl, IMPL_OFFSET>,
            SetEffects::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPara2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRangeImpl: Sized + IDispatchImpl {
    fn GetText();
    fn SetText();
    fn GetChar();
    fn SetChar();
    fn GetDuplicate();
    fn GetFormattedText();
    fn SetFormattedText();
    fn GetStart();
    fn SetStart();
    fn GetEnd();
    fn SetEnd();
    fn GetFont();
    fn SetFont();
    fn GetPara();
    fn SetPara();
    fn GetStoryLength();
    fn GetStoryType();
    fn Collapse();
    fn Expand();
    fn GetIndex();
    fn SetIndex();
    fn SetRange();
    fn InRange();
    fn InStory();
    fn IsEqual();
    fn Select();
    fn StartOf();
    fn EndOf();
    fn Move();
    fn MoveStart();
    fn MoveEnd();
    fn MoveWhile();
    fn MoveStartWhile();
    fn MoveEndWhile();
    fn MoveUntil();
    fn MoveStartUntil();
    fn MoveEndUntil();
    fn FindText();
    fn FindTextStart();
    fn FindTextEnd();
    fn Delete();
    fn Cut();
    fn Copy();
    fn Paste();
    fn CanPaste();
    fn CanEdit();
    fn ChangeCase();
    fn GetPoint();
    fn SetPoint();
    fn ScrollIntoView();
    fn GetEmbeddedObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeVtbl {
        unsafe extern "system" fn GetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChar<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChar<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuplicate<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFont<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFont<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPara<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPara<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryLength<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryType<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Collapse<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Expand<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InStory<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Select<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveStartWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEndWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveStartUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEndUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindTextStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindTextEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cut<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copy<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Paste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanPaste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanEdit<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeCase<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEmbeddedObject<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetText::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
            GetChar::<Impl, IMPL_OFFSET>,
            SetChar::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            GetFormattedText::<Impl, IMPL_OFFSET>,
            SetFormattedText::<Impl, IMPL_OFFSET>,
            GetStart::<Impl, IMPL_OFFSET>,
            SetStart::<Impl, IMPL_OFFSET>,
            GetEnd::<Impl, IMPL_OFFSET>,
            SetEnd::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            SetFont::<Impl, IMPL_OFFSET>,
            GetPara::<Impl, IMPL_OFFSET>,
            SetPara::<Impl, IMPL_OFFSET>,
            GetStoryLength::<Impl, IMPL_OFFSET>,
            GetStoryType::<Impl, IMPL_OFFSET>,
            Collapse::<Impl, IMPL_OFFSET>,
            Expand::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            SetIndex::<Impl, IMPL_OFFSET>,
            SetRange::<Impl, IMPL_OFFSET>,
            InRange::<Impl, IMPL_OFFSET>,
            InStory::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Select::<Impl, IMPL_OFFSET>,
            StartOf::<Impl, IMPL_OFFSET>,
            EndOf::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            MoveStart::<Impl, IMPL_OFFSET>,
            MoveEnd::<Impl, IMPL_OFFSET>,
            MoveWhile::<Impl, IMPL_OFFSET>,
            MoveStartWhile::<Impl, IMPL_OFFSET>,
            MoveEndWhile::<Impl, IMPL_OFFSET>,
            MoveUntil::<Impl, IMPL_OFFSET>,
            MoveStartUntil::<Impl, IMPL_OFFSET>,
            MoveEndUntil::<Impl, IMPL_OFFSET>,
            FindText::<Impl, IMPL_OFFSET>,
            FindTextStart::<Impl, IMPL_OFFSET>,
            FindTextEnd::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Cut::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
            Paste::<Impl, IMPL_OFFSET>,
            CanPaste::<Impl, IMPL_OFFSET>,
            CanEdit::<Impl, IMPL_OFFSET>,
            ChangeCase::<Impl, IMPL_OFFSET>,
            GetPoint::<Impl, IMPL_OFFSET>,
            SetPoint::<Impl, IMPL_OFFSET>,
            ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetEmbeddedObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRange2Impl: Sized + ITextSelectionImpl + ITextRangeImpl + IDispatchImpl {
    fn GetCch();
    fn GetCells();
    fn GetColumn();
    fn GetCount();
    fn GetDuplicate2();
    fn GetFont2();
    fn SetFont2();
    fn GetFormattedText2();
    fn SetFormattedText2();
    fn GetGravity();
    fn SetGravity();
    fn GetPara2();
    fn SetPara2();
    fn GetRow();
    fn GetStartPara();
    fn GetTable();
    fn GetURL();
    fn SetURL();
    fn AddSubrange();
    fn BuildUpMath();
    fn DeleteSubrange();
    fn Find();
    fn GetChar2();
    fn GetDropCap();
    fn GetInlineObject();
    fn GetProperty();
    fn GetRect();
    fn GetSubrange();
    fn GetText2();
    fn HexToUnicode();
    fn InsertTable();
    fn Linearize();
    fn SetActiveSubrange();
    fn SetDropCap();
    fn SetProperty();
    fn SetText2();
    fn UnicodeToHex();
    fn SetInlineObject();
    fn GetMathFunctionType();
    fn InsertImage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRange2Vtbl {
        unsafe extern "system" fn GetCch<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCells<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumn<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFont2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFont2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormattedText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormattedText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGravity<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGravity<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPara2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPara2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRow<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartPara<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTable<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetURL<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetURL<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuildUpMath<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Find<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChar2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDropCap<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInlineObject<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRect<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HexToUnicode<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertTable<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Linearize<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDropCap<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnicodeToHex<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInlineObject<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMathFunctionType<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertImage<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetText::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
            GetChar::<Impl, IMPL_OFFSET>,
            SetChar::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            GetFormattedText::<Impl, IMPL_OFFSET>,
            SetFormattedText::<Impl, IMPL_OFFSET>,
            GetStart::<Impl, IMPL_OFFSET>,
            SetStart::<Impl, IMPL_OFFSET>,
            GetEnd::<Impl, IMPL_OFFSET>,
            SetEnd::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            SetFont::<Impl, IMPL_OFFSET>,
            GetPara::<Impl, IMPL_OFFSET>,
            SetPara::<Impl, IMPL_OFFSET>,
            GetStoryLength::<Impl, IMPL_OFFSET>,
            GetStoryType::<Impl, IMPL_OFFSET>,
            Collapse::<Impl, IMPL_OFFSET>,
            Expand::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            SetIndex::<Impl, IMPL_OFFSET>,
            SetRange::<Impl, IMPL_OFFSET>,
            InRange::<Impl, IMPL_OFFSET>,
            InStory::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Select::<Impl, IMPL_OFFSET>,
            StartOf::<Impl, IMPL_OFFSET>,
            EndOf::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            MoveStart::<Impl, IMPL_OFFSET>,
            MoveEnd::<Impl, IMPL_OFFSET>,
            MoveWhile::<Impl, IMPL_OFFSET>,
            MoveStartWhile::<Impl, IMPL_OFFSET>,
            MoveEndWhile::<Impl, IMPL_OFFSET>,
            MoveUntil::<Impl, IMPL_OFFSET>,
            MoveStartUntil::<Impl, IMPL_OFFSET>,
            MoveEndUntil::<Impl, IMPL_OFFSET>,
            FindText::<Impl, IMPL_OFFSET>,
            FindTextStart::<Impl, IMPL_OFFSET>,
            FindTextEnd::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Cut::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
            Paste::<Impl, IMPL_OFFSET>,
            CanPaste::<Impl, IMPL_OFFSET>,
            CanEdit::<Impl, IMPL_OFFSET>,
            ChangeCase::<Impl, IMPL_OFFSET>,
            GetPoint::<Impl, IMPL_OFFSET>,
            SetPoint::<Impl, IMPL_OFFSET>,
            ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetEmbeddedObject::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            MoveLeft::<Impl, IMPL_OFFSET>,
            MoveRight::<Impl, IMPL_OFFSET>,
            MoveUp::<Impl, IMPL_OFFSET>,
            MoveDown::<Impl, IMPL_OFFSET>,
            HomeKey::<Impl, IMPL_OFFSET>,
            EndKey::<Impl, IMPL_OFFSET>,
            TypeText::<Impl, IMPL_OFFSET>,
            GetCch::<Impl, IMPL_OFFSET>,
            GetCells::<Impl, IMPL_OFFSET>,
            GetColumn::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetDuplicate2::<Impl, IMPL_OFFSET>,
            GetFont2::<Impl, IMPL_OFFSET>,
            SetFont2::<Impl, IMPL_OFFSET>,
            GetFormattedText2::<Impl, IMPL_OFFSET>,
            SetFormattedText2::<Impl, IMPL_OFFSET>,
            GetGravity::<Impl, IMPL_OFFSET>,
            SetGravity::<Impl, IMPL_OFFSET>,
            GetPara2::<Impl, IMPL_OFFSET>,
            SetPara2::<Impl, IMPL_OFFSET>,
            GetRow::<Impl, IMPL_OFFSET>,
            GetStartPara::<Impl, IMPL_OFFSET>,
            GetTable::<Impl, IMPL_OFFSET>,
            GetURL::<Impl, IMPL_OFFSET>,
            SetURL::<Impl, IMPL_OFFSET>,
            AddSubrange::<Impl, IMPL_OFFSET>,
            BuildUpMath::<Impl, IMPL_OFFSET>,
            DeleteSubrange::<Impl, IMPL_OFFSET>,
            Find::<Impl, IMPL_OFFSET>,
            GetChar2::<Impl, IMPL_OFFSET>,
            GetDropCap::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            GetRect::<Impl, IMPL_OFFSET>,
            GetSubrange::<Impl, IMPL_OFFSET>,
            GetText2::<Impl, IMPL_OFFSET>,
            HexToUnicode::<Impl, IMPL_OFFSET>,
            InsertTable::<Impl, IMPL_OFFSET>,
            Linearize::<Impl, IMPL_OFFSET>,
            SetActiveSubrange::<Impl, IMPL_OFFSET>,
            SetDropCap::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetText2::<Impl, IMPL_OFFSET>,
            UnicodeToHex::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            GetMathFunctionType::<Impl, IMPL_OFFSET>,
            InsertImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRowImpl: Sized + IDispatchImpl {
    fn GetAlignment();
    fn SetAlignment();
    fn GetCellCount();
    fn SetCellCount();
    fn GetCellCountCache();
    fn SetCellCountCache();
    fn GetCellIndex();
    fn SetCellIndex();
    fn GetCellMargin();
    fn SetCellMargin();
    fn GetHeight();
    fn SetHeight();
    fn GetIndent();
    fn SetIndent();
    fn GetKeepTogether();
    fn SetKeepTogether();
    fn GetKeepWithNext();
    fn SetKeepWithNext();
    fn GetNestLevel();
    fn GetRTL();
    fn SetRTL();
    fn GetCellAlignment();
    fn SetCellAlignment();
    fn GetCellColorBack();
    fn SetCellColorBack();
    fn GetCellColorFore();
    fn SetCellColorFore();
    fn GetCellMergeFlags();
    fn SetCellMergeFlags();
    fn GetCellShading();
    fn SetCellShading();
    fn GetCellVerticalText();
    fn SetCellVerticalText();
    fn GetCellWidth();
    fn SetCellWidth();
    fn GetCellBorderColors();
    fn GetCellBorderWidths();
    fn SetCellBorderColors();
    fn SetCellBorderWidths();
    fn Apply();
    fn CanChange();
    fn GetProperty();
    fn Insert();
    fn IsEqual();
    fn Reset();
    fn SetProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRowVtbl {
        unsafe extern "system" fn GetAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellCount<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellCount<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellCountCache<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellCountCache<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellIndex<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellIndex<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellMargin<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellMargin<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHeight<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHeight<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndent<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndent<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeepTogether<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeepWithNext<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNestLevel<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRTL<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRTL<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellColorBack<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellColorBack<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellColorFore<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellColorFore<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellMergeFlags<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellMergeFlags<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellShading<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellShading<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellVerticalText<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellVerticalText<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellWidth<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellWidth<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellBorderColors<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCellBorderWidths<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellBorderColors<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCellBorderWidths<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32, flags: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanChange<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Insert<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prow: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetAlignment::<Impl, IMPL_OFFSET>,
            SetAlignment::<Impl, IMPL_OFFSET>,
            GetCellCount::<Impl, IMPL_OFFSET>,
            SetCellCount::<Impl, IMPL_OFFSET>,
            GetCellCountCache::<Impl, IMPL_OFFSET>,
            SetCellCountCache::<Impl, IMPL_OFFSET>,
            GetCellIndex::<Impl, IMPL_OFFSET>,
            SetCellIndex::<Impl, IMPL_OFFSET>,
            GetCellMargin::<Impl, IMPL_OFFSET>,
            SetCellMargin::<Impl, IMPL_OFFSET>,
            GetHeight::<Impl, IMPL_OFFSET>,
            SetHeight::<Impl, IMPL_OFFSET>,
            GetIndent::<Impl, IMPL_OFFSET>,
            SetIndent::<Impl, IMPL_OFFSET>,
            GetKeepTogether::<Impl, IMPL_OFFSET>,
            SetKeepTogether::<Impl, IMPL_OFFSET>,
            GetKeepWithNext::<Impl, IMPL_OFFSET>,
            SetKeepWithNext::<Impl, IMPL_OFFSET>,
            GetNestLevel::<Impl, IMPL_OFFSET>,
            GetRTL::<Impl, IMPL_OFFSET>,
            SetRTL::<Impl, IMPL_OFFSET>,
            GetCellAlignment::<Impl, IMPL_OFFSET>,
            SetCellAlignment::<Impl, IMPL_OFFSET>,
            GetCellColorBack::<Impl, IMPL_OFFSET>,
            SetCellColorBack::<Impl, IMPL_OFFSET>,
            GetCellColorFore::<Impl, IMPL_OFFSET>,
            SetCellColorFore::<Impl, IMPL_OFFSET>,
            GetCellMergeFlags::<Impl, IMPL_OFFSET>,
            SetCellMergeFlags::<Impl, IMPL_OFFSET>,
            GetCellShading::<Impl, IMPL_OFFSET>,
            SetCellShading::<Impl, IMPL_OFFSET>,
            GetCellVerticalText::<Impl, IMPL_OFFSET>,
            SetCellVerticalText::<Impl, IMPL_OFFSET>,
            GetCellWidth::<Impl, IMPL_OFFSET>,
            SetCellWidth::<Impl, IMPL_OFFSET>,
            GetCellBorderColors::<Impl, IMPL_OFFSET>,
            GetCellBorderWidths::<Impl, IMPL_OFFSET>,
            SetCellBorderColors::<Impl, IMPL_OFFSET>,
            SetCellBorderWidths::<Impl, IMPL_OFFSET>,
            Apply::<Impl, IMPL_OFFSET>,
            CanChange::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            Insert::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextSelectionImpl: Sized + ITextRangeImpl + IDispatchImpl {
    fn GetFlags();
    fn SetFlags();
    fn GetType();
    fn MoveLeft();
    fn MoveRight();
    fn MoveUp();
    fn MoveDown();
    fn HomeKey();
    fn EndKey();
    fn TypeText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextSelectionVtbl {
        unsafe extern "system" fn GetFlags<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveLeft<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveRight<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveUp<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveDown<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HomeKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TypeText<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetText::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
            GetChar::<Impl, IMPL_OFFSET>,
            SetChar::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            GetFormattedText::<Impl, IMPL_OFFSET>,
            SetFormattedText::<Impl, IMPL_OFFSET>,
            GetStart::<Impl, IMPL_OFFSET>,
            SetStart::<Impl, IMPL_OFFSET>,
            GetEnd::<Impl, IMPL_OFFSET>,
            SetEnd::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            SetFont::<Impl, IMPL_OFFSET>,
            GetPara::<Impl, IMPL_OFFSET>,
            SetPara::<Impl, IMPL_OFFSET>,
            GetStoryLength::<Impl, IMPL_OFFSET>,
            GetStoryType::<Impl, IMPL_OFFSET>,
            Collapse::<Impl, IMPL_OFFSET>,
            Expand::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            SetIndex::<Impl, IMPL_OFFSET>,
            SetRange::<Impl, IMPL_OFFSET>,
            InRange::<Impl, IMPL_OFFSET>,
            InStory::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Select::<Impl, IMPL_OFFSET>,
            StartOf::<Impl, IMPL_OFFSET>,
            EndOf::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            MoveStart::<Impl, IMPL_OFFSET>,
            MoveEnd::<Impl, IMPL_OFFSET>,
            MoveWhile::<Impl, IMPL_OFFSET>,
            MoveStartWhile::<Impl, IMPL_OFFSET>,
            MoveEndWhile::<Impl, IMPL_OFFSET>,
            MoveUntil::<Impl, IMPL_OFFSET>,
            MoveStartUntil::<Impl, IMPL_OFFSET>,
            MoveEndUntil::<Impl, IMPL_OFFSET>,
            FindText::<Impl, IMPL_OFFSET>,
            FindTextStart::<Impl, IMPL_OFFSET>,
            FindTextEnd::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Cut::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
            Paste::<Impl, IMPL_OFFSET>,
            CanPaste::<Impl, IMPL_OFFSET>,
            CanEdit::<Impl, IMPL_OFFSET>,
            ChangeCase::<Impl, IMPL_OFFSET>,
            GetPoint::<Impl, IMPL_OFFSET>,
            SetPoint::<Impl, IMPL_OFFSET>,
            ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetEmbeddedObject::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            MoveLeft::<Impl, IMPL_OFFSET>,
            MoveRight::<Impl, IMPL_OFFSET>,
            MoveUp::<Impl, IMPL_OFFSET>,
            MoveDown::<Impl, IMPL_OFFSET>,
            HomeKey::<Impl, IMPL_OFFSET>,
            EndKey::<Impl, IMPL_OFFSET>,
            TypeText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextSelection2Impl: Sized + ITextRange2Impl + ITextSelectionImpl + ITextRangeImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextSelection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextSelection2Vtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetText::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
            GetChar::<Impl, IMPL_OFFSET>,
            SetChar::<Impl, IMPL_OFFSET>,
            GetDuplicate::<Impl, IMPL_OFFSET>,
            GetFormattedText::<Impl, IMPL_OFFSET>,
            SetFormattedText::<Impl, IMPL_OFFSET>,
            GetStart::<Impl, IMPL_OFFSET>,
            SetStart::<Impl, IMPL_OFFSET>,
            GetEnd::<Impl, IMPL_OFFSET>,
            SetEnd::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            SetFont::<Impl, IMPL_OFFSET>,
            GetPara::<Impl, IMPL_OFFSET>,
            SetPara::<Impl, IMPL_OFFSET>,
            GetStoryLength::<Impl, IMPL_OFFSET>,
            GetStoryType::<Impl, IMPL_OFFSET>,
            Collapse::<Impl, IMPL_OFFSET>,
            Expand::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            SetIndex::<Impl, IMPL_OFFSET>,
            SetRange::<Impl, IMPL_OFFSET>,
            InRange::<Impl, IMPL_OFFSET>,
            InStory::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Select::<Impl, IMPL_OFFSET>,
            StartOf::<Impl, IMPL_OFFSET>,
            EndOf::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            MoveStart::<Impl, IMPL_OFFSET>,
            MoveEnd::<Impl, IMPL_OFFSET>,
            MoveWhile::<Impl, IMPL_OFFSET>,
            MoveStartWhile::<Impl, IMPL_OFFSET>,
            MoveEndWhile::<Impl, IMPL_OFFSET>,
            MoveUntil::<Impl, IMPL_OFFSET>,
            MoveStartUntil::<Impl, IMPL_OFFSET>,
            MoveEndUntil::<Impl, IMPL_OFFSET>,
            FindText::<Impl, IMPL_OFFSET>,
            FindTextStart::<Impl, IMPL_OFFSET>,
            FindTextEnd::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Cut::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
            Paste::<Impl, IMPL_OFFSET>,
            CanPaste::<Impl, IMPL_OFFSET>,
            CanEdit::<Impl, IMPL_OFFSET>,
            ChangeCase::<Impl, IMPL_OFFSET>,
            GetPoint::<Impl, IMPL_OFFSET>,
            SetPoint::<Impl, IMPL_OFFSET>,
            ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetEmbeddedObject::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            MoveLeft::<Impl, IMPL_OFFSET>,
            MoveRight::<Impl, IMPL_OFFSET>,
            MoveUp::<Impl, IMPL_OFFSET>,
            MoveDown::<Impl, IMPL_OFFSET>,
            HomeKey::<Impl, IMPL_OFFSET>,
            EndKey::<Impl, IMPL_OFFSET>,
            TypeText::<Impl, IMPL_OFFSET>,
            GetCch::<Impl, IMPL_OFFSET>,
            GetCells::<Impl, IMPL_OFFSET>,
            GetColumn::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetDuplicate2::<Impl, IMPL_OFFSET>,
            GetFont2::<Impl, IMPL_OFFSET>,
            SetFont2::<Impl, IMPL_OFFSET>,
            GetFormattedText2::<Impl, IMPL_OFFSET>,
            SetFormattedText2::<Impl, IMPL_OFFSET>,
            GetGravity::<Impl, IMPL_OFFSET>,
            SetGravity::<Impl, IMPL_OFFSET>,
            GetPara2::<Impl, IMPL_OFFSET>,
            SetPara2::<Impl, IMPL_OFFSET>,
            GetRow::<Impl, IMPL_OFFSET>,
            GetStartPara::<Impl, IMPL_OFFSET>,
            GetTable::<Impl, IMPL_OFFSET>,
            GetURL::<Impl, IMPL_OFFSET>,
            SetURL::<Impl, IMPL_OFFSET>,
            AddSubrange::<Impl, IMPL_OFFSET>,
            BuildUpMath::<Impl, IMPL_OFFSET>,
            DeleteSubrange::<Impl, IMPL_OFFSET>,
            Find::<Impl, IMPL_OFFSET>,
            GetChar2::<Impl, IMPL_OFFSET>,
            GetDropCap::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            GetRect::<Impl, IMPL_OFFSET>,
            GetSubrange::<Impl, IMPL_OFFSET>,
            GetText2::<Impl, IMPL_OFFSET>,
            HexToUnicode::<Impl, IMPL_OFFSET>,
            InsertTable::<Impl, IMPL_OFFSET>,
            Linearize::<Impl, IMPL_OFFSET>,
            SetActiveSubrange::<Impl, IMPL_OFFSET>,
            SetDropCap::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetText2::<Impl, IMPL_OFFSET>,
            UnicodeToHex::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            GetMathFunctionType::<Impl, IMPL_OFFSET>,
            InsertImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServicesImpl: Sized {
    fn TxSendMessage();
    fn TxDraw();
    fn TxGetHScroll();
    fn TxGetVScroll();
    fn OnTxSetCursor();
    fn TxQueryHitPoint();
    fn OnTxInPlaceActivate();
    fn OnTxInPlaceDeactivate();
    fn OnTxUIActivate();
    fn OnTxUIDeactivate();
    fn TxGetText();
    fn TxSetText();
    fn TxGetCurTargetX();
    fn TxGetBaseLinePos();
    fn TxGetNaturalSize();
    fn TxGetDropTarget();
    fn OnTxPropertyBitsChange();
    fn TxGetCachedSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextServicesVtbl {
        unsafe extern "system" fn TxSendMessage<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxDraw<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetHScroll<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetVScroll<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxSetCursor<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxQueryHitPoint<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxUIActivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxUIDeactivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetText<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxSetText<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetCurTargetX<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetBaseLinePos<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetNaturalSize<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetDropTarget<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxGetCachedSize<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            TxSendMessage::<Impl, IMPL_OFFSET>,
            TxDraw::<Impl, IMPL_OFFSET>,
            TxGetHScroll::<Impl, IMPL_OFFSET>,
            TxGetVScroll::<Impl, IMPL_OFFSET>,
            OnTxSetCursor::<Impl, IMPL_OFFSET>,
            TxQueryHitPoint::<Impl, IMPL_OFFSET>,
            OnTxInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnTxInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            OnTxUIActivate::<Impl, IMPL_OFFSET>,
            OnTxUIDeactivate::<Impl, IMPL_OFFSET>,
            TxGetText::<Impl, IMPL_OFFSET>,
            TxSetText::<Impl, IMPL_OFFSET>,
            TxGetCurTargetX::<Impl, IMPL_OFFSET>,
            TxGetBaseLinePos::<Impl, IMPL_OFFSET>,
            TxGetNaturalSize::<Impl, IMPL_OFFSET>,
            TxGetDropTarget::<Impl, IMPL_OFFSET>,
            OnTxPropertyBitsChange::<Impl, IMPL_OFFSET>,
            TxGetCachedSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices2Impl: Sized + ITextServicesImpl {
    fn TxGetNaturalSize2();
    fn TxDrawD2D();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextServices2Vtbl {
        unsafe extern "system" fn TxGetNaturalSize2<Impl: ITextServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TxDrawD2D<Impl: ITextServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            TxSendMessage::<Impl, IMPL_OFFSET>,
            TxDraw::<Impl, IMPL_OFFSET>,
            TxGetHScroll::<Impl, IMPL_OFFSET>,
            TxGetVScroll::<Impl, IMPL_OFFSET>,
            OnTxSetCursor::<Impl, IMPL_OFFSET>,
            TxQueryHitPoint::<Impl, IMPL_OFFSET>,
            OnTxInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnTxInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            OnTxUIActivate::<Impl, IMPL_OFFSET>,
            OnTxUIDeactivate::<Impl, IMPL_OFFSET>,
            TxGetText::<Impl, IMPL_OFFSET>,
            TxSetText::<Impl, IMPL_OFFSET>,
            TxGetCurTargetX::<Impl, IMPL_OFFSET>,
            TxGetBaseLinePos::<Impl, IMPL_OFFSET>,
            TxGetNaturalSize::<Impl, IMPL_OFFSET>,
            TxGetDropTarget::<Impl, IMPL_OFFSET>,
            OnTxPropertyBitsChange::<Impl, IMPL_OFFSET>,
            TxGetCachedSize::<Impl, IMPL_OFFSET>,
            TxGetNaturalSize2::<Impl, IMPL_OFFSET>,
            TxDrawD2D::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextServices2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoryImpl: Sized {
    fn GetActive();
    fn SetActive();
    fn GetDisplay();
    fn GetIndex();
    fn GetType();
    fn SetType();
    fn GetProperty();
    fn GetRange();
    fn GetText();
    fn SetFormattedText();
    fn SetProperty();
    fn SetText();
}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoryVtbl {
        unsafe extern "system" fn GetActive<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActive<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplay<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplay: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndex<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRange<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetActive::<Impl, IMPL_OFFSET>,
            SetActive::<Impl, IMPL_OFFSET>,
            GetDisplay::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetType::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            GetRange::<Impl, IMPL_OFFSET>,
            GetText::<Impl, IMPL_OFFSET>,
            SetFormattedText::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoryRangesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn GetCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoryRangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRangesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoryRangesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoryRanges as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoryRanges2Impl: Sized + ITextStoryRangesImpl + IDispatchImpl {
    fn Item2();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoryRanges2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoryRanges2Vtbl {
        unsafe extern "system" fn Item2<Impl: ITextStoryRanges2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, Item2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoryRanges2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStringsImpl: Sized + IDispatchImpl {
    fn Item();
    fn GetCount();
    fn Add();
    fn Append();
    fn Cat2();
    fn CatTop2();
    fn DeleteRange();
    fn EncodeFunction();
    fn GetCch();
    fn InsertNullStr();
    fn MoveBoundary();
    fn PrefixTop();
    fn Remove();
    fn SetFormattedText();
    fn SetOpCp();
    fn SuffixTop();
    fn Swap();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStringsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStringsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStringsVtbl {
        unsafe extern "system" fn Item<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cat2<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CatTop2<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRange<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeFunction<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCch<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertNullStr<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveBoundary<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrefixTop<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cstring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pranged: ::windows::core::RawPtr, pranges: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpCp<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuffixTop<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Swap<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Append::<Impl, IMPL_OFFSET>,
            Cat2::<Impl, IMPL_OFFSET>,
            CatTop2::<Impl, IMPL_OFFSET>,
            DeleteRange::<Impl, IMPL_OFFSET>,
            EncodeFunction::<Impl, IMPL_OFFSET>,
            GetCch::<Impl, IMPL_OFFSET>,
            InsertNullStr::<Impl, IMPL_OFFSET>,
            MoveBoundary::<Impl, IMPL_OFFSET>,
            PrefixTop::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            SetFormattedText::<Impl, IMPL_OFFSET>,
            SetOpCp::<Impl, IMPL_OFFSET>,
            SuffixTop::<Impl, IMPL_OFFSET>,
            Swap::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStrings as ::windows::core::Interface>::IID
    }
}
