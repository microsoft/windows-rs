#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarImpl: Sized {
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetForegroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetBackgroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn ClosedDisplayMode(&self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode>;
    fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>;
    fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>;
    fn MenuOpened(&self, handler: &::core::option::Option<MenuOpenedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMenuOpened(&self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MenuClosed(&self, handler: &::core::option::Option<MenuClosedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMenuClosed(&self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<SizeChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBar";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarImpl, const OFFSET: isize>() -> IWebUICommandBarVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWebUICommandBar>,
            ::windows::core::GetTrustLevel,
            Visible::<Impl, OFFSET>,
            SetVisible::<Impl, OFFSET>,
            Opacity::<Impl, OFFSET>,
            SetOpacity::<Impl, OFFSET>,
            ForegroundColor::<Impl, OFFSET>,
            SetForegroundColor::<Impl, OFFSET>,
            BackgroundColor::<Impl, OFFSET>,
            SetBackgroundColor::<Impl, OFFSET>,
            ClosedDisplayMode::<Impl, OFFSET>,
            SetClosedDisplayMode::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            SetIsOpen::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            PrimaryCommands::<Impl, OFFSET>,
            SecondaryCommands::<Impl, OFFSET>,
            MenuOpened::<Impl, OFFSET>,
            RemoveMenuOpened::<Impl, OFFSET>,
            MenuClosed::<Impl, OFFSET>,
            RemoveMenuClosed::<Impl, OFFSET>,
            SizeChanged::<Impl, OFFSET>,
            RemoveSizeChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarBitmapIconImpl: Sized + IWebUICommandBarIconImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarBitmapIcon";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarBitmapIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarBitmapIconImpl, const OFFSET: isize>() -> IWebUICommandBarBitmapIconVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarBitmapIcon>, ::windows::core::GetTrustLevel, Uri::<Impl, OFFSET>, SetUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarBitmapIconFactoryImpl: Sized {
    fn Create(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebUICommandBarBitmapIcon>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarBitmapIconFactory {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarBitmapIconFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarBitmapIconFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarBitmapIconFactoryImpl, const OFFSET: isize>() -> IWebUICommandBarBitmapIconFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarBitmapIconFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarConfirmationButtonImpl: Sized + IWebUICommandBarElementImpl {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarConfirmationButton";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarConfirmationButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarConfirmationButtonImpl, const OFFSET: isize>() -> IWebUICommandBarConfirmationButtonVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarConfirmationButton>, ::windows::core::GetTrustLevel, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, ItemInvoked::<Impl, OFFSET>, RemoveItemInvoked::<Impl, OFFSET>)
    }
}
pub trait IWebUICommandBarElementImpl: Sized {}
impl ::windows::core::RuntimeName for IWebUICommandBarElement {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarElement";
}
impl IWebUICommandBarElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarElementImpl, const OFFSET: isize>() -> IWebUICommandBarElementVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarElement>, ::windows::core::GetTrustLevel)
    }
}
pub trait IWebUICommandBarIconImpl: Sized {}
impl ::windows::core::RuntimeName for IWebUICommandBarIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarIcon";
}
impl IWebUICommandBarIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarIconImpl, const OFFSET: isize>() -> IWebUICommandBarIconVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarIcon>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarIconButtonImpl: Sized + IWebUICommandBarElementImpl {
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsToggleButton(&self) -> ::windows::core::Result<bool>;
    fn SetIsToggleButton(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<IWebUICommandBarIcon>;
    fn SetIcon(&self, value: &::core::option::Option<IWebUICommandBarIcon>) -> ::windows::core::Result<()>;
    fn ItemInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarIconButton";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarIconButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarIconButtonImpl, const OFFSET: isize>() -> IWebUICommandBarIconButtonVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWebUICommandBarIconButton>,
            ::windows::core::GetTrustLevel,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            IsToggleButton::<Impl, OFFSET>,
            SetIsToggleButton::<Impl, OFFSET>,
            IsChecked::<Impl, OFFSET>,
            SetIsChecked::<Impl, OFFSET>,
            Icon::<Impl, OFFSET>,
            SetIcon::<Impl, OFFSET>,
            ItemInvoked::<Impl, OFFSET>,
            RemoveItemInvoked::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarItemInvokedEventArgsImpl: Sized {
    fn IsPrimaryCommand(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarItemInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarItemInvokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarItemInvokedEventArgsImpl, const OFFSET: isize>() -> IWebUICommandBarItemInvokedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarItemInvokedEventArgs>, ::windows::core::GetTrustLevel, IsPrimaryCommand::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSizeChangedEventArgsImpl: Sized {
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarSizeChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarSizeChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarSizeChangedEventArgsImpl, const OFFSET: isize>() -> IWebUICommandBarSizeChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarSizeChangedEventArgs>, ::windows::core::GetTrustLevel, Size::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<WebUICommandBar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarStatics {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarStaticsImpl, const OFFSET: isize>() -> IWebUICommandBarStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSymbolIconImpl: Sized + IWebUICommandBarIconImpl {
    fn Symbol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSymbol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarSymbolIcon";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarSymbolIconVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarSymbolIconImpl, const OFFSET: isize>() -> IWebUICommandBarSymbolIconVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarSymbolIcon>, ::windows::core::GetTrustLevel, Symbol::<Impl, OFFSET>, SetSymbol::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSymbolIconFactoryImpl: Sized {
    fn Create(&self, symbol: &::windows::core::HSTRING) -> ::windows::core::Result<WebUICommandBarSymbolIcon>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUICommandBarSymbolIconFactory {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarSymbolIconFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUICommandBarSymbolIconFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUICommandBarSymbolIconFactoryImpl, const OFFSET: isize>() -> IWebUICommandBarSymbolIconFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebUICommandBarSymbolIconFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
