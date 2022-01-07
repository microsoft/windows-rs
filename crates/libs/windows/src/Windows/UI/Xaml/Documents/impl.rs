#[cfg(feature = "implement_exclusive")]
pub trait IBlockImpl: Sized {
    fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<f64>;
    fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy>;
    fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()>;
    fn Margin(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetMargin(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBlock {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IBlock";
}
#[cfg(feature = "implement_exclusive")]
impl IBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockImpl, const OFFSET: isize>() -> IBlockVtbl {
        unsafe extern "system" fn TextAlignment<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::TextAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextAlignment<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::TextAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextAlignment(value).into()
        }
        unsafe extern "system" fn LineHeight<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineHeight(value).into()
        }
        unsafe extern "system" fn LineStackingStrategy<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::LineStackingStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineStackingStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStackingStrategy<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::LineStackingStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineStackingStrategy(value).into()
        }
        unsafe extern "system" fn Margin<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Margin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMargin<Impl: IBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMargin(&*(&value as *const <super::Thickness as ::windows::core::Abi>::Abi as *const <super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBlock>,
            ::windows::core::GetTrustLevel,
            TextAlignment::<Impl, OFFSET>,
            SetTextAlignment::<Impl, OFFSET>,
            LineHeight::<Impl, OFFSET>,
            SetLineHeight::<Impl, OFFSET>,
            LineStackingStrategy::<Impl, OFFSET>,
            SetLineStackingStrategy::<Impl, OFFSET>,
            Margin::<Impl, OFFSET>,
            SetMargin::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlock2Impl: Sized {
    fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBlock2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IBlock2";
}
#[cfg(feature = "implement_exclusive")]
impl IBlock2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlock2Impl, const OFFSET: isize>() -> IBlock2Vtbl {
        unsafe extern "system" fn HorizontalTextAlignment<Impl: IBlock2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::TextAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalTextAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalTextAlignment<Impl: IBlock2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::TextAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalTextAlignment(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlock2>, ::windows::core::GetTrustLevel, HorizontalTextAlignment::<Impl, OFFSET>, SetHorizontalTextAlignment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlockFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Block>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBlockFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IBlockFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBlockFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockFactoryImpl, const OFFSET: isize>() -> IBlockFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBlockFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlockFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlockStaticsImpl: Sized {
    fn TextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineStackingStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MarginProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBlockStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IBlockStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBlockStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockStaticsImpl, const OFFSET: isize>() -> IBlockStaticsVtbl {
        unsafe extern "system" fn TextAlignmentProperty<Impl: IBlockStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineHeightProperty<Impl: IBlockStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineStackingStrategyProperty<Impl: IBlockStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineStackingStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginProperty<Impl: IBlockStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlockStatics>, ::windows::core::GetTrustLevel, TextAlignmentProperty::<Impl, OFFSET>, LineHeightProperty::<Impl, OFFSET>, LineStackingStrategyProperty::<Impl, OFFSET>, MarginProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlockStatics2Impl: Sized {
    fn HorizontalTextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBlockStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IBlockStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBlockStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockStatics2Impl, const OFFSET: isize>() -> IBlockStatics2Vtbl {
        unsafe extern "system" fn HorizontalTextAlignmentProperty<Impl: IBlockStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalTextAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlockStatics2>, ::windows::core::GetTrustLevel, HorizontalTextAlignmentProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBoldImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBold {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IBold";
}
#[cfg(feature = "implement_exclusive")]
impl IBoldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBoldImpl, const OFFSET: isize>() -> IBoldVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBold>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactContentLinkProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactContentLinkProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContactContentLinkProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IContactContentLinkProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactContentLinkProviderImpl, const OFFSET: isize>() -> IContactContentLinkProviderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactContentLinkProvider>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkImpl: Sized {
    fn Info(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo>;
    fn SetInfo(&self, value: &::core::option::Option<super::super::Text::ContentLinkInfo>) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Cursor(&self) -> ::windows::core::Result<super::super::Core::CoreCursorType>;
    fn SetCursor(&self, value: super::super::Core::CoreCursorType) -> ::windows::core::Result<()>;
    fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusLeft(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusRight(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusUp(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusDown(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode>;
    fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn IsTabStop(&self) -> ::windows::core::Result<bool>;
    fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()>;
    fn TabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentLink, ContentLinkInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLink {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContentLink";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkImpl, const OFFSET: isize>() -> IContentLinkVtbl {
        unsafe extern "system" fn Info<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfo<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfo(&*(&value as *const <super::super::Text::ContentLinkInfo as ::windows::core::Abi>::Abi as *const <super::super::Text::ContentLinkInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Background<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Background() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackground<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackground(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Cursor<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreCursorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursor<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreCursorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCursor(value).into()
        }
        unsafe extern "system" fn XYFocusLeft<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusLeft<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusLeft(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusRight<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusRight<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusRight(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusUp<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusUp<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusUp(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusDown<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusDown<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusDown(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ElementSoundMode<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementSoundMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementSoundMode<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElementSoundMode(value).into()
        }
        unsafe extern "system" fn FocusState<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusUpNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusUpNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusDownNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusDownNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusLeftNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusLeftNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusRightNavigationStrategy<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusRightNavigationStrategy(value).into()
        }
        unsafe extern "system" fn IsTabStop<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTabStop<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTabStop(value).into()
        }
        unsafe extern "system" fn TabIndex<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTabIndex<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTabIndex(value).into()
        }
        unsafe extern "system" fn Invoked<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContentLink, ContentLinkInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContentLink, ContentLinkInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInvoked<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInvoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GotFocus<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Focus<Impl: IContentLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Focus(value) {
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
            ::windows::core::GetRuntimeClassName::<IContentLink>,
            ::windows::core::GetTrustLevel,
            Info::<Impl, OFFSET>,
            SetInfo::<Impl, OFFSET>,
            Background::<Impl, OFFSET>,
            SetBackground::<Impl, OFFSET>,
            Cursor::<Impl, OFFSET>,
            SetCursor::<Impl, OFFSET>,
            XYFocusLeft::<Impl, OFFSET>,
            SetXYFocusLeft::<Impl, OFFSET>,
            XYFocusRight::<Impl, OFFSET>,
            SetXYFocusRight::<Impl, OFFSET>,
            XYFocusUp::<Impl, OFFSET>,
            SetXYFocusUp::<Impl, OFFSET>,
            XYFocusDown::<Impl, OFFSET>,
            SetXYFocusDown::<Impl, OFFSET>,
            ElementSoundMode::<Impl, OFFSET>,
            SetElementSoundMode::<Impl, OFFSET>,
            FocusState::<Impl, OFFSET>,
            XYFocusUpNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusUpNavigationStrategy::<Impl, OFFSET>,
            XYFocusDownNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusDownNavigationStrategy::<Impl, OFFSET>,
            XYFocusLeftNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusLeftNavigationStrategy::<Impl, OFFSET>,
            XYFocusRightNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusRightNavigationStrategy::<Impl, OFFSET>,
            IsTabStop::<Impl, OFFSET>,
            SetIsTabStop::<Impl, OFFSET>,
            TabIndex::<Impl, OFFSET>,
            SetTabIndex::<Impl, OFFSET>,
            Invoked::<Impl, OFFSET>,
            RemoveInvoked::<Impl, OFFSET>,
            GotFocus::<Impl, OFFSET>,
            RemoveGotFocus::<Impl, OFFSET>,
            LostFocus::<Impl, OFFSET>,
            RemoveLostFocus::<Impl, OFFSET>,
            Focus::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkInvokedEventArgsImpl: Sized {
    fn ContentLinkInfo(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLinkInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkInvokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkInvokedEventArgsImpl, const OFFSET: isize>() -> IContentLinkInvokedEventArgsVtbl {
        unsafe extern "system" fn ContentLinkInfo<Impl: IContentLinkInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLinkInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IContentLinkInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IContentLinkInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentLinkInvokedEventArgs>, ::windows::core::GetTrustLevel, ContentLinkInfo::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLinkProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContentLinkProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkProviderImpl, const OFFSET: isize>() -> IContentLinkProviderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentLinkProvider>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkProviderCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLinkProviderCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContentLinkProviderCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkProviderCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkProviderCollectionImpl, const OFFSET: isize>() -> IContentLinkProviderCollectionVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentLinkProviderCollection>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkProviderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ContentLinkProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLinkProviderFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContentLinkProviderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkProviderFactoryImpl, const OFFSET: isize>() -> IContentLinkProviderFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IContentLinkProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentLinkProviderFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkStaticsImpl: Sized {
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CursorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FocusStateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTabStopProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TabIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLinkStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IContentLinkStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkStaticsImpl, const OFFSET: isize>() -> IContentLinkStaticsVtbl {
        unsafe extern "system" fn BackgroundProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CursorProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CursorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusLeftProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusRightProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusDownProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementSoundModeProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementSoundModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusStateProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategyProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategyProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategyProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategyProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTabStopProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTabStopProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TabIndexProperty<Impl: IContentLinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabIndexProperty() {
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
            ::windows::core::GetRuntimeClassName::<IContentLinkStatics>,
            ::windows::core::GetTrustLevel,
            BackgroundProperty::<Impl, OFFSET>,
            CursorProperty::<Impl, OFFSET>,
            XYFocusLeftProperty::<Impl, OFFSET>,
            XYFocusRightProperty::<Impl, OFFSET>,
            XYFocusUpProperty::<Impl, OFFSET>,
            XYFocusDownProperty::<Impl, OFFSET>,
            ElementSoundModeProperty::<Impl, OFFSET>,
            FocusStateProperty::<Impl, OFFSET>,
            XYFocusUpNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusDownNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusLeftNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusRightNavigationStrategyProperty::<Impl, OFFSET>,
            IsTabStopProperty::<Impl, OFFSET>,
            TabIndexProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphsImpl: Sized {
    fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUnicodeString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIndices(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetFontUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations>;
    fn SetStyleSimulations(&self, value: super::Media::StyleSimulations) -> ::windows::core::Result<()>;
    fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64>;
    fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn OriginX(&self) -> ::windows::core::Result<f64>;
    fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()>;
    fn OriginY(&self) -> ::windows::core::Result<f64>;
    fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()>;
    fn Fill(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlyphs {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IGlyphs";
}
#[cfg(feature = "implement_exclusive")]
impl IGlyphsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlyphsImpl, const OFFSET: isize>() -> IGlyphsVtbl {
        unsafe extern "system" fn UnicodeString<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicodeString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Indices<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Indices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndices<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndices(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontUri<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontUri<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StyleSimulations<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::StyleSimulations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::StyleSimulations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyleSimulations(value).into()
        }
        unsafe extern "system" fn FontRenderingEmSize<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontRenderingEmSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontRenderingEmSize(value).into()
        }
        unsafe extern "system" fn OriginX<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginX<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginX(value).into()
        }
        unsafe extern "system" fn OriginY<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginY<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginY(value).into()
        }
        unsafe extern "system" fn Fill<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fill() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFill<Impl: IGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFill(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGlyphs>,
            ::windows::core::GetTrustLevel,
            UnicodeString::<Impl, OFFSET>,
            SetUnicodeString::<Impl, OFFSET>,
            Indices::<Impl, OFFSET>,
            SetIndices::<Impl, OFFSET>,
            FontUri::<Impl, OFFSET>,
            SetFontUri::<Impl, OFFSET>,
            StyleSimulations::<Impl, OFFSET>,
            SetStyleSimulations::<Impl, OFFSET>,
            FontRenderingEmSize::<Impl, OFFSET>,
            SetFontRenderingEmSize::<Impl, OFFSET>,
            OriginX::<Impl, OFFSET>,
            SetOriginX::<Impl, OFFSET>,
            OriginY::<Impl, OFFSET>,
            SetOriginY::<Impl, OFFSET>,
            Fill::<Impl, OFFSET>,
            SetFill::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphs2Impl: Sized {
    fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32>;
    fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlyphs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IGlyphs2";
}
#[cfg(feature = "implement_exclusive")]
impl IGlyphs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlyphs2Impl, const OFFSET: isize>() -> IGlyphs2Vtbl {
        unsafe extern "system" fn IsColorFontEnabled<Impl: IGlyphs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsColorFontEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsColorFontEnabled<Impl: IGlyphs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsColorFontEnabled(value).into()
        }
        unsafe extern "system" fn ColorFontPaletteIndex<Impl: IGlyphs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorFontPaletteIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorFontPaletteIndex<Impl: IGlyphs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorFontPaletteIndex(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGlyphs2>, ::windows::core::GetTrustLevel, IsColorFontEnabled::<Impl, OFFSET>, SetIsColorFontEnabled::<Impl, OFFSET>, ColorFontPaletteIndex::<Impl, OFFSET>, SetColorFontPaletteIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphsStaticsImpl: Sized {
    fn UnicodeStringProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IndicesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StyleSimulationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontRenderingEmSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OriginXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OriginYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FillProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlyphsStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IGlyphsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGlyphsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlyphsStaticsImpl, const OFFSET: isize>() -> IGlyphsStaticsVtbl {
        unsafe extern "system" fn UnicodeStringProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicodeStringProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicesProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndicesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontUriProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontUriProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleSimulationsProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleSimulationsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontRenderingEmSizeProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontRenderingEmSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginXProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginYProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillProperty<Impl: IGlyphsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillProperty() {
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
            ::windows::core::GetRuntimeClassName::<IGlyphsStatics>,
            ::windows::core::GetTrustLevel,
            UnicodeStringProperty::<Impl, OFFSET>,
            IndicesProperty::<Impl, OFFSET>,
            FontUriProperty::<Impl, OFFSET>,
            StyleSimulationsProperty::<Impl, OFFSET>,
            FontRenderingEmSizeProperty::<Impl, OFFSET>,
            OriginXProperty::<Impl, OFFSET>,
            OriginYProperty::<Impl, OFFSET>,
            FillProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphsStatics2Impl: Sized {
    fn IsColorFontEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorFontPaletteIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGlyphsStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IGlyphsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGlyphsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlyphsStatics2Impl, const OFFSET: isize>() -> IGlyphsStatics2Vtbl {
        unsafe extern "system" fn IsColorFontEnabledProperty<Impl: IGlyphsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsColorFontEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorFontPaletteIndexProperty<Impl: IGlyphsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorFontPaletteIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGlyphsStatics2>, ::windows::core::GetTrustLevel, IsColorFontEnabledProperty::<Impl, OFFSET>, ColorFontPaletteIndexProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkImpl: Sized {
    fn NavigateUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetNavigateUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Click(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlink {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlink";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkImpl, const OFFSET: isize>() -> IHyperlinkVtbl {
        unsafe extern "system" fn NavigateUri<Impl: IHyperlinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigateUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNavigateUri<Impl: IHyperlinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNavigateUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Click<Impl: IHyperlinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Click(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClick<Impl: IHyperlinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClick(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlink>, ::windows::core::GetTrustLevel, NavigateUri::<Impl, OFFSET>, SetNavigateUri::<Impl, OFFSET>, Click::<Impl, OFFSET>, RemoveClick::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink2Impl: Sized {
    fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle>;
    fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlink2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlink2";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlink2Impl, const OFFSET: isize>() -> IHyperlink2Vtbl {
        unsafe extern "system" fn UnderlineStyle<Impl: IHyperlink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnderlineStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnderlineStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderlineStyle<Impl: IHyperlink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnderlineStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnderlineStyle(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlink2>, ::windows::core::GetTrustLevel, UnderlineStyle::<Impl, OFFSET>, SetUnderlineStyle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink3Impl: Sized {
    fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusLeft(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusRight(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusUp(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusDown(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode>;
    fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlink3 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlink3";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlink3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlink3Impl, const OFFSET: isize>() -> IHyperlink3Vtbl {
        unsafe extern "system" fn XYFocusLeft<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusLeft<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusLeft(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusRight<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusRight<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusRight(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusUp<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusUp<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusUp(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusDown<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusDown<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusDown(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ElementSoundMode<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementSoundMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementSoundMode<Impl: IHyperlink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElementSoundMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHyperlink3>,
            ::windows::core::GetTrustLevel,
            XYFocusLeft::<Impl, OFFSET>,
            SetXYFocusLeft::<Impl, OFFSET>,
            XYFocusRight::<Impl, OFFSET>,
            SetXYFocusRight::<Impl, OFFSET>,
            XYFocusUp::<Impl, OFFSET>,
            SetXYFocusUp::<Impl, OFFSET>,
            XYFocusDown::<Impl, OFFSET>,
            SetXYFocusDown::<Impl, OFFSET>,
            ElementSoundMode::<Impl, OFFSET>,
            SetElementSoundMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink4Impl: Sized {
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlink4 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlink4";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlink4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlink4Impl, const OFFSET: isize>() -> IHyperlink4Vtbl {
        unsafe extern "system" fn FocusState<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusUpNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusUpNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusDownNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusDownNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusLeftNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusLeftNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusRightNavigationStrategy<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusRightNavigationStrategy(value).into()
        }
        unsafe extern "system" fn GotFocus<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Focus<Impl: IHyperlink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Focus(value) {
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
            ::windows::core::GetRuntimeClassName::<IHyperlink4>,
            ::windows::core::GetTrustLevel,
            FocusState::<Impl, OFFSET>,
            XYFocusUpNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusUpNavigationStrategy::<Impl, OFFSET>,
            XYFocusDownNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusDownNavigationStrategy::<Impl, OFFSET>,
            XYFocusLeftNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusLeftNavigationStrategy::<Impl, OFFSET>,
            XYFocusRightNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusRightNavigationStrategy::<Impl, OFFSET>,
            GotFocus::<Impl, OFFSET>,
            RemoveGotFocus::<Impl, OFFSET>,
            LostFocus::<Impl, OFFSET>,
            RemoveLostFocus::<Impl, OFFSET>,
            Focus::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink5Impl: Sized {
    fn IsTabStop(&self) -> ::windows::core::Result<bool>;
    fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()>;
    fn TabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlink5 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlink5";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlink5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlink5Impl, const OFFSET: isize>() -> IHyperlink5Vtbl {
        unsafe extern "system" fn IsTabStop<Impl: IHyperlink5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTabStop<Impl: IHyperlink5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTabStop(value).into()
        }
        unsafe extern "system" fn TabIndex<Impl: IHyperlink5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTabIndex<Impl: IHyperlink5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTabIndex(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlink5>, ::windows::core::GetTrustLevel, IsTabStop::<Impl, OFFSET>, SetIsTabStop::<Impl, OFFSET>, TabIndex::<Impl, OFFSET>, SetTabIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkClickEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkClickEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkClickEventArgsImpl, const OFFSET: isize>() -> IHyperlinkClickEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkClickEventArgs>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStaticsImpl: Sized {
    fn NavigateUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlinkStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkStaticsImpl, const OFFSET: isize>() -> IHyperlinkStaticsVtbl {
        unsafe extern "system" fn NavigateUriProperty<Impl: IHyperlinkStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigateUriProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkStatics>, ::windows::core::GetTrustLevel, NavigateUriProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics2Impl: Sized {
    fn UnderlineStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlinkStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkStatics2Impl, const OFFSET: isize>() -> IHyperlinkStatics2Vtbl {
        unsafe extern "system" fn UnderlineStyleProperty<Impl: IHyperlinkStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnderlineStyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkStatics2>, ::windows::core::GetTrustLevel, UnderlineStyleProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics3Impl: Sized {
    fn XYFocusLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlinkStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkStatics3Impl, const OFFSET: isize>() -> IHyperlinkStatics3Vtbl {
        unsafe extern "system" fn XYFocusLeftProperty<Impl: IHyperlinkStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusRightProperty<Impl: IHyperlinkStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpProperty<Impl: IHyperlinkStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusDownProperty<Impl: IHyperlinkStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementSoundModeProperty<Impl: IHyperlinkStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementSoundModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkStatics3>, ::windows::core::GetTrustLevel, XYFocusLeftProperty::<Impl, OFFSET>, XYFocusRightProperty::<Impl, OFFSET>, XYFocusUpProperty::<Impl, OFFSET>, XYFocusDownProperty::<Impl, OFFSET>, ElementSoundModeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics4Impl: Sized {
    fn FocusStateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlinkStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkStatics4Impl, const OFFSET: isize>() -> IHyperlinkStatics4Vtbl {
        unsafe extern "system" fn FocusStateProperty<Impl: IHyperlinkStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategyProperty<Impl: IHyperlinkStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategyProperty<Impl: IHyperlinkStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategyProperty<Impl: IHyperlinkStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategyProperty<Impl: IHyperlinkStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategyProperty() {
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
            ::windows::core::GetRuntimeClassName::<IHyperlinkStatics4>,
            ::windows::core::GetTrustLevel,
            FocusStateProperty::<Impl, OFFSET>,
            XYFocusUpNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusDownNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusLeftNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusRightNavigationStrategyProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics5Impl: Sized {
    fn IsTabStopProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TabIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IHyperlinkStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkStatics5Impl, const OFFSET: isize>() -> IHyperlinkStatics5Vtbl {
        unsafe extern "system" fn IsTabStopProperty<Impl: IHyperlinkStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTabStopProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TabIndexProperty<Impl: IHyperlinkStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkStatics5>, ::windows::core::GetTrustLevel, IsTabStopProperty::<Impl, OFFSET>, TabIndexProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInlineImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInline {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IInline";
}
#[cfg(feature = "implement_exclusive")]
impl IInlineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInlineImpl, const OFFSET: isize>() -> IInlineVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInline>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInlineFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Inline>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInlineFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IInlineFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInlineFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInlineFactoryImpl, const OFFSET: isize>() -> IInlineFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IInlineFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInlineFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInlineUIContainerImpl: Sized {
    fn Child(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInlineUIContainer {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IInlineUIContainer";
}
#[cfg(feature = "implement_exclusive")]
impl IInlineUIContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInlineUIContainerImpl, const OFFSET: isize>() -> IInlineUIContainerVtbl {
        unsafe extern "system" fn Child<Impl: IInlineUIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Child() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChild<Impl: IInlineUIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChild(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInlineUIContainer>, ::windows::core::GetTrustLevel, Child::<Impl, OFFSET>, SetChild::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItalicImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItalic {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IItalic";
}
#[cfg(feature = "implement_exclusive")]
impl IItalicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItalicImpl, const OFFSET: isize>() -> IItalicVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItalic>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineBreakImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineBreak {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ILineBreak";
}
#[cfg(feature = "implement_exclusive")]
impl ILineBreakVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineBreakImpl, const OFFSET: isize>() -> ILineBreakVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILineBreak>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IParagraphImpl: Sized {
    fn Inlines(&self) -> ::windows::core::Result<InlineCollection>;
    fn TextIndent(&self) -> ::windows::core::Result<f64>;
    fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IParagraph {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IParagraph";
}
#[cfg(feature = "implement_exclusive")]
impl IParagraphVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParagraphImpl, const OFFSET: isize>() -> IParagraphVtbl {
        unsafe extern "system" fn Inlines<Impl: IParagraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inlines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextIndent<Impl: IParagraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextIndent<Impl: IParagraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextIndent(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IParagraph>, ::windows::core::GetTrustLevel, Inlines::<Impl, OFFSET>, TextIndent::<Impl, OFFSET>, SetTextIndent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IParagraphStaticsImpl: Sized {
    fn TextIndentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IParagraphStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IParagraphStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IParagraphStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParagraphStaticsImpl, const OFFSET: isize>() -> IParagraphStaticsVtbl {
        unsafe extern "system" fn TextIndentProperty<Impl: IParagraphStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextIndentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IParagraphStatics>, ::windows::core::GetTrustLevel, TextIndentProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceContentLinkProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaceContentLinkProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IPlaceContentLinkProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaceContentLinkProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaceContentLinkProviderImpl, const OFFSET: isize>() -> IPlaceContentLinkProviderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaceContentLinkProvider>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRunImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection>;
    fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRun {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IRun";
}
#[cfg(feature = "implement_exclusive")]
impl IRunVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunImpl, const OFFSET: isize>() -> IRunVtbl {
        unsafe extern "system" fn Text<Impl: IRunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetText<Impl: IRunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlowDirection<Impl: IRunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Impl: IRunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::FlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlowDirection(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRun>, ::windows::core::GetTrustLevel, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, FlowDirection::<Impl, OFFSET>, SetFlowDirection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRunStaticsImpl: Sized {
    fn FlowDirectionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRunStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IRunStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRunStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunStaticsImpl, const OFFSET: isize>() -> IRunStaticsVtbl {
        unsafe extern "system" fn FlowDirectionProperty<Impl: IRunStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRunStatics>, ::windows::core::GetTrustLevel, FlowDirectionProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpanImpl: Sized {
    fn Inlines(&self) -> ::windows::core::Result<InlineCollection>;
    fn SetInlines(&self, value: &::core::option::Option<InlineCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpan {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ISpan";
}
#[cfg(feature = "implement_exclusive")]
impl ISpanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpanImpl, const OFFSET: isize>() -> ISpanVtbl {
        unsafe extern "system" fn Inlines<Impl: ISpanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inlines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInlines<Impl: ISpanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInlines(&*(&value as *const <InlineCollection as ::windows::core::Abi>::Abi as *const <InlineCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpan>, ::windows::core::GetTrustLevel, Inlines::<Impl, OFFSET>, SetInlines::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpanFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Span>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpanFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ISpanFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpanFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpanFactoryImpl, const OFFSET: isize>() -> ISpanFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISpanFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpanFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FontSize(&self) -> ::windows::core::Result<f64>;
    fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn FontStretch(&self) -> ::windows::core::Result<super::super::Text::FontStretch>;
    fn SetFontStretch(&self, value: super::super::Text::FontStretch) -> ::windows::core::Result<()>;
    fn CharacterSpacing(&self) -> ::windows::core::Result<i32>;
    fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()>;
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentStart(&self) -> ::windows::core::Result<TextPointer>;
    fn ContentEnd(&self) -> ::windows::core::Result<TextPointer>;
    fn ElementStart(&self) -> ::windows::core::Result<TextPointer>;
    fn ElementEnd(&self) -> ::windows::core::Result<TextPointer>;
    fn FindName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElement {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElement";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementImpl, const OFFSET: isize>() -> ITextElementVtbl {
        unsafe extern "system" fn Name<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontSize<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontSize<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontSize(value).into()
        }
        unsafe extern "system" fn FontFamily<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFamily<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontFamily(&*(&value as *const <super::Media::FontFamily as ::windows::core::Abi>::Abi as *const <super::Media::FontFamily as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontWeight<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontWeight<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Text::FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontWeight(&*(&value as *const <super::super::Text::FontWeight as ::windows::core::Abi>::Abi as *const <super::super::Text::FontWeight as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontStyle<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Text::FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontStyle(value).into()
        }
        unsafe extern "system" fn FontStretch<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStretch<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Text::FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontStretch(value).into()
        }
        unsafe extern "system" fn CharacterSpacing<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSpacing<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacterSpacing(value).into()
        }
        unsafe extern "system" fn Foreground<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Foreground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeground<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForeground(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentStart<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentEnd<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementStart<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementEnd<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindName<Impl: ITextElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ITextElement>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            FontSize::<Impl, OFFSET>,
            SetFontSize::<Impl, OFFSET>,
            FontFamily::<Impl, OFFSET>,
            SetFontFamily::<Impl, OFFSET>,
            FontWeight::<Impl, OFFSET>,
            SetFontWeight::<Impl, OFFSET>,
            FontStyle::<Impl, OFFSET>,
            SetFontStyle::<Impl, OFFSET>,
            FontStretch::<Impl, OFFSET>,
            SetFontStretch::<Impl, OFFSET>,
            CharacterSpacing::<Impl, OFFSET>,
            SetCharacterSpacing::<Impl, OFFSET>,
            Foreground::<Impl, OFFSET>,
            SetForeground::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            SetLanguage::<Impl, OFFSET>,
            ContentStart::<Impl, OFFSET>,
            ContentEnd::<Impl, OFFSET>,
            ElementStart::<Impl, OFFSET>,
            ElementEnd::<Impl, OFFSET>,
            FindName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement2Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElement2";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElement2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElement2Impl, const OFFSET: isize>() -> ITextElement2Vtbl {
        unsafe extern "system" fn IsTextScaleFactorEnabled<Impl: ITextElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTextScaleFactorEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextScaleFactorEnabled<Impl: ITextElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTextScaleFactorEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextElement2>, ::windows::core::GetTrustLevel, IsTextScaleFactorEnabled::<Impl, OFFSET>, SetIsTextScaleFactorEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement3Impl: Sized {
    fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool>;
    fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElement3";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElement3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElement3Impl, const OFFSET: isize>() -> ITextElement3Vtbl {
        unsafe extern "system" fn AllowFocusOnInteraction<Impl: ITextElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteraction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusOnInteraction<Impl: ITextElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusOnInteraction(value).into()
        }
        unsafe extern "system" fn AccessKey<Impl: ITextElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKey<Impl: ITextElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvoked<Impl: ITextElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitDisplayModeOnAccessKeyInvoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitDisplayModeOnAccessKeyInvoked<Impl: ITextElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitDisplayModeOnAccessKeyInvoked(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextElement3>,
            ::windows::core::GetTrustLevel,
            AllowFocusOnInteraction::<Impl, OFFSET>,
            SetAllowFocusOnInteraction::<Impl, OFFSET>,
            AccessKey::<Impl, OFFSET>,
            SetAccessKey::<Impl, OFFSET>,
            ExitDisplayModeOnAccessKeyInvoked::<Impl, OFFSET>,
            SetExitDisplayModeOnAccessKeyInvoked::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement4Impl: Sized {
    fn TextDecorations(&self) -> ::windows::core::Result<super::super::Text::TextDecorations>;
    fn SetTextDecorations(&self, value: super::super::Text::TextDecorations) -> ::windows::core::Result<()>;
    fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool>;
    fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetAccessKeyScopeOwner(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyTipPlacementMode(&self) -> ::windows::core::Result<super::Input::KeyTipPlacementMode>;
    fn SetKeyTipPlacementMode(&self, value: super::Input::KeyTipPlacementMode) -> ::windows::core::Result<()>;
    fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayDismissed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayDismissedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayDismissed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElement4";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElement4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElement4Impl, const OFFSET: isize>() -> ITextElement4Vtbl {
        unsafe extern "system" fn TextDecorations<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::TextDecorations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextDecorations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextDecorations<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Text::TextDecorations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextDecorations(value).into()
        }
        unsafe extern "system" fn IsAccessKeyScope<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAccessKeyScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccessKeyScope<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAccessKeyScope(value).into()
        }
        unsafe extern "system" fn AccessKeyScopeOwner<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyScopeOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKeyScopeOwner<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKeyScopeOwner(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTipPlacementMode<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Input::KeyTipPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipPlacementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipPlacementMode<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Input::KeyTipPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipPlacementMode(value).into()
        }
        unsafe extern "system" fn KeyTipHorizontalOffset<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipHorizontalOffset<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipHorizontalOffset(value).into()
        }
        unsafe extern "system" fn KeyTipVerticalOffset<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipVerticalOffset<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipVerticalOffset(value).into()
        }
        unsafe extern "system" fn AccessKeyDisplayRequested<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyDisplayRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayRequested<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyDisplayRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyDisplayDismissed<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyDisplayDismissed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayDismissedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayDismissedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayDismissed<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyDisplayDismissed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyInvoked<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyInvoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyInvoked<Impl: ITextElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyInvoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextElement4>,
            ::windows::core::GetTrustLevel,
            TextDecorations::<Impl, OFFSET>,
            SetTextDecorations::<Impl, OFFSET>,
            IsAccessKeyScope::<Impl, OFFSET>,
            SetIsAccessKeyScope::<Impl, OFFSET>,
            AccessKeyScopeOwner::<Impl, OFFSET>,
            SetAccessKeyScopeOwner::<Impl, OFFSET>,
            KeyTipPlacementMode::<Impl, OFFSET>,
            SetKeyTipPlacementMode::<Impl, OFFSET>,
            KeyTipHorizontalOffset::<Impl, OFFSET>,
            SetKeyTipHorizontalOffset::<Impl, OFFSET>,
            KeyTipVerticalOffset::<Impl, OFFSET>,
            SetKeyTipVerticalOffset::<Impl, OFFSET>,
            AccessKeyDisplayRequested::<Impl, OFFSET>,
            RemoveAccessKeyDisplayRequested::<Impl, OFFSET>,
            AccessKeyDisplayDismissed::<Impl, OFFSET>,
            RemoveAccessKeyDisplayDismissed::<Impl, OFFSET>,
            AccessKeyInvoked::<Impl, OFFSET>,
            RemoveAccessKeyInvoked::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement5Impl: Sized {
    fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot>;
    fn SetXamlRoot(&self, value: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElement5 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElement5";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElement5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElement5Impl, const OFFSET: isize>() -> ITextElement5Vtbl {
        unsafe extern "system" fn XamlRoot<Impl: ITextElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XamlRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXamlRoot<Impl: ITextElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXamlRoot(&*(&value as *const <super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::XamlRoot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextElement5>, ::windows::core::GetTrustLevel, XamlRoot::<Impl, OFFSET>, SetXamlRoot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementFactoryImpl, const OFFSET: isize>() -> ITextElementFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextElementFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementOverridesImpl: Sized {
    fn OnDisconnectVisualChildren(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementOverridesImpl, const OFFSET: isize>() -> ITextElementOverridesVtbl {
        unsafe extern "system" fn OnDisconnectVisualChildren<Impl: ITextElementOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnectVisualChildren().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextElementOverrides>, ::windows::core::GetTrustLevel, OnDisconnectVisualChildren::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStaticsImpl: Sized {
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LanguageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementStaticsImpl, const OFFSET: isize>() -> ITextElementStaticsVtbl {
        unsafe extern "system" fn FontSizeProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontFamilyProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontFamilyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontWeightProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontWeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStyleProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStretchProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStretchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterSpacingProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterSpacingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForegroundProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageProperty<Impl: ITextElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageProperty() {
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
            ::windows::core::GetRuntimeClassName::<ITextElementStatics>,
            ::windows::core::GetTrustLevel,
            FontSizeProperty::<Impl, OFFSET>,
            FontFamilyProperty::<Impl, OFFSET>,
            FontWeightProperty::<Impl, OFFSET>,
            FontStyleProperty::<Impl, OFFSET>,
            FontStretchProperty::<Impl, OFFSET>,
            CharacterSpacingProperty::<Impl, OFFSET>,
            ForegroundProperty::<Impl, OFFSET>,
            LanguageProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStatics2Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementStatics2Impl, const OFFSET: isize>() -> ITextElementStatics2Vtbl {
        unsafe extern "system" fn IsTextScaleFactorEnabledProperty<Impl: ITextElementStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTextScaleFactorEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextElementStatics2>, ::windows::core::GetTrustLevel, IsTextScaleFactorEnabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStatics3Impl: Sized {
    fn AllowFocusOnInteractionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ExitDisplayModeOnAccessKeyInvokedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElementStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementStatics3Impl, const OFFSET: isize>() -> ITextElementStatics3Vtbl {
        unsafe extern "system" fn AllowFocusOnInteractionProperty<Impl: ITextElementStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteractionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: ITextElementStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvokedProperty<Impl: ITextElementStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitDisplayModeOnAccessKeyInvokedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextElementStatics3>, ::windows::core::GetTrustLevel, AllowFocusOnInteractionProperty::<Impl, OFFSET>, AccessKeyProperty::<Impl, OFFSET>, ExitDisplayModeOnAccessKeyInvokedProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStatics4Impl: Sized {
    fn TextDecorationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsAccessKeyScopeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyScopeOwnerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyTipPlacementModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyTipHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyTipVerticalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl ITextElementStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextElementStatics4Impl, const OFFSET: isize>() -> ITextElementStatics4Vtbl {
        unsafe extern "system" fn TextDecorationsProperty<Impl: ITextElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextDecorationsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAccessKeyScopeProperty<Impl: ITextElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAccessKeyScopeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyScopeOwnerProperty<Impl: ITextElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyScopeOwnerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipPlacementModeProperty<Impl: ITextElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipPlacementModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipHorizontalOffsetProperty<Impl: ITextElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipVerticalOffsetProperty<Impl: ITextElementStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipVerticalOffsetProperty() {
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
            ::windows::core::GetRuntimeClassName::<ITextElementStatics4>,
            ::windows::core::GetTrustLevel,
            TextDecorationsProperty::<Impl, OFFSET>,
            IsAccessKeyScopeProperty::<Impl, OFFSET>,
            AccessKeyScopeOwnerProperty::<Impl, OFFSET>,
            KeyTipPlacementModeProperty::<Impl, OFFSET>,
            KeyTipHorizontalOffsetProperty::<Impl, OFFSET>,
            KeyTipVerticalOffsetProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterImpl: Sized {
    fn Ranges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TextRange>>;
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextHighlighter {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextHighlighter";
}
#[cfg(feature = "implement_exclusive")]
impl ITextHighlighterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHighlighterImpl, const OFFSET: isize>() -> ITextHighlighterVtbl {
        unsafe extern "system" fn Ranges<Impl: ITextHighlighterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ranges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Foreground<Impl: ITextHighlighterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Foreground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeground<Impl: ITextHighlighterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForeground(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Background<Impl: ITextHighlighterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Background() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackground<Impl: ITextHighlighterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackground(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextHighlighter>, ::windows::core::GetTrustLevel, Ranges::<Impl, OFFSET>, Foreground::<Impl, OFFSET>, SetForeground::<Impl, OFFSET>, Background::<Impl, OFFSET>, SetBackground::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextHighlighterBase {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextHighlighterBase";
}
#[cfg(feature = "implement_exclusive")]
impl ITextHighlighterBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHighlighterBaseImpl, const OFFSET: isize>() -> ITextHighlighterBaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextHighlighterBase>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextHighlighterBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextHighlighterBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextHighlighterBaseFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHighlighterBaseFactoryImpl, const OFFSET: isize>() -> ITextHighlighterBaseFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextHighlighterBaseFactory>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextHighlighter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextHighlighterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextHighlighterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextHighlighterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHighlighterFactoryImpl, const OFFSET: isize>() -> ITextHighlighterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITextHighlighterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextHighlighterFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterStaticsImpl: Sized {
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextHighlighterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextHighlighterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITextHighlighterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHighlighterStaticsImpl, const OFFSET: isize>() -> ITextHighlighterStaticsVtbl {
        unsafe extern "system" fn ForegroundProperty<Impl: ITextHighlighterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundProperty<Impl: ITextHighlighterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextHighlighterStatics>, ::windows::core::GetTrustLevel, ForegroundProperty::<Impl, OFFSET>, BackgroundProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPointerImpl: Sized {
    fn Parent(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement>;
    fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection>;
    fn Offset(&self) -> ::windows::core::Result<i32>;
    fn GetCharacterRect(&self, direction: LogicalDirection) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetPositionAtOffset(&self, offset: i32, direction: LogicalDirection) -> ::windows::core::Result<TextPointer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextPointer {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextPointer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextPointerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPointerImpl, const OFFSET: isize>() -> ITextPointerVtbl {
        unsafe extern "system" fn Parent<Impl: ITextPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisualParent<Impl: ITextPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisualParent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDirection<Impl: ITextPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LogicalDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogicalDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offset<Impl: ITextPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacterRect<Impl: ITextPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: LogicalDirection, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacterRect(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionAtOffset<Impl: ITextPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, direction: LogicalDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionAtOffset(offset, direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextPointer>, ::windows::core::GetTrustLevel, Parent::<Impl, OFFSET>, VisualParent::<Impl, OFFSET>, LogicalDirection::<Impl, OFFSET>, Offset::<Impl, OFFSET>, GetCharacterRect::<Impl, OFFSET>, GetPositionAtOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITypographyImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITypography {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITypography";
}
#[cfg(feature = "implement_exclusive")]
impl ITypographyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypographyImpl, const OFFSET: isize>() -> ITypographyVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypography>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITypographyStaticsImpl: Sized {
    fn AnnotationAlternatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAnnotationAlternates(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetAnnotationAlternates(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn EastAsianExpertFormsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetEastAsianExpertForms(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetEastAsianExpertForms(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn EastAsianLanguageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetEastAsianLanguage(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontEastAsianLanguage>;
    fn SetEastAsianLanguage(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontEastAsianLanguage) -> ::windows::core::Result<()>;
    fn EastAsianWidthsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetEastAsianWidths(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontEastAsianWidths>;
    fn SetEastAsianWidths(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontEastAsianWidths) -> ::windows::core::Result<()>;
    fn StandardLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStandardLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStandardLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn ContextualLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetContextualLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetContextualLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn DiscretionaryLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDiscretionaryLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetDiscretionaryLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn HistoricalLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHistoricalLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetHistoricalLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StandardSwashesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStandardSwashes(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetStandardSwashes(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn ContextualSwashesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetContextualSwashes(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetContextualSwashes(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn ContextualAlternatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetContextualAlternates(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetContextualAlternates(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticAlternatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticAlternates(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetStylisticAlternates(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn StylisticSet1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet1(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet1(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet2(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet2(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet3Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet3(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet3(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet4Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet4(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet4(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet5Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet5(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet5(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet6Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet6(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet6(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet7Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet7(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet7(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet8Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet8(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet8(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet9Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet9(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet9(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet10Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet10(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet10(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet11Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet11(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet11(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet12Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet12(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet12(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet13Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet13(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet13(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet14Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet14(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet14(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet15Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet15(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet15(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet16Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet16(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet16(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet17Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet17(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet17(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet18Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet18(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet18(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet19Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet19(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet19(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet20Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet20(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet20(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn CapitalsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCapitals(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontCapitals>;
    fn SetCapitals(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontCapitals) -> ::windows::core::Result<()>;
    fn CapitalSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCapitalSpacing(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetCapitalSpacing(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn KerningProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetKerning(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetKerning(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn CaseSensitiveFormsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCaseSensitiveForms(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetCaseSensitiveForms(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn HistoricalFormsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHistoricalForms(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetHistoricalForms(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn FractionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFraction(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontFraction>;
    fn SetFraction(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontFraction) -> ::windows::core::Result<()>;
    fn NumeralStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetNumeralStyle(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontNumeralStyle>;
    fn SetNumeralStyle(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontNumeralStyle) -> ::windows::core::Result<()>;
    fn NumeralAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetNumeralAlignment(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontNumeralAlignment>;
    fn SetNumeralAlignment(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontNumeralAlignment) -> ::windows::core::Result<()>;
    fn SlashedZeroProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetSlashedZero(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetSlashedZero(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn MathematicalGreekProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetMathematicalGreek(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetMathematicalGreek(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn VariantsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetVariants(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontVariants>;
    fn SetVariants(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontVariants) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITypographyStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITypographyStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITypographyStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypographyStaticsImpl, const OFFSET: isize>() -> ITypographyStaticsVtbl {
        unsafe extern "system" fn AnnotationAlternatesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationAlternatesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationAlternates<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationAlternates(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnnotationAlternates<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnnotationAlternates(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn EastAsianExpertFormsProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EastAsianExpertFormsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEastAsianExpertForms<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEastAsianExpertForms(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEastAsianExpertForms<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEastAsianExpertForms(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn EastAsianLanguageProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EastAsianLanguageProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEastAsianLanguage<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontEastAsianLanguage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEastAsianLanguage(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEastAsianLanguage<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontEastAsianLanguage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEastAsianLanguage(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn EastAsianWidthsProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EastAsianWidthsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEastAsianWidths<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontEastAsianWidths) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEastAsianWidths(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEastAsianWidths<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontEastAsianWidths) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEastAsianWidths(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StandardLigaturesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StandardLigaturesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStandardLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStandardLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ContextualLigaturesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextualLigaturesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextualLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextualLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContextualLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContextualLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn DiscretionaryLigaturesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscretionaryLigaturesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiscretionaryLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiscretionaryLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscretionaryLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscretionaryLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn HistoricalLigaturesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoricalLigaturesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHistoricalLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHistoricalLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoricalLigatures<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHistoricalLigatures(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StandardSwashesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StandardSwashesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStandardSwashes<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStandardSwashes(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardSwashes<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardSwashes(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ContextualSwashesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextualSwashesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextualSwashes<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextualSwashes(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContextualSwashes<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContextualSwashes(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ContextualAlternatesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextualAlternatesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextualAlternates<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextualAlternates(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContextualAlternates<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContextualAlternates(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticAlternatesProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticAlternatesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticAlternates<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticAlternates(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticAlternates<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticAlternates(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet1Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet1<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet1(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet1<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet1(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet2Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet2<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet2(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet2<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet2(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet3Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet3Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet3<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet3(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet3<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet3(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet4Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet4Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet4<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet4(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet4<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet4(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet5Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet5Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet5<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet5(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet5<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet5(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet6Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet6Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet6<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet6(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet6<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet6(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet7Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet7Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet7<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet7(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet7<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet7(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet8Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet8Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet8<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet8(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet8<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet8(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet9Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet9Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet9<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet9(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet9<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet9(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet10Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet10Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet10<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet10(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet10<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet10(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet11Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet11Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet11<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet11(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet11<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet11(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet12Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet12Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet12<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet12(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet12<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet12(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet13Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet13Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet13<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet13(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet13<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet13(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet14Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet14Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet14<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet14(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet14<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet14(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet15Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet15Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet15<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet15(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet15<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet15(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet16Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet16Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet16<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet16(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet16<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet16(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet17Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet17Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet17<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet17(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet17<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet17(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet18Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet18Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet18<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet18(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet18<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet18(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet19Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet19Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet19<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet19(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet19<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet19(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn StylisticSet20Property<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylisticSet20Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylisticSet20<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylisticSet20(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStylisticSet20<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStylisticSet20(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn CapitalsProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapitalsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapitals<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontCapitals) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapitals(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCapitals<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontCapitals) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCapitals(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn CapitalSpacingProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapitalSpacingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapitalSpacing<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapitalSpacing(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCapitalSpacing<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCapitalSpacing(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn KerningProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KerningProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKerning<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKerning(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKerning(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn CaseSensitiveFormsProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaseSensitiveFormsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaseSensitiveForms<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaseSensitiveForms(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseSensitiveForms<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaseSensitiveForms(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn HistoricalFormsProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoricalFormsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHistoricalForms<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHistoricalForms(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoricalForms<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHistoricalForms(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn FractionProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FractionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFraction<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontFraction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFraction(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFraction<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontFraction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFraction(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn NumeralStyleProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumeralStyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumeralStyle<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontNumeralStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumeralStyle(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralStyle<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontNumeralStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumeralStyle(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn NumeralAlignmentProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumeralAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumeralAlignment<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontNumeralAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumeralAlignment(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralAlignment<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontNumeralAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumeralAlignment(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SlashedZeroProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlashedZeroProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSlashedZero<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSlashedZero(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlashedZero<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSlashedZero(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn MathematicalGreekProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MathematicalGreekProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathematicalGreek<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathematicalGreek(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathematicalGreek<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMathematicalGreek(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn VariantsProperty<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VariantsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariants<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontVariants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariants(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVariants<Impl: ITypographyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontVariants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVariants(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITypographyStatics>,
            ::windows::core::GetTrustLevel,
            AnnotationAlternatesProperty::<Impl, OFFSET>,
            GetAnnotationAlternates::<Impl, OFFSET>,
            SetAnnotationAlternates::<Impl, OFFSET>,
            EastAsianExpertFormsProperty::<Impl, OFFSET>,
            GetEastAsianExpertForms::<Impl, OFFSET>,
            SetEastAsianExpertForms::<Impl, OFFSET>,
            EastAsianLanguageProperty::<Impl, OFFSET>,
            GetEastAsianLanguage::<Impl, OFFSET>,
            SetEastAsianLanguage::<Impl, OFFSET>,
            EastAsianWidthsProperty::<Impl, OFFSET>,
            GetEastAsianWidths::<Impl, OFFSET>,
            SetEastAsianWidths::<Impl, OFFSET>,
            StandardLigaturesProperty::<Impl, OFFSET>,
            GetStandardLigatures::<Impl, OFFSET>,
            SetStandardLigatures::<Impl, OFFSET>,
            ContextualLigaturesProperty::<Impl, OFFSET>,
            GetContextualLigatures::<Impl, OFFSET>,
            SetContextualLigatures::<Impl, OFFSET>,
            DiscretionaryLigaturesProperty::<Impl, OFFSET>,
            GetDiscretionaryLigatures::<Impl, OFFSET>,
            SetDiscretionaryLigatures::<Impl, OFFSET>,
            HistoricalLigaturesProperty::<Impl, OFFSET>,
            GetHistoricalLigatures::<Impl, OFFSET>,
            SetHistoricalLigatures::<Impl, OFFSET>,
            StandardSwashesProperty::<Impl, OFFSET>,
            GetStandardSwashes::<Impl, OFFSET>,
            SetStandardSwashes::<Impl, OFFSET>,
            ContextualSwashesProperty::<Impl, OFFSET>,
            GetContextualSwashes::<Impl, OFFSET>,
            SetContextualSwashes::<Impl, OFFSET>,
            ContextualAlternatesProperty::<Impl, OFFSET>,
            GetContextualAlternates::<Impl, OFFSET>,
            SetContextualAlternates::<Impl, OFFSET>,
            StylisticAlternatesProperty::<Impl, OFFSET>,
            GetStylisticAlternates::<Impl, OFFSET>,
            SetStylisticAlternates::<Impl, OFFSET>,
            StylisticSet1Property::<Impl, OFFSET>,
            GetStylisticSet1::<Impl, OFFSET>,
            SetStylisticSet1::<Impl, OFFSET>,
            StylisticSet2Property::<Impl, OFFSET>,
            GetStylisticSet2::<Impl, OFFSET>,
            SetStylisticSet2::<Impl, OFFSET>,
            StylisticSet3Property::<Impl, OFFSET>,
            GetStylisticSet3::<Impl, OFFSET>,
            SetStylisticSet3::<Impl, OFFSET>,
            StylisticSet4Property::<Impl, OFFSET>,
            GetStylisticSet4::<Impl, OFFSET>,
            SetStylisticSet4::<Impl, OFFSET>,
            StylisticSet5Property::<Impl, OFFSET>,
            GetStylisticSet5::<Impl, OFFSET>,
            SetStylisticSet5::<Impl, OFFSET>,
            StylisticSet6Property::<Impl, OFFSET>,
            GetStylisticSet6::<Impl, OFFSET>,
            SetStylisticSet6::<Impl, OFFSET>,
            StylisticSet7Property::<Impl, OFFSET>,
            GetStylisticSet7::<Impl, OFFSET>,
            SetStylisticSet7::<Impl, OFFSET>,
            StylisticSet8Property::<Impl, OFFSET>,
            GetStylisticSet8::<Impl, OFFSET>,
            SetStylisticSet8::<Impl, OFFSET>,
            StylisticSet9Property::<Impl, OFFSET>,
            GetStylisticSet9::<Impl, OFFSET>,
            SetStylisticSet9::<Impl, OFFSET>,
            StylisticSet10Property::<Impl, OFFSET>,
            GetStylisticSet10::<Impl, OFFSET>,
            SetStylisticSet10::<Impl, OFFSET>,
            StylisticSet11Property::<Impl, OFFSET>,
            GetStylisticSet11::<Impl, OFFSET>,
            SetStylisticSet11::<Impl, OFFSET>,
            StylisticSet12Property::<Impl, OFFSET>,
            GetStylisticSet12::<Impl, OFFSET>,
            SetStylisticSet12::<Impl, OFFSET>,
            StylisticSet13Property::<Impl, OFFSET>,
            GetStylisticSet13::<Impl, OFFSET>,
            SetStylisticSet13::<Impl, OFFSET>,
            StylisticSet14Property::<Impl, OFFSET>,
            GetStylisticSet14::<Impl, OFFSET>,
            SetStylisticSet14::<Impl, OFFSET>,
            StylisticSet15Property::<Impl, OFFSET>,
            GetStylisticSet15::<Impl, OFFSET>,
            SetStylisticSet15::<Impl, OFFSET>,
            StylisticSet16Property::<Impl, OFFSET>,
            GetStylisticSet16::<Impl, OFFSET>,
            SetStylisticSet16::<Impl, OFFSET>,
            StylisticSet17Property::<Impl, OFFSET>,
            GetStylisticSet17::<Impl, OFFSET>,
            SetStylisticSet17::<Impl, OFFSET>,
            StylisticSet18Property::<Impl, OFFSET>,
            GetStylisticSet18::<Impl, OFFSET>,
            SetStylisticSet18::<Impl, OFFSET>,
            StylisticSet19Property::<Impl, OFFSET>,
            GetStylisticSet19::<Impl, OFFSET>,
            SetStylisticSet19::<Impl, OFFSET>,
            StylisticSet20Property::<Impl, OFFSET>,
            GetStylisticSet20::<Impl, OFFSET>,
            SetStylisticSet20::<Impl, OFFSET>,
            CapitalsProperty::<Impl, OFFSET>,
            GetCapitals::<Impl, OFFSET>,
            SetCapitals::<Impl, OFFSET>,
            CapitalSpacingProperty::<Impl, OFFSET>,
            GetCapitalSpacing::<Impl, OFFSET>,
            SetCapitalSpacing::<Impl, OFFSET>,
            KerningProperty::<Impl, OFFSET>,
            GetKerning::<Impl, OFFSET>,
            SetKerning::<Impl, OFFSET>,
            CaseSensitiveFormsProperty::<Impl, OFFSET>,
            GetCaseSensitiveForms::<Impl, OFFSET>,
            SetCaseSensitiveForms::<Impl, OFFSET>,
            HistoricalFormsProperty::<Impl, OFFSET>,
            GetHistoricalForms::<Impl, OFFSET>,
            SetHistoricalForms::<Impl, OFFSET>,
            FractionProperty::<Impl, OFFSET>,
            GetFraction::<Impl, OFFSET>,
            SetFraction::<Impl, OFFSET>,
            NumeralStyleProperty::<Impl, OFFSET>,
            GetNumeralStyle::<Impl, OFFSET>,
            SetNumeralStyle::<Impl, OFFSET>,
            NumeralAlignmentProperty::<Impl, OFFSET>,
            GetNumeralAlignment::<Impl, OFFSET>,
            SetNumeralAlignment::<Impl, OFFSET>,
            SlashedZeroProperty::<Impl, OFFSET>,
            GetSlashedZero::<Impl, OFFSET>,
            SetSlashedZero::<Impl, OFFSET>,
            MathematicalGreekProperty::<Impl, OFFSET>,
            GetMathematicalGreek::<Impl, OFFSET>,
            SetMathematicalGreek::<Impl, OFFSET>,
            VariantsProperty::<Impl, OFFSET>,
            GetVariants::<Impl, OFFSET>,
            SetVariants::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnderlineImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnderline {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.IUnderline";
}
#[cfg(feature = "implement_exclusive")]
impl IUnderlineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnderlineImpl, const OFFSET: isize>() -> IUnderlineVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUnderline>, ::windows::core::GetTrustLevel)
    }
}
