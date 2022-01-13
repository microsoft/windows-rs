#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWebUICommandBarImpl: Sized {
    fn Visible(&mut self) -> ::windows::core::Result<bool>;
    fn SetVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Opacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetForegroundColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetBackgroundColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn ClosedDisplayMode(&mut self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode>;
    fn SetClosedDisplayMode(&mut self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()>;
    fn IsOpen(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn PrimaryCommands(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>;
    fn SecondaryCommands(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>;
    fn MenuOpened(&mut self, handler: &::core::option::Option<MenuOpenedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMenuOpened(&mut self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MenuClosed(&mut self, handler: &::core::option::Option<MenuClosedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMenuClosed(&mut self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&mut self, handler: &::core::option::Option<SizeChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&mut self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBar";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWebUICommandBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarVtbl {
        unsafe extern "system" fn Visible<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        unsafe extern "system" fn Opacity<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedDisplayMode<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebUICommandBarClosedDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedDisplayMode<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedDisplayMode(value).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOpen<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOpen(value).into()
        }
        unsafe extern "system" fn Size<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryCommands<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecondaryCommands<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecondaryCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MenuOpened<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MenuOpened(&*(&handler as *const <MenuOpenedEventHandler as ::windows::core::Abi>::Abi as *const <MenuOpenedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMenuOpened<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMenuOpened(&*(&value as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MenuClosed<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MenuClosed(&*(&handler as *const <MenuClosedEventHandler as ::windows::core::Abi>::Abi as *const <MenuClosedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMenuClosed<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMenuClosed(&*(&value as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SizeChanged<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeChanged(&*(&handler as *const <SizeChangedEventHandler as ::windows::core::Abi>::Abi as *const <SizeChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Impl: IWebUICommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSizeChanged(&*(&value as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBar, BASE_OFFSET>(),
            Visible: Visible::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            ClosedDisplayMode: ClosedDisplayMode::<Impl, IMPL_OFFSET>,
            SetClosedDisplayMode: SetClosedDisplayMode::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            SetIsOpen: SetIsOpen::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            PrimaryCommands: PrimaryCommands::<Impl, IMPL_OFFSET>,
            SecondaryCommands: SecondaryCommands::<Impl, IMPL_OFFSET>,
            MenuOpened: MenuOpened::<Impl, IMPL_OFFSET>,
            RemoveMenuOpened: RemoveMenuOpened::<Impl, IMPL_OFFSET>,
            MenuClosed: MenuClosed::<Impl, IMPL_OFFSET>,
            RemoveMenuClosed: RemoveMenuClosed::<Impl, IMPL_OFFSET>,
            SizeChanged: SizeChanged::<Impl, IMPL_OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUICommandBarBitmapIconImpl: Sized + IWebUICommandBarIconImpl {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarBitmapIcon";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUICommandBarBitmapIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarBitmapIconImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarBitmapIconVtbl {
        unsafe extern "system" fn Uri<Impl: IWebUICommandBarBitmapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IWebUICommandBarBitmapIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarBitmapIcon, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarBitmapIcon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUICommandBarBitmapIconFactoryImpl: Sized {
    fn Create(&mut self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebUICommandBarBitmapIcon>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUICommandBarBitmapIconFactory {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarBitmapIconFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUICommandBarBitmapIconFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarBitmapIconFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarBitmapIconFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebUICommandBarBitmapIconFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarBitmapIconFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarBitmapIconFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUICommandBarConfirmationButtonImpl: Sized + IWebUICommandBarElementImpl {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemInvoked(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarConfirmationButton";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUICommandBarConfirmationButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarConfirmationButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarConfirmationButtonVtbl {
        unsafe extern "system" fn Text<Impl: IWebUICommandBarConfirmationButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IWebUICommandBarConfirmationButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemInvoked<Impl: IWebUICommandBarConfirmationButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemInvoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemInvoked<Impl: IWebUICommandBarConfirmationButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemInvoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarConfirmationButton, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            ItemInvoked: ItemInvoked::<Impl, IMPL_OFFSET>,
            RemoveItemInvoked: RemoveItemInvoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarConfirmationButton as ::windows::core::Interface>::IID
    }
}
pub trait IWebUICommandBarElementImpl: Sized {}
impl ::windows::core::RuntimeName for IWebUICommandBarElement {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarElement";
}
impl IWebUICommandBarElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarElementVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarElement, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarElement as ::windows::core::Interface>::IID
    }
}
pub trait IWebUICommandBarIconImpl: Sized {}
impl ::windows::core::RuntimeName for IWebUICommandBarIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarIcon";
}
impl IWebUICommandBarIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarIconImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarIconVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarIcon, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarIcon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUICommandBarIconButtonImpl: Sized + IWebUICommandBarElementImpl {
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsToggleButton(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsToggleButton(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsChecked(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsChecked(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Icon(&mut self) -> ::windows::core::Result<IWebUICommandBarIcon>;
    fn SetIcon(&mut self, value: &::core::option::Option<IWebUICommandBarIcon>) -> ::windows::core::Result<()>;
    fn ItemInvoked(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarIconButton";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUICommandBarIconButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarIconButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarIconButtonVtbl {
        unsafe extern "system" fn Enabled<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Label<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsToggleButton<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsToggleButton() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsToggleButton<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsToggleButton(value).into()
        }
        unsafe extern "system" fn IsChecked<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsChecked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChecked<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChecked(value).into()
        }
        unsafe extern "system" fn Icon<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcon<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcon(&*(&value as *const <IWebUICommandBarIcon as ::windows::core::Abi>::Abi as *const <IWebUICommandBarIcon as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemInvoked<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemInvoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemInvoked<Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemInvoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarIconButton, BASE_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            IsToggleButton: IsToggleButton::<Impl, IMPL_OFFSET>,
            SetIsToggleButton: SetIsToggleButton::<Impl, IMPL_OFFSET>,
            IsChecked: IsChecked::<Impl, IMPL_OFFSET>,
            SetIsChecked: SetIsChecked::<Impl, IMPL_OFFSET>,
            Icon: Icon::<Impl, IMPL_OFFSET>,
            SetIcon: SetIcon::<Impl, IMPL_OFFSET>,
            ItemInvoked: ItemInvoked::<Impl, IMPL_OFFSET>,
            RemoveItemInvoked: RemoveItemInvoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarIconButton as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarItemInvokedEventArgsImpl: Sized {
    fn IsPrimaryCommand(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarItemInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarItemInvokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarItemInvokedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarItemInvokedEventArgsVtbl {
        unsafe extern "system" fn IsPrimaryCommand<Impl: IWebUICommandBarItemInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrimaryCommand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarItemInvokedEventArgs, BASE_OFFSET>(),
            IsPrimaryCommand: IsPrimaryCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarItemInvokedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUICommandBarSizeChangedEventArgsImpl: Sized {
    fn Size(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarSizeChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUICommandBarSizeChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarSizeChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarSizeChangedEventArgsVtbl {
        unsafe extern "system" fn Size<Impl: IWebUICommandBarSizeChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarSizeChangedEventArgs, BASE_OFFSET>(), Size: Size::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarSizeChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<WebUICommandBar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarStatics {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IWebUICommandBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSymbolIconImpl: Sized + IWebUICommandBarIconImpl {
    fn Symbol(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSymbol(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarSymbolIcon";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarSymbolIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarSymbolIconImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarSymbolIconVtbl {
        unsafe extern "system" fn Symbol<Impl: IWebUICommandBarSymbolIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSymbol<Impl: IWebUICommandBarSymbolIconImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSymbol(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarSymbolIcon, BASE_OFFSET>(),
            Symbol: Symbol::<Impl, IMPL_OFFSET>,
            SetSymbol: SetSymbol::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarSymbolIcon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSymbolIconFactoryImpl: Sized {
    fn Create(&mut self, symbol: &::windows::core::HSTRING) -> ::windows::core::Result<WebUICommandBarSymbolIcon>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarSymbolIconFactory {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarSymbolIconFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarSymbolIconFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarSymbolIconFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUICommandBarSymbolIconFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebUICommandBarSymbolIconFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&symbol as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUICommandBarSymbolIconFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarSymbolIconFactory as ::windows::core::Interface>::IID
    }
}
