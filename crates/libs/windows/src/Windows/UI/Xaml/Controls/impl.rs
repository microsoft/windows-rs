pub trait IAppBarOverrides_Impl: Sized {
    fn OnClosed(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnOpened(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppBarOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IAppBarOverrides";
}
impl IAppBarOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarOverrides_Vtbl {
        unsafe extern "system" fn OnClosed<Impl: IAppBarOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClosed(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnOpened<Impl: IAppBarOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOpened(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarOverrides, BASE_OFFSET>(),
            OnClosed: OnClosed::<Impl, IMPL_OFFSET>,
            OnOpened: OnOpened::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IAppBarOverrides3_Impl: Sized {
    fn OnClosing(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnOpening(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppBarOverrides3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IAppBarOverrides3";
}
impl IAppBarOverrides3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarOverrides3_Vtbl {
        unsafe extern "system" fn OnClosing<Impl: IAppBarOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClosing(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnOpening<Impl: IAppBarOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOpening(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarOverrides3, BASE_OFFSET>(),
            OnClosing: OnClosing::<Impl, IMPL_OFFSET>,
            OnOpening: OnOpening::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarOverrides3 as ::windows::core::Interface>::IID
    }
}
pub trait IComboBoxOverrides_Impl: Sized {
    fn OnDropDownClosed(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnDropDownOpened(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComboBoxOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IComboBoxOverrides";
}
impl IComboBoxOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxOverrides_Vtbl {
        unsafe extern "system" fn OnDropDownClosed<Impl: IComboBoxOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDropDownClosed(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDropDownOpened<Impl: IComboBoxOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDropDownOpened(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxOverrides, BASE_OFFSET>(),
            OnDropDownClosed: OnDropDownClosed::<Impl, IMPL_OFFSET>,
            OnDropDownOpened: OnDropDownOpened::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxOverrides as ::windows::core::Interface>::IID
    }
}
pub trait ICommandBarElement_Impl: Sized {
    fn IsCompact(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCompact(&mut self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandBarElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ICommandBarElement";
}
impl ICommandBarElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarElement_Vtbl {
        unsafe extern "system" fn IsCompact<Impl: ICommandBarElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCompact<Impl: ICommandBarElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCompact(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarElement, BASE_OFFSET>(),
            IsCompact: IsCompact::<Impl, IMPL_OFFSET>,
            SetIsCompact: SetIsCompact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarElement as ::windows::core::Interface>::IID
    }
}
pub trait ICommandBarElement2_Impl: Sized {
    fn IsInOverflow(&mut self) -> ::windows::core::Result<bool>;
    fn DynamicOverflowOrder(&mut self) -> ::windows::core::Result<i32>;
    fn SetDynamicOverflowOrder(&mut self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandBarElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ICommandBarElement2";
}
impl ICommandBarElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarElement2_Vtbl {
        unsafe extern "system" fn IsInOverflow<Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInOverflow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicOverflowOrder<Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicOverflowOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicOverflowOrder<Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDynamicOverflowOrder(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarElement2, BASE_OFFSET>(),
            IsInOverflow: IsInOverflow::<Impl, IMPL_OFFSET>,
            DynamicOverflowOrder: DynamicOverflowOrder::<Impl, IMPL_OFFSET>,
            SetDynamicOverflowOrder: SetDynamicOverflowOrder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarElement2 as ::windows::core::Interface>::IID
    }
}
pub trait IContentControlOverrides_Impl: Sized {
    fn OnContentChanged(&mut self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnContentTemplateChanged(&mut self, oldcontenttemplate: &::core::option::Option<super::DataTemplate>, newcontenttemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnContentTemplateSelectorChanged(&mut self, oldcontenttemplateselector: &::core::option::Option<DataTemplateSelector>, newcontenttemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContentControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControlOverrides";
}
impl IContentControlOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentControlOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentControlOverrides_Vtbl {
        unsafe extern "system" fn OnContentChanged<Impl: IContentControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentChanged(&*(&oldcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&newcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnContentTemplateChanged<Impl: IContentControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplate: ::windows::core::RawPtr, newcontenttemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentTemplateChanged(&*(&oldcontenttemplate as *const <super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::DataTemplate as ::windows::core::DefaultType>::DefaultType), &*(&newcontenttemplate as *const <super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnContentTemplateSelectorChanged<Impl: IContentControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplateselector: ::windows::core::RawPtr, newcontenttemplateselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentTemplateSelectorChanged(&*(&oldcontenttemplateselector as *const <DataTemplateSelector as ::windows::core::Abi>::Abi as *const <DataTemplateSelector as ::windows::core::DefaultType>::DefaultType), &*(&newcontenttemplateselector as *const <DataTemplateSelector as ::windows::core::Abi>::Abi as *const <DataTemplateSelector as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentControlOverrides, BASE_OFFSET>(),
            OnContentChanged: OnContentChanged::<Impl, IMPL_OFFSET>,
            OnContentTemplateChanged: OnContentTemplateChanged::<Impl, IMPL_OFFSET>,
            OnContentTemplateSelectorChanged: OnContentTemplateSelectorChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentControlOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IContentPresenterOverrides_Impl: Sized {
    fn OnContentTemplateChanged(&mut self, oldcontenttemplate: &::core::option::Option<super::DataTemplate>, newcontenttemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnContentTemplateSelectorChanged(&mut self, oldcontenttemplateselector: &::core::option::Option<DataTemplateSelector>, newcontenttemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContentPresenterOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentPresenterOverrides";
}
impl IContentPresenterOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPresenterOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPresenterOverrides_Vtbl {
        unsafe extern "system" fn OnContentTemplateChanged<Impl: IContentPresenterOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplate: ::windows::core::RawPtr, newcontenttemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentTemplateChanged(&*(&oldcontenttemplate as *const <super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::DataTemplate as ::windows::core::DefaultType>::DefaultType), &*(&newcontenttemplate as *const <super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnContentTemplateSelectorChanged<Impl: IContentPresenterOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplateselector: ::windows::core::RawPtr, newcontenttemplateselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentTemplateSelectorChanged(&*(&oldcontenttemplateselector as *const <DataTemplateSelector as ::windows::core::Abi>::Abi as *const <DataTemplateSelector as ::windows::core::DefaultType>::DefaultType), &*(&newcontenttemplateselector as *const <DataTemplateSelector as ::windows::core::Abi>::Abi as *const <DataTemplateSelector as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPresenterOverrides, BASE_OFFSET>(),
            OnContentTemplateChanged: OnContentTemplateChanged::<Impl, IMPL_OFFSET>,
            OnContentTemplateSelectorChanged: OnContentTemplateSelectorChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPresenterOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IControlOverrides_Impl: Sized {
    fn OnPointerEntered(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerPressed(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerMoved(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerReleased(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerExited(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerCaptureLost(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerCanceled(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerWheelChanged(&mut self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnTapped(&mut self, e: &::core::option::Option<super::Input::TappedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnDoubleTapped(&mut self, e: &::core::option::Option<super::Input::DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnHolding(&mut self, e: &::core::option::Option<super::Input::HoldingRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnRightTapped(&mut self, e: &::core::option::Option<super::Input::RightTappedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationStarting(&mut self, e: &::core::option::Option<super::Input::ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationInertiaStarting(&mut self, e: &::core::option::Option<super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationStarted(&mut self, e: &::core::option::Option<super::Input::ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationDelta(&mut self, e: &::core::option::Option<super::Input::ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationCompleted(&mut self, e: &::core::option::Option<super::Input::ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnKeyUp(&mut self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnKeyDown(&mut self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnGotFocus(&mut self, e: &::core::option::Option<super::RoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnLostFocus(&mut self, e: &::core::option::Option<super::RoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnDragEnter(&mut self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
    fn OnDragLeave(&mut self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
    fn OnDragOver(&mut self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
    fn OnDrop(&mut self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlOverrides";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IControlOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlOverrides_Vtbl {
        unsafe extern "system" fn OnPointerEntered<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerEntered(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerPressed<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerPressed(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerMoved<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerMoved(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerReleased<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerReleased(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerExited<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerExited(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerCaptureLost<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerCaptureLost(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerCanceled<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerCanceled(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPointerWheelChanged<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPointerWheelChanged(&*(&e as *const <super::Input::PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnTapped<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTapped(&*(&e as *const <super::Input::TappedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::TappedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDoubleTapped<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDoubleTapped(&*(&e as *const <super::Input::DoubleTappedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::DoubleTappedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnHolding<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnHolding(&*(&e as *const <super::Input::HoldingRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::HoldingRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnRightTapped<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRightTapped(&*(&e as *const <super::Input::RightTappedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::RightTappedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnManipulationStarting<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManipulationStarting(&*(&e as *const <super::Input::ManipulationStartingRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::ManipulationStartingRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnManipulationInertiaStarting<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManipulationInertiaStarting(&*(&e as *const <super::Input::ManipulationInertiaStartingRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::ManipulationInertiaStartingRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnManipulationStarted<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManipulationStarted(&*(&e as *const <super::Input::ManipulationStartedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::ManipulationStartedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnManipulationDelta<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManipulationDelta(&*(&e as *const <super::Input::ManipulationDeltaRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::ManipulationDeltaRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnManipulationCompleted<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManipulationCompleted(&*(&e as *const <super::Input::ManipulationCompletedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::ManipulationCompletedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnKeyUp<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyUp(&*(&e as *const <super::Input::KeyRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::KeyRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnKeyDown<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyDown(&*(&e as *const <super::Input::KeyRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::KeyRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnGotFocus<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGotFocus(&*(&e as *const <super::RoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::RoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLostFocus<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLostFocus(&*(&e as *const <super::RoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::RoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDragEnter<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDragEnter(&*(&e as *const <super::DragEventArgs as ::windows::core::Abi>::Abi as *const <super::DragEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDragLeave<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDragLeave(&*(&e as *const <super::DragEventArgs as ::windows::core::Abi>::Abi as *const <super::DragEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDragOver<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDragOver(&*(&e as *const <super::DragEventArgs as ::windows::core::Abi>::Abi as *const <super::DragEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDrop<Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDrop(&*(&e as *const <super::DragEventArgs as ::windows::core::Abi>::Abi as *const <super::DragEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlOverrides, BASE_OFFSET>(),
            OnPointerEntered: OnPointerEntered::<Impl, IMPL_OFFSET>,
            OnPointerPressed: OnPointerPressed::<Impl, IMPL_OFFSET>,
            OnPointerMoved: OnPointerMoved::<Impl, IMPL_OFFSET>,
            OnPointerReleased: OnPointerReleased::<Impl, IMPL_OFFSET>,
            OnPointerExited: OnPointerExited::<Impl, IMPL_OFFSET>,
            OnPointerCaptureLost: OnPointerCaptureLost::<Impl, IMPL_OFFSET>,
            OnPointerCanceled: OnPointerCanceled::<Impl, IMPL_OFFSET>,
            OnPointerWheelChanged: OnPointerWheelChanged::<Impl, IMPL_OFFSET>,
            OnTapped: OnTapped::<Impl, IMPL_OFFSET>,
            OnDoubleTapped: OnDoubleTapped::<Impl, IMPL_OFFSET>,
            OnHolding: OnHolding::<Impl, IMPL_OFFSET>,
            OnRightTapped: OnRightTapped::<Impl, IMPL_OFFSET>,
            OnManipulationStarting: OnManipulationStarting::<Impl, IMPL_OFFSET>,
            OnManipulationInertiaStarting: OnManipulationInertiaStarting::<Impl, IMPL_OFFSET>,
            OnManipulationStarted: OnManipulationStarted::<Impl, IMPL_OFFSET>,
            OnManipulationDelta: OnManipulationDelta::<Impl, IMPL_OFFSET>,
            OnManipulationCompleted: OnManipulationCompleted::<Impl, IMPL_OFFSET>,
            OnKeyUp: OnKeyUp::<Impl, IMPL_OFFSET>,
            OnKeyDown: OnKeyDown::<Impl, IMPL_OFFSET>,
            OnGotFocus: OnGotFocus::<Impl, IMPL_OFFSET>,
            OnLostFocus: OnLostFocus::<Impl, IMPL_OFFSET>,
            OnDragEnter: OnDragEnter::<Impl, IMPL_OFFSET>,
            OnDragLeave: OnDragLeave::<Impl, IMPL_OFFSET>,
            OnDragOver: OnDragOver::<Impl, IMPL_OFFSET>,
            OnDrop: OnDrop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IControlOverrides6_Impl: Sized {
    fn OnPreviewKeyDown(&mut self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPreviewKeyUp(&mut self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnCharacterReceived(&mut self, e: &::core::option::Option<super::Input::CharacterReceivedRoutedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IControlOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlOverrides6";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IControlOverrides6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlOverrides6_Vtbl {
        unsafe extern "system" fn OnPreviewKeyDown<Impl: IControlOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPreviewKeyDown(&*(&e as *const <super::Input::KeyRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::KeyRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnPreviewKeyUp<Impl: IControlOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPreviewKeyUp(&*(&e as *const <super::Input::KeyRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::KeyRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnCharacterReceived<Impl: IControlOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCharacterReceived(&*(&e as *const <super::Input::CharacterReceivedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <super::Input::CharacterReceivedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlOverrides6, BASE_OFFSET>(),
            OnPreviewKeyDown: OnPreviewKeyDown::<Impl, IMPL_OFFSET>,
            OnPreviewKeyUp: OnPreviewKeyUp::<Impl, IMPL_OFFSET>,
            OnCharacterReceived: OnCharacterReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlOverrides6 as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateSelectorOverrides_Impl: Sized {
    fn SelectTemplateCore(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DataTemplate>;
}
impl ::windows::core::RuntimeName for IDataTemplateSelectorOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IDataTemplateSelectorOverrides";
}
impl IDataTemplateSelectorOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateSelectorOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectTemplateCore<Impl: IDataTemplateSelectorOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectTemplateCore(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&container as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateSelectorOverrides, BASE_OFFSET>(),
            SelectTemplateCore: SelectTemplateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateSelectorOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateSelectorOverrides2_Impl: Sized {
    fn SelectTemplateForItemCore(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DataTemplate>;
}
impl ::windows::core::RuntimeName for IDataTemplateSelectorOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IDataTemplateSelectorOverrides2";
}
impl IDataTemplateSelectorOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateSelectorOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateSelectorOverrides2_Vtbl {
        unsafe extern "system" fn SelectTemplateForItemCore<Impl: IDataTemplateSelectorOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectTemplateForItemCore(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateSelectorOverrides2, BASE_OFFSET>(),
            SelectTemplateForItemCore: SelectTemplateForItemCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateSelectorOverrides2 as ::windows::core::Interface>::IID
    }
}
pub trait IGroupStyleSelectorOverrides_Impl: Sized {
    fn SelectGroupStyleCore(&mut self, group: &::core::option::Option<::windows::core::IInspectable>, level: u32) -> ::windows::core::Result<GroupStyle>;
}
impl ::windows::core::RuntimeName for IGroupStyleSelectorOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IGroupStyleSelectorOverrides";
}
impl IGroupStyleSelectorOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupStyleSelectorOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupStyleSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectGroupStyleCore<Impl: IGroupStyleSelectorOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void, level: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectGroupStyleCore(&*(&group as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGroupStyleSelectorOverrides, BASE_OFFSET>(),
            SelectGroupStyleCore: SelectGroupStyleCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupStyleSelectorOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
pub trait IInkToolbarCustomPenOverrides_Impl: Sized {
    fn CreateInkDrawingAttributesCore(&mut self, brush: &::core::option::Option<super::Media::Brush>, strokewidth: f64) -> ::windows::core::Result<super::super::Input::Inking::InkDrawingAttributes>;
}
#[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
impl ::windows::core::RuntimeName for IInkToolbarCustomPenOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IInkToolbarCustomPenOverrides";
}
#[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
impl IInkToolbarCustomPenOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkToolbarCustomPenOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkToolbarCustomPenOverrides_Vtbl {
        unsafe extern "system" fn CreateInkDrawingAttributesCore<Impl: IInkToolbarCustomPenOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr, strokewidth: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInkDrawingAttributesCore(&*(&brush as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType), strokewidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkToolbarCustomPenOverrides, BASE_OFFSET>(),
            CreateInkDrawingAttributesCore: CreateInkDrawingAttributesCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkToolbarCustomPenOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IInsertionPanel_Impl: Sized {
    fn GetInsertionIndexes(&mut self, position: &super::super::super::Foundation::Point, first: &mut i32, second: &mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInsertionPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IInsertionPanel";
}
#[cfg(feature = "Foundation")]
impl IInsertionPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInsertionPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInsertionPanel_Vtbl {
        unsafe extern "system" fn GetInsertionIndexes<Impl: IInsertionPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, first: *mut i32, second: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInsertionIndexes(&*(&position as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInsertionPanel, BASE_OFFSET>(),
            GetInsertionIndexes: GetInsertionIndexes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInsertionPanel as ::windows::core::Interface>::IID
    }
}
pub trait IItemContainerMapping_Impl: Sized {
    fn ItemFromContainer(&mut self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn IndexFromContainer(&mut self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn ContainerFromIndex(&mut self, index: i32) -> ::windows::core::Result<super::DependencyObject>;
}
impl ::windows::core::RuntimeName for IItemContainerMapping {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IItemContainerMapping";
}
impl IItemContainerMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemContainerMapping_Vtbl {
        unsafe extern "system" fn ItemFromContainer<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemFromContainer(&*(&container as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromItem<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerFromItem(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexFromContainer<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexFromContainer(&*(&container as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromIndex<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerFromIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemContainerMapping, BASE_OFFSET>(),
            ItemFromContainer: ItemFromContainer::<Impl, IMPL_OFFSET>,
            ContainerFromItem: ContainerFromItem::<Impl, IMPL_OFFSET>,
            IndexFromContainer: IndexFromContainer::<Impl, IMPL_OFFSET>,
            ContainerFromIndex: ContainerFromIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerMapping as ::windows::core::Interface>::IID
    }
}
pub trait IItemsControlOverrides_Impl: Sized {
    fn IsItemItsOwnContainerOverride(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn GetContainerForItemOverride(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn ClearContainerForItemOverride(&mut self, element: &::core::option::Option<super::DependencyObject>, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PrepareContainerForItemOverride(&mut self, element: &::core::option::Option<super::DependencyObject>, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnItemsChanged(&mut self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnItemContainerStyleChanged(&mut self, olditemcontainerstyle: &::core::option::Option<super::Style>, newitemcontainerstyle: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn OnItemContainerStyleSelectorChanged(&mut self, olditemcontainerstyleselector: &::core::option::Option<StyleSelector>, newitemcontainerstyleselector: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()>;
    fn OnItemTemplateChanged(&mut self, olditemtemplate: &::core::option::Option<super::DataTemplate>, newitemtemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnItemTemplateSelectorChanged(&mut self, olditemtemplateselector: &::core::option::Option<DataTemplateSelector>, newitemtemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn OnGroupStyleSelectorChanged(&mut self, oldgroupstyleselector: &::core::option::Option<GroupStyleSelector>, newgroupstyleselector: &::core::option::Option<GroupStyleSelector>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IItemsControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IItemsControlOverrides";
}
impl IItemsControlOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlOverrides_Vtbl {
        unsafe extern "system" fn IsItemItsOwnContainerOverride<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsItemItsOwnContainerOverride(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerForItemOverride<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerForItemOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearContainerForItemOverride<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearContainerForItemOverride(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrepareContainerForItemOverride<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareContainerForItemOverride(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemsChanged(&*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnItemContainerStyleChanged<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemcontainerstyle: ::windows::core::RawPtr, newitemcontainerstyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemContainerStyleChanged(&*(&olditemcontainerstyle as *const <super::Style as ::windows::core::Abi>::Abi as *const <super::Style as ::windows::core::DefaultType>::DefaultType), &*(&newitemcontainerstyle as *const <super::Style as ::windows::core::Abi>::Abi as *const <super::Style as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnItemContainerStyleSelectorChanged<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemcontainerstyleselector: ::windows::core::RawPtr, newitemcontainerstyleselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemContainerStyleSelectorChanged(&*(&olditemcontainerstyleselector as *const <StyleSelector as ::windows::core::Abi>::Abi as *const <StyleSelector as ::windows::core::DefaultType>::DefaultType), &*(&newitemcontainerstyleselector as *const <StyleSelector as ::windows::core::Abi>::Abi as *const <StyleSelector as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnItemTemplateChanged<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemtemplate: ::windows::core::RawPtr, newitemtemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemTemplateChanged(&*(&olditemtemplate as *const <super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::DataTemplate as ::windows::core::DefaultType>::DefaultType), &*(&newitemtemplate as *const <super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnItemTemplateSelectorChanged<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemtemplateselector: ::windows::core::RawPtr, newitemtemplateselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemTemplateSelectorChanged(&*(&olditemtemplateselector as *const <DataTemplateSelector as ::windows::core::Abi>::Abi as *const <DataTemplateSelector as ::windows::core::DefaultType>::DefaultType), &*(&newitemtemplateselector as *const <DataTemplateSelector as ::windows::core::Abi>::Abi as *const <DataTemplateSelector as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnGroupStyleSelectorChanged<Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldgroupstyleselector: ::windows::core::RawPtr, newgroupstyleselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGroupStyleSelectorChanged(&*(&oldgroupstyleselector as *const <GroupStyleSelector as ::windows::core::Abi>::Abi as *const <GroupStyleSelector as ::windows::core::DefaultType>::DefaultType), &*(&newgroupstyleselector as *const <GroupStyleSelector as ::windows::core::Abi>::Abi as *const <GroupStyleSelector as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlOverrides, BASE_OFFSET>(),
            IsItemItsOwnContainerOverride: IsItemItsOwnContainerOverride::<Impl, IMPL_OFFSET>,
            GetContainerForItemOverride: GetContainerForItemOverride::<Impl, IMPL_OFFSET>,
            ClearContainerForItemOverride: ClearContainerForItemOverride::<Impl, IMPL_OFFSET>,
            PrepareContainerForItemOverride: PrepareContainerForItemOverride::<Impl, IMPL_OFFSET>,
            OnItemsChanged: OnItemsChanged::<Impl, IMPL_OFFSET>,
            OnItemContainerStyleChanged: OnItemContainerStyleChanged::<Impl, IMPL_OFFSET>,
            OnItemContainerStyleSelectorChanged: OnItemContainerStyleSelectorChanged::<Impl, IMPL_OFFSET>,
            OnItemTemplateChanged: OnItemTemplateChanged::<Impl, IMPL_OFFSET>,
            OnItemTemplateSelectorChanged: OnItemTemplateSelectorChanged::<Impl, IMPL_OFFSET>,
            OnGroupStyleSelectorChanged: OnGroupStyleSelectorChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait INavigate_Impl: Sized {
    fn Navigate(&mut self, sourcepagetype: &super::Interop::TypeName) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for INavigate {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.INavigate";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl INavigate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigate_Vtbl {
        unsafe extern "system" fn Navigate<Impl: INavigate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(&*(&sourcepagetype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INavigate, BASE_OFFSET>(), Navigate: Navigate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Navigation")]
pub trait IPageOverrides_Impl: Sized {
    fn OnNavigatedFrom(&mut self, e: &::core::option::Option<super::Navigation::NavigationEventArgs>) -> ::windows::core::Result<()>;
    fn OnNavigatedTo(&mut self, e: &::core::option::Option<super::Navigation::NavigationEventArgs>) -> ::windows::core::Result<()>;
    fn OnNavigatingFrom(&mut self, e: &::core::option::Option<super::Navigation::NavigatingCancelEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Navigation")]
impl ::windows::core::RuntimeName for IPageOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IPageOverrides";
}
#[cfg(feature = "UI_Xaml_Navigation")]
impl IPageOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPageOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPageOverrides_Vtbl {
        unsafe extern "system" fn OnNavigatedFrom<Impl: IPageOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNavigatedFrom(&*(&e as *const <super::Navigation::NavigationEventArgs as ::windows::core::Abi>::Abi as *const <super::Navigation::NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnNavigatedTo<Impl: IPageOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNavigatedTo(&*(&e as *const <super::Navigation::NavigationEventArgs as ::windows::core::Abi>::Abi as *const <super::Navigation::NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnNavigatingFrom<Impl: IPageOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNavigatingFrom(&*(&e as *const <super::Navigation::NavigatingCancelEventArgs as ::windows::core::Abi>::Abi as *const <super::Navigation::NavigatingCancelEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPageOverrides, BASE_OFFSET>(),
            OnNavigatedFrom: OnNavigatedFrom::<Impl, IMPL_OFFSET>,
            OnNavigatedTo: OnNavigatedTo::<Impl, IMPL_OFFSET>,
            OnNavigatingFrom: OnNavigatingFrom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPageOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IScrollAnchorProvider_Impl: Sized {
    fn CurrentAnchor(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn RegisterAnchorCandidate(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn UnregisterAnchorCandidate(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollAnchorProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IScrollAnchorProvider";
}
impl IScrollAnchorProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollAnchorProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollAnchorProvider_Vtbl {
        unsafe extern "system" fn CurrentAnchor<Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAnchorCandidate<Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterAnchorCandidate(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterAnchorCandidate<Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAnchorCandidate(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollAnchorProvider, BASE_OFFSET>(),
            CurrentAnchor: CurrentAnchor::<Impl, IMPL_OFFSET>,
            RegisterAnchorCandidate: RegisterAnchorCandidate::<Impl, IMPL_OFFSET>,
            UnregisterAnchorCandidate: UnregisterAnchorCandidate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollAnchorProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISemanticZoomInformation_Impl: Sized {
    fn SemanticZoomOwner(&mut self) -> ::windows::core::Result<SemanticZoom>;
    fn SetSemanticZoomOwner(&mut self, value: &::core::option::Option<SemanticZoom>) -> ::windows::core::Result<()>;
    fn IsActiveView(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsActiveView(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomedInView(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsZoomedInView(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InitializeViewChange(&mut self) -> ::windows::core::Result<()>;
    fn CompleteViewChange(&mut self) -> ::windows::core::Result<()>;
    fn MakeVisible(&mut self, item: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeFrom(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeTo(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeFrom(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeTo(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISemanticZoomInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ISemanticZoomInformation";
}
impl ISemanticZoomInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticZoomInformation_Vtbl {
        unsafe extern "system" fn SemanticZoomOwner<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SemanticZoomOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSemanticZoomOwner<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSemanticZoomOwner(&*(&value as *const <SemanticZoom as ::windows::core::Abi>::Abi as *const <SemanticZoom as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsActiveView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActiveView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsActiveView(value).into()
        }
        unsafe extern "system" fn IsZoomedInView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsZoomedInView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZoomedInView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsZoomedInView(value).into()
        }
        unsafe extern "system" fn InitializeViewChange<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeViewChange().into()
        }
        unsafe extern "system" fn CompleteViewChange<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteViewChange().into()
        }
        unsafe extern "system" fn MakeVisible<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeVisible(&*(&item as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartViewChangeFrom<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartViewChangeFrom(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartViewChangeTo<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartViewChangeTo(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompleteViewChangeFrom<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteViewChangeFrom(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompleteViewChangeTo<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteViewChangeTo(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticZoomInformation, BASE_OFFSET>(),
            SemanticZoomOwner: SemanticZoomOwner::<Impl, IMPL_OFFSET>,
            SetSemanticZoomOwner: SetSemanticZoomOwner::<Impl, IMPL_OFFSET>,
            IsActiveView: IsActiveView::<Impl, IMPL_OFFSET>,
            SetIsActiveView: SetIsActiveView::<Impl, IMPL_OFFSET>,
            IsZoomedInView: IsZoomedInView::<Impl, IMPL_OFFSET>,
            SetIsZoomedInView: SetIsZoomedInView::<Impl, IMPL_OFFSET>,
            InitializeViewChange: InitializeViewChange::<Impl, IMPL_OFFSET>,
            CompleteViewChange: CompleteViewChange::<Impl, IMPL_OFFSET>,
            MakeVisible: MakeVisible::<Impl, IMPL_OFFSET>,
            StartViewChangeFrom: StartViewChangeFrom::<Impl, IMPL_OFFSET>,
            StartViewChangeTo: StartViewChangeTo::<Impl, IMPL_OFFSET>,
            CompleteViewChangeFrom: CompleteViewChangeFrom::<Impl, IMPL_OFFSET>,
            CompleteViewChangeTo: CompleteViewChangeTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticZoomInformation as ::windows::core::Interface>::IID
    }
}
pub trait IStyleSelectorOverrides_Impl: Sized {
    fn SelectStyleCore(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::Style>;
}
impl ::windows::core::RuntimeName for IStyleSelectorOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IStyleSelectorOverrides";
}
impl IStyleSelectorOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStyleSelectorOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStyleSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectStyleCore<Impl: IStyleSelectorOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectStyleCore(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&container as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStyleSelectorOverrides, BASE_OFFSET>(),
            SelectStyleCore: SelectStyleCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStyleSelectorOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IToggleSwitchOverrides_Impl: Sized {
    fn OnToggled(&mut self) -> ::windows::core::Result<()>;
    fn OnOnContentChanged(&mut self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnOffContentChanged(&mut self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnHeaderChanged(&mut self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IToggleSwitchOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IToggleSwitchOverrides";
}
impl IToggleSwitchOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchOverrides_Vtbl {
        unsafe extern "system" fn OnToggled<Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnToggled().into()
        }
        unsafe extern "system" fn OnOnContentChanged<Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOnContentChanged(&*(&oldcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&newcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnOffContentChanged<Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOffContentChanged(&*(&oldcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&newcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnHeaderChanged<Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnHeaderChanged(&*(&oldcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&newcontent as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleSwitchOverrides, BASE_OFFSET>(),
            OnToggled: OnToggled::<Impl, IMPL_OFFSET>,
            OnOnContentChanged: OnOnContentChanged::<Impl, IMPL_OFFSET>,
            OnOffContentChanged: OnOffContentChanged::<Impl, IMPL_OFFSET>,
            OnHeaderChanged: OnHeaderChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleSwitchOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
pub trait IVirtualizingPanelOverrides_Impl: Sized {
    fn OnItemsChanged(&mut self, sender: &::core::option::Option<::windows::core::IInspectable>, args: &::core::option::Option<Primitives::ItemsChangedEventArgs>) -> ::windows::core::Result<()>;
    fn OnClearChildren(&mut self) -> ::windows::core::Result<()>;
    fn BringIndexIntoView(&mut self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
impl ::windows::core::RuntimeName for IVirtualizingPanelOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IVirtualizingPanelOverrides";
}
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
impl IVirtualizingPanelOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingPanelOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualizingPanelOverrides_Vtbl {
        unsafe extern "system" fn OnItemsChanged<Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemsChanged(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <Primitives::ItemsChangedEventArgs as ::windows::core::Abi>::Abi as *const <Primitives::ItemsChangedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnClearChildren<Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClearChildren().into()
        }
        unsafe extern "system" fn BringIndexIntoView<Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BringIndexIntoView(index).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualizingPanelOverrides, BASE_OFFSET>(),
            OnItemsChanged: OnItemsChanged::<Impl, IMPL_OFFSET>,
            OnClearChildren: OnClearChildren::<Impl, IMPL_OFFSET>,
            BringIndexIntoView: BringIndexIntoView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizingPanelOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualizingStackPanelOverrides_Impl: Sized {
    fn OnCleanUpVirtualizedItem(&mut self, e: &::core::option::Option<CleanUpVirtualizedItemEventArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVirtualizingStackPanelOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IVirtualizingStackPanelOverrides";
}
impl IVirtualizingStackPanelOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingStackPanelOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualizingStackPanelOverrides_Vtbl {
        unsafe extern "system" fn OnCleanUpVirtualizedItem<Impl: IVirtualizingStackPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCleanUpVirtualizedItem(&*(&e as *const <CleanUpVirtualizedItemEventArgs as ::windows::core::Abi>::Abi as *const <CleanUpVirtualizedItemEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualizingStackPanelOverrides, BASE_OFFSET>(),
            OnCleanUpVirtualizedItem: OnCleanUpVirtualizedItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizingStackPanelOverrides as ::windows::core::Interface>::IID
    }
}
