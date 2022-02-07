pub trait IAppBarOverrides_Impl: Sized {
    fn OnClosed(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnOpened(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IAppBarOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IAppBarOverrides";
}
impl IAppBarOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides_Impl, const OFFSET: isize>() -> IAppBarOverrides_Vtbl {
        unsafe extern "system" fn OnClosed<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnClosed(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnOpened<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOpened(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarOverrides, OFFSET>(),
            OnClosed: OnClosed::<Identity, Impl, OFFSET>,
            OnOpened: OnOpened::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IAppBarOverrides3_Impl: Sized {
    fn OnClosing(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnOpening(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IAppBarOverrides3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IAppBarOverrides3";
}
impl IAppBarOverrides3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides3_Impl, const OFFSET: isize>() -> IAppBarOverrides3_Vtbl {
        unsafe extern "system" fn OnClosing<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnClosing(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnOpening<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOpening(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarOverrides3, OFFSET>(),
            OnClosing: OnClosing::<Identity, Impl, OFFSET>,
            OnOpening: OnOpening::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarOverrides3 as ::windows::core::Interface>::IID
    }
}
pub trait IComboBoxOverrides_Impl: Sized {
    fn OnDropDownClosed(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDropDownOpened(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IComboBoxOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IComboBoxOverrides";
}
impl IComboBoxOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxOverrides_Impl, const OFFSET: isize>() -> IComboBoxOverrides_Vtbl {
        unsafe extern "system" fn OnDropDownClosed<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDropDownClosed(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnDropDownOpened<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDropDownOpened(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxOverrides, OFFSET>(),
            OnDropDownClosed: OnDropDownClosed::<Identity, Impl, OFFSET>,
            OnDropDownOpened: OnDropDownOpened::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxOverrides as ::windows::core::Interface>::IID
    }
}
pub trait ICommandBarElement_Impl: Sized {
    fn IsCompact(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompact(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandBarElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ICommandBarElement";
}
impl ICommandBarElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement_Impl, const OFFSET: isize>() -> ICommandBarElement_Vtbl {
        unsafe extern "system" fn IsCompact<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCompact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCompact<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsCompact(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarElement, OFFSET>(),
            IsCompact: IsCompact::<Identity, Impl, OFFSET>,
            SetIsCompact: SetIsCompact::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarElement as ::windows::core::Interface>::IID
    }
}
pub trait ICommandBarElement2_Impl: Sized {
    fn IsInOverflow(&self) -> ::windows::core::Result<bool>;
    fn DynamicOverflowOrder(&self) -> ::windows::core::Result<i32>;
    fn SetDynamicOverflowOrder(&self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandBarElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ICommandBarElement2";
}
impl ICommandBarElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement2_Impl, const OFFSET: isize>() -> ICommandBarElement2_Vtbl {
        unsafe extern "system" fn IsInOverflow<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInOverflow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicOverflowOrder<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DynamicOverflowOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicOverflowOrder<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDynamicOverflowOrder(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarElement2, OFFSET>(),
            IsInOverflow: IsInOverflow::<Identity, Impl, OFFSET>,
            DynamicOverflowOrder: DynamicOverflowOrder::<Identity, Impl, OFFSET>,
            SetDynamicOverflowOrder: SetDynamicOverflowOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarElement2 as ::windows::core::Interface>::IID
    }
}
pub trait IContentControlOverrides_Impl: Sized {
    fn OnContentChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnContentTemplateChanged(&self, oldcontenttemplate: &::core::option::Option<super::DataTemplate>, newcontenttemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnContentTemplateSelectorChanged(&self, oldcontenttemplateselector: &::core::option::Option<DataTemplateSelector>, newcontenttemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IContentControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentControlOverrides";
}
impl IContentControlOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentControlOverrides_Impl, const OFFSET: isize>() -> IContentControlOverrides_Vtbl {
        unsafe extern "system" fn OnContentChanged<Identity: ::windows::core::IUnknownImpl, Impl: IContentControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentChanged(::core::mem::transmute(&oldcontent), ::core::mem::transmute(&newcontent)).into()
        }
        unsafe extern "system" fn OnContentTemplateChanged<Identity: ::windows::core::IUnknownImpl, Impl: IContentControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplate: ::windows::core::RawPtr, newcontenttemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentTemplateChanged(::core::mem::transmute(&oldcontenttemplate), ::core::mem::transmute(&newcontenttemplate)).into()
        }
        unsafe extern "system" fn OnContentTemplateSelectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IContentControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplateselector: ::windows::core::RawPtr, newcontenttemplateselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentTemplateSelectorChanged(::core::mem::transmute(&oldcontenttemplateselector), ::core::mem::transmute(&newcontenttemplateselector)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentControlOverrides, OFFSET>(),
            OnContentChanged: OnContentChanged::<Identity, Impl, OFFSET>,
            OnContentTemplateChanged: OnContentTemplateChanged::<Identity, Impl, OFFSET>,
            OnContentTemplateSelectorChanged: OnContentTemplateSelectorChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentControlOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IContentPresenterOverrides_Impl: Sized {
    fn OnContentTemplateChanged(&self, oldcontenttemplate: &::core::option::Option<super::DataTemplate>, newcontenttemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnContentTemplateSelectorChanged(&self, oldcontenttemplateselector: &::core::option::Option<DataTemplateSelector>, newcontenttemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IContentPresenterOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IContentPresenterOverrides";
}
impl IContentPresenterOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPresenterOverrides_Impl, const OFFSET: isize>() -> IContentPresenterOverrides_Vtbl {
        unsafe extern "system" fn OnContentTemplateChanged<Identity: ::windows::core::IUnknownImpl, Impl: IContentPresenterOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplate: ::windows::core::RawPtr, newcontenttemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentTemplateChanged(::core::mem::transmute(&oldcontenttemplate), ::core::mem::transmute(&newcontenttemplate)).into()
        }
        unsafe extern "system" fn OnContentTemplateSelectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IContentPresenterOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontenttemplateselector: ::windows::core::RawPtr, newcontenttemplateselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentTemplateSelectorChanged(::core::mem::transmute(&oldcontenttemplateselector), ::core::mem::transmute(&newcontenttemplateselector)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPresenterOverrides, OFFSET>(),
            OnContentTemplateChanged: OnContentTemplateChanged::<Identity, Impl, OFFSET>,
            OnContentTemplateSelectorChanged: OnContentTemplateSelectorChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPresenterOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IControlOverrides_Impl: Sized {
    fn OnPointerEntered(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerPressed(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerMoved(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerReleased(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerExited(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerCaptureLost(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerCanceled(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPointerWheelChanged(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnTapped(&self, e: &::core::option::Option<super::Input::TappedRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDoubleTapped(&self, e: &::core::option::Option<super::Input::DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnHolding(&self, e: &::core::option::Option<super::Input::HoldingRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnRightTapped(&self, e: &::core::option::Option<super::Input::RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnManipulationStarting(&self, e: &::core::option::Option<super::Input::ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnManipulationInertiaStarting(&self, e: &::core::option::Option<super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnManipulationStarted(&self, e: &::core::option::Option<super::Input::ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnManipulationDelta(&self, e: &::core::option::Option<super::Input::ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnManipulationCompleted(&self, e: &::core::option::Option<super::Input::ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnKeyUp(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnKeyDown(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnGotFocus(&self, e: &::core::option::Option<super::RoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnLostFocus(&self, e: &::core::option::Option<super::RoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDragEnter(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDragLeave(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDragOver(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDrop(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlOverrides";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IControlOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>() -> IControlOverrides_Vtbl {
        unsafe extern "system" fn OnPointerEntered<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerEntered(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerPressed<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerPressed(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerMoved<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerMoved(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerReleased<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerReleased(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerExited<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerExited(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerCaptureLost<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerCaptureLost(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerCanceled<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerCanceled(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPointerWheelChanged<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPointerWheelChanged(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnTapped<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTapped(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnDoubleTapped<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDoubleTapped(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnHolding<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnHolding(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnRightTapped<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRightTapped(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnManipulationStarting<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManipulationStarting(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnManipulationInertiaStarting<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManipulationInertiaStarting(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnManipulationStarted<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManipulationStarted(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnManipulationDelta<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManipulationDelta(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnManipulationCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManipulationCompleted(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnKeyUp<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnKeyUp(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnKeyDown<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnKeyDown(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnGotFocus<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnGotFocus(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnLostFocus<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLostFocus(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnDragEnter<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDragEnter(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnDragLeave<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDragLeave(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnDragOver<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDragOver(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnDrop<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDrop(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlOverrides, OFFSET>(),
            OnPointerEntered: OnPointerEntered::<Identity, Impl, OFFSET>,
            OnPointerPressed: OnPointerPressed::<Identity, Impl, OFFSET>,
            OnPointerMoved: OnPointerMoved::<Identity, Impl, OFFSET>,
            OnPointerReleased: OnPointerReleased::<Identity, Impl, OFFSET>,
            OnPointerExited: OnPointerExited::<Identity, Impl, OFFSET>,
            OnPointerCaptureLost: OnPointerCaptureLost::<Identity, Impl, OFFSET>,
            OnPointerCanceled: OnPointerCanceled::<Identity, Impl, OFFSET>,
            OnPointerWheelChanged: OnPointerWheelChanged::<Identity, Impl, OFFSET>,
            OnTapped: OnTapped::<Identity, Impl, OFFSET>,
            OnDoubleTapped: OnDoubleTapped::<Identity, Impl, OFFSET>,
            OnHolding: OnHolding::<Identity, Impl, OFFSET>,
            OnRightTapped: OnRightTapped::<Identity, Impl, OFFSET>,
            OnManipulationStarting: OnManipulationStarting::<Identity, Impl, OFFSET>,
            OnManipulationInertiaStarting: OnManipulationInertiaStarting::<Identity, Impl, OFFSET>,
            OnManipulationStarted: OnManipulationStarted::<Identity, Impl, OFFSET>,
            OnManipulationDelta: OnManipulationDelta::<Identity, Impl, OFFSET>,
            OnManipulationCompleted: OnManipulationCompleted::<Identity, Impl, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, Impl, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            OnGotFocus: OnGotFocus::<Identity, Impl, OFFSET>,
            OnLostFocus: OnLostFocus::<Identity, Impl, OFFSET>,
            OnDragEnter: OnDragEnter::<Identity, Impl, OFFSET>,
            OnDragLeave: OnDragLeave::<Identity, Impl, OFFSET>,
            OnDragOver: OnDragOver::<Identity, Impl, OFFSET>,
            OnDrop: OnDrop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IControlOverrides6_Impl: Sized {
    fn OnPreviewKeyDown(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnPreviewKeyUp(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnCharacterReceived(&self, e: &::core::option::Option<super::Input::CharacterReceivedRoutedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IControlOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IControlOverrides6";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IControlOverrides6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides6_Impl, const OFFSET: isize>() -> IControlOverrides6_Vtbl {
        unsafe extern "system" fn OnPreviewKeyDown<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPreviewKeyDown(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnPreviewKeyUp<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPreviewKeyUp(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnCharacterReceived<Identity: ::windows::core::IUnknownImpl, Impl: IControlOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCharacterReceived(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlOverrides6, OFFSET>(),
            OnPreviewKeyDown: OnPreviewKeyDown::<Identity, Impl, OFFSET>,
            OnPreviewKeyUp: OnPreviewKeyUp::<Identity, Impl, OFFSET>,
            OnCharacterReceived: OnCharacterReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlOverrides6 as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateSelectorOverrides_Impl: Sized {
    fn SelectTemplateCore(&self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DataTemplate>;
}
impl ::windows::core::RuntimeName for IDataTemplateSelectorOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IDataTemplateSelectorOverrides";
}
impl IDataTemplateSelectorOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateSelectorOverrides_Impl, const OFFSET: isize>() -> IDataTemplateSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectTemplateCore<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateSelectorOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectTemplateCore(::core::mem::transmute(&item), ::core::mem::transmute(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateSelectorOverrides, OFFSET>(),
            SelectTemplateCore: SelectTemplateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateSelectorOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateSelectorOverrides2_Impl: Sized {
    fn SelectTemplateForItemCore(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DataTemplate>;
}
impl ::windows::core::RuntimeName for IDataTemplateSelectorOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IDataTemplateSelectorOverrides2";
}
impl IDataTemplateSelectorOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateSelectorOverrides2_Impl, const OFFSET: isize>() -> IDataTemplateSelectorOverrides2_Vtbl {
        unsafe extern "system" fn SelectTemplateForItemCore<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateSelectorOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectTemplateForItemCore(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateSelectorOverrides2, OFFSET>(),
            SelectTemplateForItemCore: SelectTemplateForItemCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateSelectorOverrides2 as ::windows::core::Interface>::IID
    }
}
pub trait IGroupStyleSelectorOverrides_Impl: Sized {
    fn SelectGroupStyleCore(&self, group: &::core::option::Option<::windows::core::IInspectable>, level: u32) -> ::windows::core::Result<GroupStyle>;
}
impl ::windows::core::RuntimeName for IGroupStyleSelectorOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IGroupStyleSelectorOverrides";
}
impl IGroupStyleSelectorOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupStyleSelectorOverrides_Impl, const OFFSET: isize>() -> IGroupStyleSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectGroupStyleCore<Identity: ::windows::core::IUnknownImpl, Impl: IGroupStyleSelectorOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void, level: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectGroupStyleCore(::core::mem::transmute(&group), level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGroupStyleSelectorOverrides, OFFSET>(),
            SelectGroupStyleCore: SelectGroupStyleCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupStyleSelectorOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
pub trait IInkToolbarCustomPenOverrides_Impl: Sized {
    fn CreateInkDrawingAttributesCore(&self, brush: &::core::option::Option<super::Media::Brush>, strokewidth: f64) -> ::windows::core::Result<super::super::Input::Inking::InkDrawingAttributes>;
}
#[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
impl ::windows::core::RuntimeName for IInkToolbarCustomPenOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IInkToolbarCustomPenOverrides";
}
#[cfg(all(feature = "UI_Input_Inking", feature = "UI_Xaml_Media"))]
impl IInkToolbarCustomPenOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkToolbarCustomPenOverrides_Impl, const OFFSET: isize>() -> IInkToolbarCustomPenOverrides_Vtbl {
        unsafe extern "system" fn CreateInkDrawingAttributesCore<Identity: ::windows::core::IUnknownImpl, Impl: IInkToolbarCustomPenOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr, strokewidth: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInkDrawingAttributesCore(::core::mem::transmute(&brush), strokewidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkToolbarCustomPenOverrides, OFFSET>(),
            CreateInkDrawingAttributesCore: CreateInkDrawingAttributesCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkToolbarCustomPenOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IInsertionPanel_Impl: Sized {
    fn GetInsertionIndexes(&self, position: &super::super::super::Foundation::Point, first: &mut i32, second: &mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInsertionPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IInsertionPanel";
}
#[cfg(feature = "Foundation")]
impl IInsertionPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInsertionPanel_Impl, const OFFSET: isize>() -> IInsertionPanel_Vtbl {
        unsafe extern "system" fn GetInsertionIndexes<Identity: ::windows::core::IUnknownImpl, Impl: IInsertionPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, first: *mut i32, second: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInsertionIndexes(::core::mem::transmute(&position), ::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInsertionPanel, OFFSET>(),
            GetInsertionIndexes: GetInsertionIndexes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInsertionPanel as ::windows::core::Interface>::IID
    }
}
pub trait IItemContainerMapping_Impl: Sized {
    fn ItemFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromItem(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn IndexFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn ContainerFromIndex(&self, index: i32) -> ::windows::core::Result<super::DependencyObject>;
}
impl ::windows::core::RuntimeName for IItemContainerMapping {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IItemContainerMapping";
}
impl IItemContainerMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const OFFSET: isize>() -> IItemContainerMapping_Vtbl {
        unsafe extern "system" fn ItemFromContainer<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemFromContainer(::core::mem::transmute(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromItem<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContainerFromItem(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexFromContainer<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IndexFromContainer(::core::mem::transmute(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromIndex<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemContainerMapping, OFFSET>(),
            ItemFromContainer: ItemFromContainer::<Identity, Impl, OFFSET>,
            ContainerFromItem: ContainerFromItem::<Identity, Impl, OFFSET>,
            IndexFromContainer: IndexFromContainer::<Identity, Impl, OFFSET>,
            ContainerFromIndex: ContainerFromIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerMapping as ::windows::core::Interface>::IID
    }
}
pub trait IItemsControlOverrides_Impl: Sized {
    fn IsItemItsOwnContainerOverride(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn GetContainerForItemOverride(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn ClearContainerForItemOverride(&self, element: &::core::option::Option<super::DependencyObject>, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn PrepareContainerForItemOverride(&self, element: &::core::option::Option<super::DependencyObject>, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnItemsChanged(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnItemContainerStyleChanged(&self, olditemcontainerstyle: &::core::option::Option<super::Style>, newitemcontainerstyle: &::core::option::Option<super::Style>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnItemContainerStyleSelectorChanged(&self, olditemcontainerstyleselector: &::core::option::Option<StyleSelector>, newitemcontainerstyleselector: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnItemTemplateChanged(&self, olditemtemplate: &::core::option::Option<super::DataTemplate>, newitemtemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnItemTemplateSelectorChanged(&self, olditemtemplateselector: &::core::option::Option<DataTemplateSelector>, newitemtemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnGroupStyleSelectorChanged(&self, oldgroupstyleselector: &::core::option::Option<GroupStyleSelector>, newgroupstyleselector: &::core::option::Option<GroupStyleSelector>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IItemsControlOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IItemsControlOverrides";
}
impl IItemsControlOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>() -> IItemsControlOverrides_Vtbl {
        unsafe extern "system" fn IsItemItsOwnContainerOverride<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsItemItsOwnContainerOverride(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerForItemOverride<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerForItemOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearContainerForItemOverride<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearContainerForItemOverride(::core::mem::transmute(&element), ::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn PrepareContainerForItemOverride<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrepareContainerForItemOverride(::core::mem::transmute(&element), ::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemsChanged(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnItemContainerStyleChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemcontainerstyle: ::windows::core::RawPtr, newitemcontainerstyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemContainerStyleChanged(::core::mem::transmute(&olditemcontainerstyle), ::core::mem::transmute(&newitemcontainerstyle)).into()
        }
        unsafe extern "system" fn OnItemContainerStyleSelectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemcontainerstyleselector: ::windows::core::RawPtr, newitemcontainerstyleselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemContainerStyleSelectorChanged(::core::mem::transmute(&olditemcontainerstyleselector), ::core::mem::transmute(&newitemcontainerstyleselector)).into()
        }
        unsafe extern "system" fn OnItemTemplateChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemtemplate: ::windows::core::RawPtr, newitemtemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemTemplateChanged(::core::mem::transmute(&olditemtemplate), ::core::mem::transmute(&newitemtemplate)).into()
        }
        unsafe extern "system" fn OnItemTemplateSelectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olditemtemplateselector: ::windows::core::RawPtr, newitemtemplateselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemTemplateSelectorChanged(::core::mem::transmute(&olditemtemplateselector), ::core::mem::transmute(&newitemtemplateselector)).into()
        }
        unsafe extern "system" fn OnGroupStyleSelectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldgroupstyleselector: ::windows::core::RawPtr, newgroupstyleselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnGroupStyleSelectorChanged(::core::mem::transmute(&oldgroupstyleselector), ::core::mem::transmute(&newgroupstyleselector)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlOverrides, OFFSET>(),
            IsItemItsOwnContainerOverride: IsItemItsOwnContainerOverride::<Identity, Impl, OFFSET>,
            GetContainerForItemOverride: GetContainerForItemOverride::<Identity, Impl, OFFSET>,
            ClearContainerForItemOverride: ClearContainerForItemOverride::<Identity, Impl, OFFSET>,
            PrepareContainerForItemOverride: PrepareContainerForItemOverride::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
            OnItemContainerStyleChanged: OnItemContainerStyleChanged::<Identity, Impl, OFFSET>,
            OnItemContainerStyleSelectorChanged: OnItemContainerStyleSelectorChanged::<Identity, Impl, OFFSET>,
            OnItemTemplateChanged: OnItemTemplateChanged::<Identity, Impl, OFFSET>,
            OnItemTemplateSelectorChanged: OnItemTemplateSelectorChanged::<Identity, Impl, OFFSET>,
            OnGroupStyleSelectorChanged: OnGroupStyleSelectorChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait INavigate_Impl: Sized {
    fn Navigate(&self, sourcepagetype: &super::Interop::TypeName) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for INavigate {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.INavigate";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl INavigate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigate_Impl, const OFFSET: isize>() -> INavigate_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows::core::IUnknownImpl, Impl: INavigate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Navigate(::core::mem::transmute(&sourcepagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INavigate, OFFSET>(), Navigate: Navigate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Navigation")]
pub trait IPageOverrides_Impl: Sized {
    fn OnNavigatedFrom(&self, e: &::core::option::Option<super::Navigation::NavigationEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnNavigatedTo(&self, e: &::core::option::Option<super::Navigation::NavigationEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnNavigatingFrom(&self, e: &::core::option::Option<super::Navigation::NavigatingCancelEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Navigation")]
impl ::windows::core::RuntimeName for IPageOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IPageOverrides";
}
#[cfg(feature = "UI_Xaml_Navigation")]
impl IPageOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPageOverrides_Impl, const OFFSET: isize>() -> IPageOverrides_Vtbl {
        unsafe extern "system" fn OnNavigatedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IPageOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNavigatedFrom(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnNavigatedTo<Identity: ::windows::core::IUnknownImpl, Impl: IPageOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNavigatedTo(::core::mem::transmute(&e)).into()
        }
        unsafe extern "system" fn OnNavigatingFrom<Identity: ::windows::core::IUnknownImpl, Impl: IPageOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNavigatingFrom(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPageOverrides, OFFSET>(),
            OnNavigatedFrom: OnNavigatedFrom::<Identity, Impl, OFFSET>,
            OnNavigatedTo: OnNavigatedTo::<Identity, Impl, OFFSET>,
            OnNavigatingFrom: OnNavigatingFrom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPageOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IScrollAnchorProvider_Impl: Sized {
    fn CurrentAnchor(&self) -> ::windows::core::Result<super::UIElement>;
    fn RegisterAnchorCandidate(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn UnregisterAnchorCandidate(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollAnchorProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IScrollAnchorProvider";
}
impl IScrollAnchorProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>() -> IScrollAnchorProvider_Vtbl {
        unsafe extern "system" fn CurrentAnchor<Identity: ::windows::core::IUnknownImpl, Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAnchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAnchorCandidate<Identity: ::windows::core::IUnknownImpl, Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterAnchorCandidate(::core::mem::transmute(&element)).into()
        }
        unsafe extern "system" fn UnregisterAnchorCandidate<Identity: ::windows::core::IUnknownImpl, Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterAnchorCandidate(::core::mem::transmute(&element)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollAnchorProvider, OFFSET>(),
            CurrentAnchor: CurrentAnchor::<Identity, Impl, OFFSET>,
            RegisterAnchorCandidate: RegisterAnchorCandidate::<Identity, Impl, OFFSET>,
            UnregisterAnchorCandidate: UnregisterAnchorCandidate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollAnchorProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISemanticZoomInformation_Impl: Sized {
    fn SemanticZoomOwner(&self) -> ::windows::core::Result<SemanticZoom>;
    fn SetSemanticZoomOwner(&self, value: &::core::option::Option<SemanticZoom>) -> ::windows::core::Result<()>;
    fn IsActiveView(&self) -> ::windows::core::Result<bool>;
    fn SetIsActiveView(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomedInView(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomedInView(&self, value: bool) -> ::windows::core::Result<()>;
    fn InitializeViewChange(&self) -> ::windows::core::Result<()>;
    fn CompleteViewChange(&self) -> ::windows::core::Result<()>;
    fn MakeVisible(&self, item: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeFrom(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeTo(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeFrom(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeTo(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISemanticZoomInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ISemanticZoomInformation";
}
impl ISemanticZoomInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>() -> ISemanticZoomInformation_Vtbl {
        unsafe extern "system" fn SemanticZoomOwner<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SemanticZoomOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSemanticZoomOwner<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSemanticZoomOwner(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsActiveView<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActiveView<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsActiveView(value).into()
        }
        unsafe extern "system" fn IsZoomedInView<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsZoomedInView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZoomedInView<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsZoomedInView(value).into()
        }
        unsafe extern "system" fn InitializeViewChange<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeViewChange().into()
        }
        unsafe extern "system" fn CompleteViewChange<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompleteViewChange().into()
        }
        unsafe extern "system" fn MakeVisible<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeVisible(::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn StartViewChangeFrom<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartViewChangeFrom(::core::mem::transmute(&source), ::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn StartViewChangeTo<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartViewChangeTo(::core::mem::transmute(&source), ::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn CompleteViewChangeFrom<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompleteViewChangeFrom(::core::mem::transmute(&source), ::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn CompleteViewChangeTo<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompleteViewChangeTo(::core::mem::transmute(&source), ::core::mem::transmute(&destination)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticZoomInformation, OFFSET>(),
            SemanticZoomOwner: SemanticZoomOwner::<Identity, Impl, OFFSET>,
            SetSemanticZoomOwner: SetSemanticZoomOwner::<Identity, Impl, OFFSET>,
            IsActiveView: IsActiveView::<Identity, Impl, OFFSET>,
            SetIsActiveView: SetIsActiveView::<Identity, Impl, OFFSET>,
            IsZoomedInView: IsZoomedInView::<Identity, Impl, OFFSET>,
            SetIsZoomedInView: SetIsZoomedInView::<Identity, Impl, OFFSET>,
            InitializeViewChange: InitializeViewChange::<Identity, Impl, OFFSET>,
            CompleteViewChange: CompleteViewChange::<Identity, Impl, OFFSET>,
            MakeVisible: MakeVisible::<Identity, Impl, OFFSET>,
            StartViewChangeFrom: StartViewChangeFrom::<Identity, Impl, OFFSET>,
            StartViewChangeTo: StartViewChangeTo::<Identity, Impl, OFFSET>,
            CompleteViewChangeFrom: CompleteViewChangeFrom::<Identity, Impl, OFFSET>,
            CompleteViewChangeTo: CompleteViewChangeTo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticZoomInformation as ::windows::core::Interface>::IID
    }
}
pub trait IStyleSelectorOverrides_Impl: Sized {
    fn SelectStyleCore(&self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::Style>;
}
impl ::windows::core::RuntimeName for IStyleSelectorOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IStyleSelectorOverrides";
}
impl IStyleSelectorOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStyleSelectorOverrides_Impl, const OFFSET: isize>() -> IStyleSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectStyleCore<Identity: ::windows::core::IUnknownImpl, Impl: IStyleSelectorOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectStyleCore(::core::mem::transmute(&item), ::core::mem::transmute(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStyleSelectorOverrides, OFFSET>(),
            SelectStyleCore: SelectStyleCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStyleSelectorOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IToggleSwitchOverrides_Impl: Sized {
    fn OnToggled(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnOnContentChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnOffContentChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnHeaderChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IToggleSwitchOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IToggleSwitchOverrides";
}
impl IToggleSwitchOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>() -> IToggleSwitchOverrides_Vtbl {
        unsafe extern "system" fn OnToggled<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnToggled().into()
        }
        unsafe extern "system" fn OnOnContentChanged<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOnContentChanged(::core::mem::transmute(&oldcontent), ::core::mem::transmute(&newcontent)).into()
        }
        unsafe extern "system" fn OnOffContentChanged<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOffContentChanged(::core::mem::transmute(&oldcontent), ::core::mem::transmute(&newcontent)).into()
        }
        unsafe extern "system" fn OnHeaderChanged<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcontent: *mut ::core::ffi::c_void, newcontent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnHeaderChanged(::core::mem::transmute(&oldcontent), ::core::mem::transmute(&newcontent)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleSwitchOverrides, OFFSET>(),
            OnToggled: OnToggled::<Identity, Impl, OFFSET>,
            OnOnContentChanged: OnOnContentChanged::<Identity, Impl, OFFSET>,
            OnOffContentChanged: OnOffContentChanged::<Identity, Impl, OFFSET>,
            OnHeaderChanged: OnHeaderChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleSwitchOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
pub trait IVirtualizingPanelOverrides_Impl: Sized {
    fn OnItemsChanged(&self, sender: &::core::option::Option<::windows::core::IInspectable>, args: &::core::option::Option<Primitives::ItemsChangedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnClearChildren(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn BringIndexIntoView(&self, index: i32) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
impl ::windows::core::RuntimeName for IVirtualizingPanelOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IVirtualizingPanelOverrides";
}
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
impl IVirtualizingPanelOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>() -> IVirtualizingPanelOverrides_Vtbl {
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemsChanged(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnClearChildren<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnClearChildren().into()
        }
        unsafe extern "system" fn BringIndexIntoView<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BringIndexIntoView(index).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualizingPanelOverrides, OFFSET>(),
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
            OnClearChildren: OnClearChildren::<Identity, Impl, OFFSET>,
            BringIndexIntoView: BringIndexIntoView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizingPanelOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualizingStackPanelOverrides_Impl: Sized {
    fn OnCleanUpVirtualizedItem(&self, e: &::core::option::Option<CleanUpVirtualizedItemEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IVirtualizingStackPanelOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IVirtualizingStackPanelOverrides";
}
impl IVirtualizingStackPanelOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingStackPanelOverrides_Impl, const OFFSET: isize>() -> IVirtualizingStackPanelOverrides_Vtbl {
        unsafe extern "system" fn OnCleanUpVirtualizedItem<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizingStackPanelOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCleanUpVirtualizedItem(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVirtualizingStackPanelOverrides, OFFSET>(),
            OnCleanUpVirtualizedItem: OnCleanUpVirtualizedItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizingStackPanelOverrides as ::windows::core::Interface>::IID
    }
}
