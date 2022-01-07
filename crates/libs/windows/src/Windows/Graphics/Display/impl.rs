#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedColorInfoImpl: Sized {
    fn CurrentAdvancedColorKind(&self) -> ::windows::core::Result<AdvancedColorKind>;
    fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn SdrWhiteLevelInNits(&self) -> ::windows::core::Result<f32>;
    fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat) -> ::windows::core::Result<bool>;
    fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedColorInfo {
    const NAME: &'static str = "Windows.Graphics.Display.IAdvancedColorInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedColorInfoVtbl {
    pub const fn new<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedColorInfoVtbl {
        unsafe extern "system" fn CurrentAdvancedColorKind<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AdvancedColorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentAdvancedColorKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedPrimary<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GreenPrimary<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GreenPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BluePrimary<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluePrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhitePoint<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhitePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLuminanceInNits<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLuminanceInNits<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAverageFullFrameLuminanceInNits<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxAverageFullFrameLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SdrWhiteLevelInNits<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SdrWhiteLevelInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHdrMetadataFormatCurrentlySupported<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: HdrMetadataFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHdrMetadataFormatCurrentlySupported(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAdvancedColorKindAvailable<Impl: IAdvancedColorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: AdvancedColorKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAdvancedColorKindAvailable(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAdvancedColorInfo>,
            base.5,
            CurrentAdvancedColorKind::<Impl, OFFSET>,
            RedPrimary::<Impl, OFFSET>,
            GreenPrimary::<Impl, OFFSET>,
            BluePrimary::<Impl, OFFSET>,
            WhitePoint::<Impl, OFFSET>,
            MaxLuminanceInNits::<Impl, OFFSET>,
            MinLuminanceInNits::<Impl, OFFSET>,
            MaxAverageFullFrameLuminanceInNits::<Impl, OFFSET>,
            SdrWhiteLevelInNits::<Impl, OFFSET>,
            IsHdrMetadataFormatCurrentlySupported::<Impl, OFFSET>,
            IsAdvancedColorKindAvailable::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsOverrideActive(&self) -> ::windows::core::Result<bool>;
    fn BrightnessLevel(&self) -> ::windows::core::Result<f64>;
    fn SetBrightnessLevel(&self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()>;
    fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()>;
    fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario) -> ::windows::core::Result<f64>;
    fn StartOverride(&self) -> ::windows::core::Result<()>;
    fn StopOverride(&self) -> ::windows::core::Result<()>;
    fn IsSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsOverrideActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsOverrideActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BrightnessLevelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBrightnessLevelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrightnessOverride {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverride";
}
#[cfg(feature = "implement_exclusive")]
impl IBrightnessOverrideVtbl {
    pub const fn new<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrightnessOverrideVtbl {
        unsafe extern "system" fn IsSupported<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOverrideActive<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverrideActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrightnessLevel<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrightnessLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightnessLevel<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBrightnessLevel(brightnesslevel, options).into()
        }
        unsafe extern "system" fn SetBrightnessScenario<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBrightnessScenario(scenario, options).into()
        }
        unsafe extern "system" fn GetLevelForScenario<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevelForScenario(scenario) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOverride<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartOverride().into()
        }
        unsafe extern "system" fn StopOverride<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopOverride().into()
        }
        unsafe extern "system" fn IsSupportedChanged<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsSupportedChanged<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverrideActiveChanged<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverrideActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsOverrideActiveChanged<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsOverrideActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BrightnessLevelChanged<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrightnessLevelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBrightnessLevelChanged<Impl: IBrightnessOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBrightnessLevelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBrightnessOverride>,
            base.5,
            IsSupported::<Impl, OFFSET>,
            IsOverrideActive::<Impl, OFFSET>,
            BrightnessLevel::<Impl, OFFSET>,
            SetBrightnessLevel::<Impl, OFFSET>,
            SetBrightnessScenario::<Impl, OFFSET>,
            GetLevelForScenario::<Impl, OFFSET>,
            StartOverride::<Impl, OFFSET>,
            StopOverride::<Impl, OFFSET>,
            IsSupportedChanged::<Impl, OFFSET>,
            RemoveIsSupportedChanged::<Impl, OFFSET>,
            IsOverrideActiveChanged::<Impl, OFFSET>,
            RemoveIsOverrideActiveChanged::<Impl, OFFSET>,
            BrightnessLevelChanged::<Impl, OFFSET>,
            RemoveBrightnessLevelChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideSettingsImpl: Sized {
    fn DesiredLevel(&self) -> ::windows::core::Result<f64>;
    fn DesiredNits(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrightnessOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverrideSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IBrightnessOverrideSettingsVtbl {
    pub const fn new<Impl: IBrightnessOverrideSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrightnessOverrideSettingsVtbl {
        unsafe extern "system" fn DesiredLevel<Impl: IBrightnessOverrideSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredNits<Impl: IBrightnessOverrideSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrightnessOverrideSettings>, base.5, DesiredLevel::<Impl, OFFSET>, DesiredNits::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideSettingsStaticsImpl: Sized {
    fn CreateFromLevel(&self, level: f64) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn CreateFromNits(&self, nits: f32) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn CreateFromDisplayBrightnessOverrideScenario(&self, overridescenario: DisplayBrightnessOverrideScenario) -> ::windows::core::Result<BrightnessOverrideSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrightnessOverrideSettingsStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverrideSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBrightnessOverrideSettingsStaticsVtbl {
    pub const fn new<Impl: IBrightnessOverrideSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrightnessOverrideSettingsStaticsVtbl {
        unsafe extern "system" fn CreateFromLevel<Impl: IBrightnessOverrideSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNits<Impl: IBrightnessOverrideSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nits: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromNits(nits) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDisplayBrightnessOverrideScenario<Impl: IBrightnessOverrideSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDisplayBrightnessOverrideScenario(overridescenario) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrightnessOverrideSettingsStatics>, base.5, CreateFromLevel::<Impl, OFFSET>, CreateFromNits::<Impl, OFFSET>, CreateFromDisplayBrightnessOverrideScenario::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideStaticsImpl: Sized {
    fn GetDefaultForSystem(&self) -> ::windows::core::Result<BrightnessOverride>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<BrightnessOverride>;
    fn SaveForSystemAsync(&self, value: &::core::option::Option<BrightnessOverride>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrightnessOverrideStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverrideStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBrightnessOverrideStaticsVtbl {
    pub const fn new<Impl: IBrightnessOverrideStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrightnessOverrideStaticsVtbl {
        unsafe extern "system" fn GetDefaultForSystem<Impl: IBrightnessOverrideStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultForSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentView<Impl: IBrightnessOverrideStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveForSystemAsync<Impl: IBrightnessOverrideStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveForSystemAsync(&*(&value as *const <BrightnessOverride as ::windows::core::Abi>::Abi as *const <BrightnessOverride as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrightnessOverrideStatics>, base.5, GetDefaultForSystem::<Impl, OFFSET>, GetForCurrentView::<Impl, OFFSET>, SaveForSystemAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorOverrideSettingsImpl: Sized {
    fn DesiredDisplayColorOverrideScenario(&self) -> ::windows::core::Result<DisplayColorOverrideScenario>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.IColorOverrideSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IColorOverrideSettingsVtbl {
    pub const fn new<Impl: IColorOverrideSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorOverrideSettingsVtbl {
        unsafe extern "system" fn DesiredDisplayColorOverrideScenario<Impl: IColorOverrideSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayColorOverrideScenario) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredDisplayColorOverrideScenario() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorOverrideSettings>, base.5, DesiredDisplayColorOverrideScenario::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorOverrideSettingsStaticsImpl: Sized {
    fn CreateFromDisplayColorOverrideScenario(&self, overridescenario: DisplayColorOverrideScenario) -> ::windows::core::Result<ColorOverrideSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorOverrideSettingsStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IColorOverrideSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorOverrideSettingsStaticsVtbl {
    pub const fn new<Impl: IColorOverrideSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorOverrideSettingsStaticsVtbl {
        unsafe extern "system" fn CreateFromDisplayColorOverrideScenario<Impl: IColorOverrideSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overridescenario: DisplayColorOverrideScenario, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDisplayColorOverrideScenario(overridescenario) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorOverrideSettingsStatics>, base.5, CreateFromDisplayColorOverrideScenario::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideImpl: Sized {
    fn ColorOverrideSettings(&self) -> ::windows::core::Result<ColorOverrideSettings>;
    fn SetColorOverrideSettings(&self, value: &::core::option::Option<ColorOverrideSettings>) -> ::windows::core::Result<()>;
    fn BrightnessOverrideSettings(&self) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn SetBrightnessOverrideSettings(&self, value: &::core::option::Option<BrightnessOverrideSettings>) -> ::windows::core::Result<()>;
    fn CanOverride(&self) -> ::windows::core::Result<bool>;
    fn IsOverrideActive(&self) -> ::windows::core::Result<bool>;
    fn GetCurrentDisplayEnhancementOverrideCapabilities(&self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities>;
    fn RequestOverride(&self) -> ::windows::core::Result<()>;
    fn StopOverride(&self) -> ::windows::core::Result<()>;
    fn CanOverrideChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanOverrideChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsOverrideActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsOverrideActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayEnhancementOverrideCapabilitiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayEnhancementOverrideCapabilitiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverride {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverride";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayEnhancementOverrideVtbl {
    pub const fn new<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayEnhancementOverrideVtbl {
        unsafe extern "system" fn ColorOverrideSettings<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorOverrideSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorOverrideSettings<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColorOverrideSettings(&*(&value as *const <ColorOverrideSettings as ::windows::core::Abi>::Abi as *const <ColorOverrideSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BrightnessOverrideSettings<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrightnessOverrideSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightnessOverrideSettings<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBrightnessOverrideSettings(&*(&value as *const <BrightnessOverrideSettings as ::windows::core::Abi>::Abi as *const <BrightnessOverrideSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanOverride<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOverrideActive<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverrideActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDisplayEnhancementOverrideCapabilities<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentDisplayEnhancementOverrideCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestOverride<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestOverride().into()
        }
        unsafe extern "system" fn StopOverride<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopOverride().into()
        }
        unsafe extern "system" fn CanOverrideChanged<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanOverrideChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanOverrideChanged<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCanOverrideChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverrideActiveChanged<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverrideActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsOverrideActiveChanged<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsOverrideActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayEnhancementOverrideCapabilitiesChanged<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayEnhancementOverrideCapabilitiesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayEnhancementOverrideCapabilitiesChanged<Impl: IDisplayEnhancementOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDisplayEnhancementOverrideCapabilitiesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDisplayEnhancementOverride>,
            base.5,
            ColorOverrideSettings::<Impl, OFFSET>,
            SetColorOverrideSettings::<Impl, OFFSET>,
            BrightnessOverrideSettings::<Impl, OFFSET>,
            SetBrightnessOverrideSettings::<Impl, OFFSET>,
            CanOverride::<Impl, OFFSET>,
            IsOverrideActive::<Impl, OFFSET>,
            GetCurrentDisplayEnhancementOverrideCapabilities::<Impl, OFFSET>,
            RequestOverride::<Impl, OFFSET>,
            StopOverride::<Impl, OFFSET>,
            CanOverrideChanged::<Impl, OFFSET>,
            RemoveCanOverrideChanged::<Impl, OFFSET>,
            IsOverrideActiveChanged::<Impl, OFFSET>,
            RemoveIsOverrideActiveChanged::<Impl, OFFSET>,
            DisplayEnhancementOverrideCapabilitiesChanged::<Impl, OFFSET>,
            RemoveDisplayEnhancementOverrideCapabilitiesChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideCapabilitiesImpl: Sized {
    fn IsBrightnessControlSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBrightnessNitsControlSupported(&self) -> ::windows::core::Result<bool>;
    fn GetSupportedNitRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NitRange>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverrideCapabilities {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayEnhancementOverrideCapabilitiesVtbl {
    pub const fn new<Impl: IDisplayEnhancementOverrideCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayEnhancementOverrideCapabilitiesVtbl {
        unsafe extern "system" fn IsBrightnessControlSupported<Impl: IDisplayEnhancementOverrideCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBrightnessControlSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBrightnessNitsControlSupported<Impl: IDisplayEnhancementOverrideCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBrightnessNitsControlSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedNitRanges<Impl: IDisplayEnhancementOverrideCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedNitRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayEnhancementOverrideCapabilities>, base.5, IsBrightnessControlSupported::<Impl, OFFSET>, IsBrightnessNitsControlSupported::<Impl, OFFSET>, GetSupportedNitRanges::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideCapabilitiesChangedEventArgsImpl: Sized {
    fn Capabilities(&self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl {
    pub const fn new<Impl: IDisplayEnhancementOverrideCapabilitiesChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl {
        unsafe extern "system" fn Capabilities<Impl: IDisplayEnhancementOverrideCapabilitiesChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayEnhancementOverrideCapabilitiesChangedEventArgs>, base.5, Capabilities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<DisplayEnhancementOverride>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverrideStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverrideStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayEnhancementOverrideStaticsVtbl {
    pub const fn new<Impl: IDisplayEnhancementOverrideStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayEnhancementOverrideStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IDisplayEnhancementOverrideStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayEnhancementOverrideStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformationImpl: Sized {
    fn CurrentOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn NativeOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn OrientationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolutionScale(&self) -> ::windows::core::Result<ResolutionScale>;
    fn LogicalDpi(&self) -> ::windows::core::Result<f32>;
    fn RawDpiX(&self) -> ::windows::core::Result<f32>;
    fn RawDpiY(&self) -> ::windows::core::Result<f32>;
    fn DpiChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDpiChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StereoEnabled(&self) -> ::windows::core::Result<bool>;
    fn StereoEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStereoEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetColorProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ColorProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformationVtbl {
    pub const fn new<Impl: IDisplayInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayInformationVtbl {
        unsafe extern "system" fn CurrentOrientation<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NativeOrientation<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientationChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrientationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOrientationChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOrientationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolutionScale<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolutionScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDpi<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LogicalDpi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDpiX<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawDpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDpiY<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawDpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DpiChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDpiChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDpiChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StereoEnabled<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoEnabledChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StereoEnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStereoEnabledChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStereoEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetColorProfileAsync<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorProfileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorProfileChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorProfileChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorProfileChanged<Impl: IDisplayInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveColorProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDisplayInformation>,
            base.5,
            CurrentOrientation::<Impl, OFFSET>,
            NativeOrientation::<Impl, OFFSET>,
            OrientationChanged::<Impl, OFFSET>,
            RemoveOrientationChanged::<Impl, OFFSET>,
            ResolutionScale::<Impl, OFFSET>,
            LogicalDpi::<Impl, OFFSET>,
            RawDpiX::<Impl, OFFSET>,
            RawDpiY::<Impl, OFFSET>,
            DpiChanged::<Impl, OFFSET>,
            RemoveDpiChanged::<Impl, OFFSET>,
            StereoEnabled::<Impl, OFFSET>,
            StereoEnabledChanged::<Impl, OFFSET>,
            RemoveStereoEnabledChanged::<Impl, OFFSET>,
            GetColorProfileAsync::<Impl, OFFSET>,
            ColorProfileChanged::<Impl, OFFSET>,
            RemoveColorProfileChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation2Impl: Sized + IDisplayInformationImpl {
    fn RawPixelsPerViewPixel(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformation2 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformation2Vtbl {
    pub const fn new<Impl: IDisplayInformation2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayInformation2Vtbl {
        unsafe extern "system" fn RawPixelsPerViewPixel<Impl: IDisplayInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawPixelsPerViewPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayInformation2>, base.5, RawPixelsPerViewPixel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation3Impl: Sized {
    fn DiagonalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformation3 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation3";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformation3Vtbl {
    pub const fn new<Impl: IDisplayInformation3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayInformation3Vtbl {
        unsafe extern "system" fn DiagonalSizeInInches<Impl: IDisplayInformation3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiagonalSizeInInches() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayInformation3>, base.5, DiagonalSizeInInches::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation4Impl: Sized {
    fn ScreenWidthInRawPixels(&self) -> ::windows::core::Result<u32>;
    fn ScreenHeightInRawPixels(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformation4 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation4";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformation4Vtbl {
    pub const fn new<Impl: IDisplayInformation4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayInformation4Vtbl {
        unsafe extern "system" fn ScreenWidthInRawPixels<Impl: IDisplayInformation4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenWidthInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenHeightInRawPixels<Impl: IDisplayInformation4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenHeightInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayInformation4>, base.5, ScreenWidthInRawPixels::<Impl, OFFSET>, ScreenHeightInRawPixels::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation5Impl: Sized {
    fn GetAdvancedColorInfo(&self) -> ::windows::core::Result<AdvancedColorInfo>;
    fn AdvancedColorInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvancedColorInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformation5 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation5";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformation5Vtbl {
    pub const fn new<Impl: IDisplayInformation5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayInformation5Vtbl {
        unsafe extern "system" fn GetAdvancedColorInfo<Impl: IDisplayInformation5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdvancedColorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvancedColorInfoChanged<Impl: IDisplayInformation5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvancedColorInfoChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdvancedColorInfoChanged<Impl: IDisplayInformation5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAdvancedColorInfoChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayInformation5>, base.5, GetAdvancedColorInfo::<Impl, OFFSET>, AdvancedColorInfoChanged::<Impl, OFFSET>, RemoveAdvancedColorInfoChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformationStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<DisplayInformation>;
    fn AutoRotationPreferences(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn SetAutoRotationPreferences(&self, value: DisplayOrientations) -> ::windows::core::Result<()>;
    fn DisplayContentsInvalidated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayContentsInvalidated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformationStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformationStaticsVtbl {
    pub const fn new<Impl: IDisplayInformationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayInformationStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IDisplayInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRotationPreferences<Impl: IDisplayInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoRotationPreferences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRotationPreferences<Impl: IDisplayInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoRotationPreferences(value).into()
        }
        unsafe extern "system" fn DisplayContentsInvalidated<Impl: IDisplayInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayContentsInvalidated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayContentsInvalidated<Impl: IDisplayInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDisplayContentsInvalidated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayInformationStatics>, base.5, GetForCurrentView::<Impl, OFFSET>, AutoRotationPreferences::<Impl, OFFSET>, SetAutoRotationPreferences::<Impl, OFFSET>, DisplayContentsInvalidated::<Impl, OFFSET>, RemoveDisplayContentsInvalidated::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDisplayPropertiesStaticsImpl: Sized {
    fn CurrentOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn NativeOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn AutoRotationPreferences(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn SetAutoRotationPreferences(&self, value: DisplayOrientations) -> ::windows::core::Result<()>;
    fn OrientationChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolutionScale(&self) -> ::windows::core::Result<ResolutionScale>;
    fn LogicalDpi(&self) -> ::windows::core::Result<f32>;
    fn LogicalDpiChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLogicalDpiChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StereoEnabled(&self) -> ::windows::core::Result<bool>;
    fn StereoEnabledChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStereoEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetColorProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ColorProfileChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayContentsInvalidated(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayContentsInvalidated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPropertiesStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayPropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDisplayPropertiesStaticsVtbl {
    pub const fn new<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayPropertiesStaticsVtbl {
        unsafe extern "system" fn CurrentOrientation<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NativeOrientation<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRotationPreferences<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoRotationPreferences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRotationPreferences<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoRotationPreferences(value).into()
        }
        unsafe extern "system" fn OrientationChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrientationChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOrientationChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOrientationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolutionScale<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolutionScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDpi<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LogicalDpi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDpiChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LogicalDpiChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogicalDpiChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLogicalDpiChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StereoEnabled<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoEnabledChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StereoEnabledChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStereoEnabledChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStereoEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetColorProfileAsync<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorProfileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorProfileChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorProfileChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorProfileChanged<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveColorProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayContentsInvalidated<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayContentsInvalidated(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayContentsInvalidated<Impl: IDisplayPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDisplayContentsInvalidated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDisplayPropertiesStatics>,
            base.5,
            CurrentOrientation::<Impl, OFFSET>,
            NativeOrientation::<Impl, OFFSET>,
            AutoRotationPreferences::<Impl, OFFSET>,
            SetAutoRotationPreferences::<Impl, OFFSET>,
            OrientationChanged::<Impl, OFFSET>,
            RemoveOrientationChanged::<Impl, OFFSET>,
            ResolutionScale::<Impl, OFFSET>,
            LogicalDpi::<Impl, OFFSET>,
            LogicalDpiChanged::<Impl, OFFSET>,
            RemoveLogicalDpiChanged::<Impl, OFFSET>,
            StereoEnabled::<Impl, OFFSET>,
            StereoEnabledChanged::<Impl, OFFSET>,
            RemoveStereoEnabledChanged::<Impl, OFFSET>,
            GetColorProfileAsync::<Impl, OFFSET>,
            ColorProfileChanged::<Impl, OFFSET>,
            RemoveColorProfileChanged::<Impl, OFFSET>,
            DisplayContentsInvalidated::<Impl, OFFSET>,
            RemoveDisplayContentsInvalidated::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayServicesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayServices {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayServices";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayServicesVtbl {
    pub const fn new<Impl: IDisplayServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayServicesVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayServices>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayServicesStaticsImpl: Sized {
    fn FindAll(&self) -> ::windows::core::Result<::windows::core::Array<super::DisplayId>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayServicesStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayServicesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayServicesStaticsVtbl {
    pub const fn new<Impl: IDisplayServicesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayServicesStaticsVtbl {
        unsafe extern "system" fn FindAll<Impl: IDisplayServicesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAll() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayServicesStatics>, base.5, FindAll::<Impl, OFFSET>)
    }
}
