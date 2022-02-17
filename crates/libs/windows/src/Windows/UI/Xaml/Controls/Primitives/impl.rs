pub trait IFlyoutBaseOverrides_Impl: Sized {
    fn CreatePresenter(&self) -> ::windows::core::Result<super::Control>;
}
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides";
}
impl IFlyoutBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides_Impl, const OFFSET: isize>() -> IFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn CreatePresenter<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePresenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseOverrides, OFFSET>(),
            CreatePresenter: CreatePresenter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IFlyoutBaseOverrides4_Impl: Sized {
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides4";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IFlyoutBaseOverrides4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides4_Impl, const OFFSET: isize>() -> IFlyoutBaseOverrides4_Vtbl {
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(::core::mem::transmute(&args)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseOverrides4, OFFSET>(),
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseOverrides4 as ::windows::core::Interface>::IID
    }
}
pub trait IPickerFlyoutBaseOverrides_Impl: Sized {
    fn OnConfirmed(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn ShouldShowConfirmationButtons(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPickerFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseOverrides";
}
impl IPickerFlyoutBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>() -> IPickerFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConfirmed<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConfirmed().into()
        }
        unsafe extern "system" fn ShouldShowConfirmationButtons<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShouldShowConfirmationButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutBaseOverrides, OFFSET>(),
            OnConfirmed: OnConfirmed::<Identity, Impl, OFFSET>,
            ShouldShowConfirmationButtons: ShouldShowConfirmationButtons::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutBaseOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IRangeBaseOverrides_Impl: Sized {
    fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IRangeBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseOverrides";
}
impl IRangeBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>() -> IRangeBaseOverrides_Vtbl {
        unsafe extern "system" fn OnMinimumChanged<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldminimum: f64, newminimum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnMinimumChanged(oldminimum, newminimum).into()
        }
        unsafe extern "system" fn OnMaximumChanged<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnMaximumChanged(oldmaximum, newmaximum).into()
        }
        unsafe extern "system" fn OnValueChanged<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvalue: f64, newvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnValueChanged(oldvalue, newvalue).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseOverrides, OFFSET>(),
            OnMinimumChanged: OnMinimumChanged::<Identity, Impl, OFFSET>,
            OnMaximumChanged: OnMaximumChanged::<Identity, Impl, OFFSET>,
            OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IScrollSnapPointsInfo_Impl: Sized {
    fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn HorizontalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[cfg(feature = "Foundation_Collections")]
impl IScrollSnapPointsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>() -> IScrollSnapPointsInfo_Vtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AreHorizontalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreVerticalSnapPointsRegular<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AreVerticalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalSnapPointsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontalSnapPointsChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveHorizontalSnapPointsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticalSnapPointsChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveVerticalSnapPointsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIrregularSnapPoints(orientation, alignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularSnapPoints<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegularSnapPoints(orientation, alignment, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollSnapPointsInfo, OFFSET>(),
            AreHorizontalSnapPointsRegular: AreHorizontalSnapPointsRegular::<Identity, Impl, OFFSET>,
            AreVerticalSnapPointsRegular: AreVerticalSnapPointsRegular::<Identity, Impl, OFFSET>,
            HorizontalSnapPointsChanged: HorizontalSnapPointsChanged::<Identity, Impl, OFFSET>,
            RemoveHorizontalSnapPointsChanged: RemoveHorizontalSnapPointsChanged::<Identity, Impl, OFFSET>,
            VerticalSnapPointsChanged: VerticalSnapPointsChanged::<Identity, Impl, OFFSET>,
            RemoveVerticalSnapPointsChanged: RemoveVerticalSnapPointsChanged::<Identity, Impl, OFFSET>,
            GetIrregularSnapPoints: GetIrregularSnapPoints::<Identity, Impl, OFFSET>,
            GetRegularSnapPoints: GetRegularSnapPoints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollSnapPointsInfo as ::windows::core::Interface>::IID
    }
}
pub trait IToggleButtonOverrides_Impl: Sized {
    fn OnToggle(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IToggleButtonOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonOverrides";
}
impl IToggleButtonOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonOverrides_Impl, const OFFSET: isize>() -> IToggleButtonOverrides_Vtbl {
        unsafe extern "system" fn OnToggle<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnToggle().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonOverrides, OFFSET>(), OnToggle: OnToggle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonOverrides as ::windows::core::Interface>::IID
    }
}
