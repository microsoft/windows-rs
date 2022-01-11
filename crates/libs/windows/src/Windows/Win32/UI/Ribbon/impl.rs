pub trait IUIApplicationImpl: Sized {
    fn OnViewChanged();
    fn OnCreateUICommand();
    fn OnDestroyUICommand();
}
impl IUIApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIApplicationVtbl {
        unsafe extern "system" fn OnViewChanged<Impl: IUIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCreateUICommand<Impl: IUIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDestroyUICommand<Impl: IUIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnViewChanged: OnViewChanged::<Impl, IMPL_OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Impl, IMPL_OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIApplication as ::windows::core::Interface>::IID
    }
}
pub trait IUICollectionImpl: Sized {
    fn GetCount();
    fn GetItem();
    fn Add();
    fn Insert();
    fn RemoveAt();
    fn Replace();
    fn Clear();
}
impl IUICollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUICollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItem<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Insert<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Replace<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            Replace: Replace::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICollection as ::windows::core::Interface>::IID
    }
}
pub trait IUICollectionChangedEventImpl: Sized {
    fn OnChanged();
}
impl IUICollectionChangedEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionChangedEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUICollectionChangedEventVtbl {
        unsafe extern "system" fn OnChanged<Impl: IUICollectionChangedEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnChanged: OnChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICollectionChangedEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUICommandHandlerImpl: Sized {
    fn Execute();
    fn UpdateProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUICommandHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICommandHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUICommandHandlerVtbl {
        unsafe extern "system" fn Execute<Impl: IUICommandHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateProperty<Impl: IUICommandHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Execute: Execute::<Impl, IMPL_OFFSET>,
            UpdateProperty: UpdateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICommandHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIContextualUIImpl: Sized {
    fn ShowAtLocation();
}
impl IUIContextualUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextualUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIContextualUIVtbl {
        unsafe extern "system" fn ShowAtLocation<Impl: IUIContextualUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ShowAtLocation: ShowAtLocation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIContextualUI as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIEventLoggerImpl: Sized {
    fn OnUIEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIEventLoggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventLoggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIEventLoggerVtbl {
        unsafe extern "system" fn OnUIEvent<Impl: IUIEventLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUIEvent: OnUIEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIEventLogger as ::windows::core::Interface>::IID
    }
}
pub trait IUIEventingManagerImpl: Sized {
    fn SetEventLogger();
}
impl IUIEventingManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventingManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIEventingManagerVtbl {
        unsafe extern "system" fn SetEventLogger<Impl: IUIEventingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventlogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetEventLogger: SetEventLogger::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIEventingManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUIFrameworkImpl: Sized {
    fn Initialize();
    fn Destroy();
    fn LoadUI();
    fn GetView();
    fn GetUICommandProperty();
    fn SetUICommandProperty();
    fn InvalidateUICommand();
    fn FlushPendingInvalidations();
    fn SetModes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUIFrameworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIFrameworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIFrameworkVtbl {
        unsafe extern "system" fn Initialize<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadUI<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetView<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUICommandProperty<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUICommandProperty<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidateUICommand<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushPendingInvalidations<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModes<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            LoadUI: LoadUI::<Impl, IMPL_OFFSET>,
            GetView: GetView::<Impl, IMPL_OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Impl, IMPL_OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Impl, IMPL_OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Impl, IMPL_OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Impl, IMPL_OFFSET>,
            SetModes: SetModes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIFramework as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageImpl: Sized {
    fn GetBitmap();
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIImageVtbl {
        unsafe extern "system" fn GetBitmap<Impl: IUIImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetBitmap: GetBitmap::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIImage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageFromBitmapImpl: Sized {
    fn CreateImage();
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImageFromBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageFromBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIImageFromBitmapVtbl {
        unsafe extern "system" fn CreateImage<Impl: IUIImageFromBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateImage: CreateImage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIImageFromBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIRibbonImpl: Sized {
    fn GetHeight();
    fn LoadSettingsFromStream();
    fn SaveSettingsToStream();
}
#[cfg(feature = "Win32_System_Com")]
impl IUIRibbonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIRibbonVtbl {
        unsafe extern "system" fn GetHeight<Impl: IUIRibbonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadSettingsFromStream<Impl: IUIRibbonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveSettingsToStream<Impl: IUIRibbonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetHeight: GetHeight::<Impl, IMPL_OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Impl, IMPL_OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIRibbon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUISimplePropertySetImpl: Sized {
    fn GetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUISimplePropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISimplePropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISimplePropertySetVtbl {
        unsafe extern "system" fn GetValue<Impl: IUISimplePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetValue: GetValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISimplePropertySet as ::windows::core::Interface>::IID
    }
}
