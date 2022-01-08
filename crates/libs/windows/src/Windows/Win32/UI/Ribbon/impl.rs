pub trait IUIApplicationImpl: Sized {
    fn OnViewChanged();
    fn OnCreateUICommand();
    fn OnDestroyUICommand();
}
impl ::windows::core::RuntimeName for IUIApplication {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIApplication";
}
impl IUIApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplicationImpl, const OFFSET: isize>() -> IUIApplicationVtbl {
        unsafe extern "system" fn OnViewChanged<Impl: IUIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnViewChanged(viewid, typeid, &*(&view as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), verb, ureasoncode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCreateUICommand<Impl: IUIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCreateUICommand(commandid, typeid, ::core::mem::transmute_copy(&commandhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDestroyUICommand<Impl: IUIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDestroyUICommand(commandid, typeid, &*(&commandhandler as *const <IUICommandHandler as ::windows::core::Abi>::Abi as *const <IUICommandHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIApplication>, ::windows::core::GetTrustLevel, OnViewChanged::<Impl, OFFSET>, OnCreateUICommand::<Impl, OFFSET>, OnDestroyUICommand::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IUICollection {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUICollection";
}
impl IUICollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionImpl, const OFFSET: isize>() -> IUICollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(index, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&item as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(index, &*(&item as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replace<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replace(indexreplaced, &*(&itemreplacewith as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IUICollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUICollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetItem::<Impl, OFFSET>, Add::<Impl, OFFSET>, Insert::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, Replace::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait IUICollectionChangedEventImpl: Sized {
    fn OnChanged();
}
impl ::windows::core::RuntimeName for IUICollectionChangedEvent {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUICollectionChangedEvent";
}
impl IUICollectionChangedEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionChangedEventImpl, const OFFSET: isize>() -> IUICollectionChangedEventVtbl {
        unsafe extern "system" fn OnChanged<Impl: IUICollectionChangedEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnChanged(action, oldindex, &*(&olditem as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), newindex, &*(&newitem as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUICollectionChangedEvent>, ::windows::core::GetTrustLevel, OnChanged::<Impl, OFFSET>)
    }
}
pub trait IUICommandHandlerImpl: Sized {
    fn Execute();
    fn UpdateProperty();
}
impl ::windows::core::RuntimeName for IUICommandHandler {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUICommandHandler";
}
impl IUICommandHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICommandHandlerImpl, const OFFSET: isize>() -> IUICommandHandlerVtbl {
        unsafe extern "system" fn Execute<Impl: IUICommandHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute(
                commandid,
                verb,
                &*(&key as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&currentvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&commandexecutionproperties as *const <IUISimplePropertySet as ::windows::core::Abi>::Abi as *const <IUISimplePropertySet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProperty<Impl: IUICommandHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateProperty(commandid, &*(&key as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&currentvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUICommandHandler>, ::windows::core::GetTrustLevel, Execute::<Impl, OFFSET>, UpdateProperty::<Impl, OFFSET>)
    }
}
pub trait IUIContextualUIImpl: Sized {
    fn ShowAtLocation();
}
impl ::windows::core::RuntimeName for IUIContextualUI {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIContextualUI";
}
impl IUIContextualUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextualUIImpl, const OFFSET: isize>() -> IUIContextualUIVtbl {
        unsafe extern "system" fn ShowAtLocation<Impl: IUIContextualUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAtLocation(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIContextualUI>, ::windows::core::GetTrustLevel, ShowAtLocation::<Impl, OFFSET>)
    }
}
pub trait IUIEventLoggerImpl: Sized {
    fn OnUIEvent();
}
impl ::windows::core::RuntimeName for IUIEventLogger {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIEventLogger";
}
impl IUIEventLoggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventLoggerImpl, const OFFSET: isize>() -> IUIEventLoggerVtbl {
        unsafe extern "system" fn OnUIEvent<Impl: IUIEventLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUIEvent(&*(&peventparams as *const <UI_EVENTPARAMS as ::windows::core::Abi>::Abi as *const <UI_EVENTPARAMS as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIEventLogger>, ::windows::core::GetTrustLevel, OnUIEvent::<Impl, OFFSET>)
    }
}
pub trait IUIEventingManagerImpl: Sized {
    fn SetEventLogger();
}
impl ::windows::core::RuntimeName for IUIEventingManager {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIEventingManager";
}
impl IUIEventingManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventingManagerImpl, const OFFSET: isize>() -> IUIEventingManagerVtbl {
        unsafe extern "system" fn SetEventLogger<Impl: IUIEventingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventlogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventLogger(&*(&eventlogger as *const <IUIEventLogger as ::windows::core::Abi>::Abi as *const <IUIEventLogger as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIEventingManager>, ::windows::core::GetTrustLevel, SetEventLogger::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IUIFramework {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIFramework";
}
impl IUIFrameworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIFrameworkImpl, const OFFSET: isize>() -> IUIFrameworkVtbl {
        unsafe extern "system" fn Initialize<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&framewnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&application as *const <IUIApplication as ::windows::core::Abi>::Abi as *const <IUIApplication as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadUI<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadUI(&*(&instance as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), &*(&resourcename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetView(viewid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUICommandProperty<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUICommandProperty(commandid, &*(&key as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUICommandProperty<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUICommandProperty(commandid, &*(&key as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateUICommand<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateUICommand(commandid, flags, &*(&key as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPendingInvalidations<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushPendingInvalidations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModes<Impl: IUIFrameworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModes(imodes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIFramework>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            Destroy::<Impl, OFFSET>,
            LoadUI::<Impl, OFFSET>,
            GetView::<Impl, OFFSET>,
            GetUICommandProperty::<Impl, OFFSET>,
            SetUICommandProperty::<Impl, OFFSET>,
            InvalidateUICommand::<Impl, OFFSET>,
            FlushPendingInvalidations::<Impl, OFFSET>,
            SetModes::<Impl, OFFSET>,
        )
    }
}
pub trait IUIImageImpl: Sized {
    fn GetBitmap();
}
impl ::windows::core::RuntimeName for IUIImage {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIImage";
}
impl IUIImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageImpl, const OFFSET: isize>() -> IUIImageVtbl {
        unsafe extern "system" fn GetBitmap<Impl: IUIImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmap(::core::mem::transmute_copy(&bitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIImage>, ::windows::core::GetTrustLevel, GetBitmap::<Impl, OFFSET>)
    }
}
pub trait IUIImageFromBitmapImpl: Sized {
    fn CreateImage();
}
impl ::windows::core::RuntimeName for IUIImageFromBitmap {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIImageFromBitmap";
}
impl IUIImageFromBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageFromBitmapImpl, const OFFSET: isize>() -> IUIImageFromBitmapVtbl {
        unsafe extern "system" fn CreateImage<Impl: IUIImageFromBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImage(&*(&bitmap as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), options, ::core::mem::transmute_copy(&image)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIImageFromBitmap>, ::windows::core::GetTrustLevel, CreateImage::<Impl, OFFSET>)
    }
}
pub trait IUIRibbonImpl: Sized {
    fn GetHeight();
    fn LoadSettingsFromStream();
    fn SaveSettingsToStream();
}
impl ::windows::core::RuntimeName for IUIRibbon {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUIRibbon";
}
impl IUIRibbonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbonImpl, const OFFSET: isize>() -> IUIRibbonVtbl {
        unsafe extern "system" fn GetHeight<Impl: IUIRibbonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeight(::core::mem::transmute_copy(&cy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSettingsFromStream<Impl: IUIRibbonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadSettingsFromStream(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveSettingsToStream<Impl: IUIRibbonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveSettingsToStream(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIRibbon>, ::windows::core::GetTrustLevel, GetHeight::<Impl, OFFSET>, LoadSettingsFromStream::<Impl, OFFSET>, SaveSettingsToStream::<Impl, OFFSET>)
    }
}
pub trait IUISimplePropertySetImpl: Sized {
    fn GetValue();
}
impl ::windows::core::RuntimeName for IUISimplePropertySet {
    const NAME: &'static str = "Windows.Win32.UI.Ribbon.IUISimplePropertySet";
}
impl IUISimplePropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISimplePropertySetImpl, const OFFSET: isize>() -> IUISimplePropertySetVtbl {
        unsafe extern "system" fn GetValue<Impl: IUISimplePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&key as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUISimplePropertySet>, ::windows::core::GetTrustLevel, GetValue::<Impl, OFFSET>)
    }
}
