#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdvancedColorInfo_Impl: Sized {
    fn CurrentAdvancedColorKind(&mut self) -> ::windows::core::Result<AdvancedColorKind>;
    fn RedPrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn GreenPrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BluePrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WhitePoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MaxLuminanceInNits(&mut self) -> ::windows::core::Result<f32>;
    fn MinLuminanceInNits(&mut self) -> ::windows::core::Result<f32>;
    fn MaxAverageFullFrameLuminanceInNits(&mut self) -> ::windows::core::Result<f32>;
    fn SdrWhiteLevelInNits(&mut self) -> ::windows::core::Result<f32>;
    fn IsHdrMetadataFormatCurrentlySupported(&mut self, format: HdrMetadataFormat) -> ::windows::core::Result<bool>;
    fn IsAdvancedColorKindAvailable(&mut self, kind: AdvancedColorKind) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedColorInfo {
    const NAME: &'static str = "Windows.Graphics.Display.IAdvancedColorInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdvancedColorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedColorInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedColorInfo_Vtbl {
        unsafe extern "system" fn CurrentAdvancedColorKind<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdvancedColorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAdvancedColorKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedPrimary<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GreenPrimary<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GreenPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BluePrimary<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluePrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhitePoint<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhitePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLuminanceInNits<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLuminanceInNits<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAverageFullFrameLuminanceInNits<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAverageFullFrameLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SdrWhiteLevelInNits<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SdrWhiteLevelInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHdrMetadataFormatCurrentlySupported<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: HdrMetadataFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHdrMetadataFormatCurrentlySupported(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAdvancedColorKindAvailable<Impl: IAdvancedColorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: AdvancedColorKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAdvancedColorKindAvailable(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedColorInfo, BASE_OFFSET>(),
            CurrentAdvancedColorKind: CurrentAdvancedColorKind::<Impl, IMPL_OFFSET>,
            RedPrimary: RedPrimary::<Impl, IMPL_OFFSET>,
            GreenPrimary: GreenPrimary::<Impl, IMPL_OFFSET>,
            BluePrimary: BluePrimary::<Impl, IMPL_OFFSET>,
            WhitePoint: WhitePoint::<Impl, IMPL_OFFSET>,
            MaxLuminanceInNits: MaxLuminanceInNits::<Impl, IMPL_OFFSET>,
            MinLuminanceInNits: MinLuminanceInNits::<Impl, IMPL_OFFSET>,
            MaxAverageFullFrameLuminanceInNits: MaxAverageFullFrameLuminanceInNits::<Impl, IMPL_OFFSET>,
            SdrWhiteLevelInNits: SdrWhiteLevelInNits::<Impl, IMPL_OFFSET>,
            IsHdrMetadataFormatCurrentlySupported: IsHdrMetadataFormatCurrentlySupported::<Impl, IMPL_OFFSET>,
            IsAdvancedColorKindAvailable: IsAdvancedColorKindAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedColorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBrightnessOverride_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsOverrideActive(&mut self) -> ::windows::core::Result<bool>;
    fn BrightnessLevel(&mut self) -> ::windows::core::Result<f64>;
    fn SetBrightnessLevel(&mut self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()>;
    fn SetBrightnessScenario(&mut self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()>;
    fn GetLevelForScenario(&mut self, scenario: DisplayBrightnessScenario) -> ::windows::core::Result<f64>;
    fn StartOverride(&mut self) -> ::windows::core::Result<()>;
    fn StopOverride(&mut self) -> ::windows::core::Result<()>;
    fn IsSupportedChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsOverrideActiveChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsOverrideActiveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BrightnessLevelChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBrightnessLevelChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBrightnessOverride {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverride";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBrightnessOverride_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrightnessOverride_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrightnessOverride_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOverrideActive<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverrideActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrightnessLevel<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrightnessLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightnessLevel<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightnessLevel(brightnesslevel, options).into()
        }
        unsafe extern "system" fn SetBrightnessScenario<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightnessScenario(scenario, options).into()
        }
        unsafe extern "system" fn GetLevelForScenario<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevelForScenario(scenario) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOverride<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartOverride().into()
        }
        unsafe extern "system" fn StopOverride<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopOverride().into()
        }
        unsafe extern "system" fn IsSupportedChanged<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsSupportedChanged<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverrideActiveChanged<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverrideActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsOverrideActiveChanged<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsOverrideActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BrightnessLevelChanged<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrightnessLevelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBrightnessLevelChanged<Impl: IBrightnessOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBrightnessLevelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrightnessOverride, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            IsOverrideActive: IsOverrideActive::<Impl, IMPL_OFFSET>,
            BrightnessLevel: BrightnessLevel::<Impl, IMPL_OFFSET>,
            SetBrightnessLevel: SetBrightnessLevel::<Impl, IMPL_OFFSET>,
            SetBrightnessScenario: SetBrightnessScenario::<Impl, IMPL_OFFSET>,
            GetLevelForScenario: GetLevelForScenario::<Impl, IMPL_OFFSET>,
            StartOverride: StartOverride::<Impl, IMPL_OFFSET>,
            StopOverride: StopOverride::<Impl, IMPL_OFFSET>,
            IsSupportedChanged: IsSupportedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsSupportedChanged: RemoveIsSupportedChanged::<Impl, IMPL_OFFSET>,
            IsOverrideActiveChanged: IsOverrideActiveChanged::<Impl, IMPL_OFFSET>,
            RemoveIsOverrideActiveChanged: RemoveIsOverrideActiveChanged::<Impl, IMPL_OFFSET>,
            BrightnessLevelChanged: BrightnessLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveBrightnessLevelChanged: RemoveBrightnessLevelChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrightnessOverride as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideSettings_Impl: Sized {
    fn DesiredLevel(&mut self) -> ::windows::core::Result<f64>;
    fn DesiredNits(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrightnessOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverrideSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IBrightnessOverrideSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrightnessOverrideSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrightnessOverrideSettings_Vtbl {
        unsafe extern "system" fn DesiredLevel<Impl: IBrightnessOverrideSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredNits<Impl: IBrightnessOverrideSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrightnessOverrideSettings, BASE_OFFSET>(),
            DesiredLevel: DesiredLevel::<Impl, IMPL_OFFSET>,
            DesiredNits: DesiredNits::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrightnessOverrideSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideSettingsStatics_Impl: Sized {
    fn CreateFromLevel(&mut self, level: f64) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn CreateFromNits(&mut self, nits: f32) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn CreateFromDisplayBrightnessOverrideScenario(&mut self, overridescenario: DisplayBrightnessOverrideScenario) -> ::windows::core::Result<BrightnessOverrideSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrightnessOverrideSettingsStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverrideSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBrightnessOverrideSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrightnessOverrideSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrightnessOverrideSettingsStatics_Vtbl {
        unsafe extern "system" fn CreateFromLevel<Impl: IBrightnessOverrideSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNits<Impl: IBrightnessOverrideSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nits: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNits(nits) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDisplayBrightnessOverrideScenario<Impl: IBrightnessOverrideSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDisplayBrightnessOverrideScenario(overridescenario) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrightnessOverrideSettingsStatics, BASE_OFFSET>(),
            CreateFromLevel: CreateFromLevel::<Impl, IMPL_OFFSET>,
            CreateFromNits: CreateFromNits::<Impl, IMPL_OFFSET>,
            CreateFromDisplayBrightnessOverrideScenario: CreateFromDisplayBrightnessOverrideScenario::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrightnessOverrideSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBrightnessOverrideStatics_Impl: Sized {
    fn GetDefaultForSystem(&mut self) -> ::windows::core::Result<BrightnessOverride>;
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<BrightnessOverride>;
    fn SaveForSystemAsync(&mut self, value: &::core::option::Option<BrightnessOverride>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBrightnessOverrideStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IBrightnessOverrideStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBrightnessOverrideStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrightnessOverrideStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrightnessOverrideStatics_Vtbl {
        unsafe extern "system" fn GetDefaultForSystem<Impl: IBrightnessOverrideStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultForSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentView<Impl: IBrightnessOverrideStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SaveForSystemAsync<Impl: IBrightnessOverrideStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveForSystemAsync(&*(&value as *const <BrightnessOverride as ::windows::core::Abi>::Abi as *const <BrightnessOverride as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrightnessOverrideStatics, BASE_OFFSET>(),
            GetDefaultForSystem: GetDefaultForSystem::<Impl, IMPL_OFFSET>,
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            SaveForSystemAsync: SaveForSystemAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrightnessOverrideStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorOverrideSettings_Impl: Sized {
    fn DesiredDisplayColorOverrideScenario(&mut self) -> ::windows::core::Result<DisplayColorOverrideScenario>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.IColorOverrideSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IColorOverrideSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorOverrideSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorOverrideSettings_Vtbl {
        unsafe extern "system" fn DesiredDisplayColorOverrideScenario<Impl: IColorOverrideSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayColorOverrideScenario) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDisplayColorOverrideScenario() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorOverrideSettings, BASE_OFFSET>(),
            DesiredDisplayColorOverrideScenario: DesiredDisplayColorOverrideScenario::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorOverrideSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorOverrideSettingsStatics_Impl: Sized {
    fn CreateFromDisplayColorOverrideScenario(&mut self, overridescenario: DisplayColorOverrideScenario) -> ::windows::core::Result<ColorOverrideSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorOverrideSettingsStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IColorOverrideSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorOverrideSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorOverrideSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorOverrideSettingsStatics_Vtbl {
        unsafe extern "system" fn CreateFromDisplayColorOverrideScenario<Impl: IColorOverrideSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overridescenario: DisplayColorOverrideScenario, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDisplayColorOverrideScenario(overridescenario) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorOverrideSettingsStatics, BASE_OFFSET>(),
            CreateFromDisplayColorOverrideScenario: CreateFromDisplayColorOverrideScenario::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorOverrideSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayEnhancementOverride_Impl: Sized {
    fn ColorOverrideSettings(&mut self) -> ::windows::core::Result<ColorOverrideSettings>;
    fn SetColorOverrideSettings(&mut self, value: &::core::option::Option<ColorOverrideSettings>) -> ::windows::core::Result<()>;
    fn BrightnessOverrideSettings(&mut self) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn SetBrightnessOverrideSettings(&mut self, value: &::core::option::Option<BrightnessOverrideSettings>) -> ::windows::core::Result<()>;
    fn CanOverride(&mut self) -> ::windows::core::Result<bool>;
    fn IsOverrideActive(&mut self) -> ::windows::core::Result<bool>;
    fn GetCurrentDisplayEnhancementOverrideCapabilities(&mut self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities>;
    fn RequestOverride(&mut self) -> ::windows::core::Result<()>;
    fn StopOverride(&mut self) -> ::windows::core::Result<()>;
    fn CanOverrideChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanOverrideChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsOverrideActiveChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsOverrideActiveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayEnhancementOverrideCapabilitiesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayEnhancementOverrideCapabilitiesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverride {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverride";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayEnhancementOverride_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayEnhancementOverride_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayEnhancementOverride_Vtbl {
        unsafe extern "system" fn ColorOverrideSettings<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorOverrideSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorOverrideSettings<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorOverrideSettings(&*(&value as *const <ColorOverrideSettings as ::windows::core::Abi>::Abi as *const <ColorOverrideSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BrightnessOverrideSettings<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrightnessOverrideSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightnessOverrideSettings<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightnessOverrideSettings(&*(&value as *const <BrightnessOverrideSettings as ::windows::core::Abi>::Abi as *const <BrightnessOverrideSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanOverride<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOverrideActive<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverrideActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDisplayEnhancementOverrideCapabilities<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDisplayEnhancementOverrideCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestOverride<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestOverride().into()
        }
        unsafe extern "system" fn StopOverride<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopOverride().into()
        }
        unsafe extern "system" fn CanOverrideChanged<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanOverrideChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanOverrideChanged<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanOverrideChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverrideActiveChanged<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverrideActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsOverrideActiveChanged<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsOverrideActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayEnhancementOverrideCapabilitiesChanged<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayEnhancementOverrideCapabilitiesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayEnhancementOverrideCapabilitiesChanged<Impl: IDisplayEnhancementOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisplayEnhancementOverrideCapabilitiesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayEnhancementOverride, BASE_OFFSET>(),
            ColorOverrideSettings: ColorOverrideSettings::<Impl, IMPL_OFFSET>,
            SetColorOverrideSettings: SetColorOverrideSettings::<Impl, IMPL_OFFSET>,
            BrightnessOverrideSettings: BrightnessOverrideSettings::<Impl, IMPL_OFFSET>,
            SetBrightnessOverrideSettings: SetBrightnessOverrideSettings::<Impl, IMPL_OFFSET>,
            CanOverride: CanOverride::<Impl, IMPL_OFFSET>,
            IsOverrideActive: IsOverrideActive::<Impl, IMPL_OFFSET>,
            GetCurrentDisplayEnhancementOverrideCapabilities: GetCurrentDisplayEnhancementOverrideCapabilities::<Impl, IMPL_OFFSET>,
            RequestOverride: RequestOverride::<Impl, IMPL_OFFSET>,
            StopOverride: StopOverride::<Impl, IMPL_OFFSET>,
            CanOverrideChanged: CanOverrideChanged::<Impl, IMPL_OFFSET>,
            RemoveCanOverrideChanged: RemoveCanOverrideChanged::<Impl, IMPL_OFFSET>,
            IsOverrideActiveChanged: IsOverrideActiveChanged::<Impl, IMPL_OFFSET>,
            RemoveIsOverrideActiveChanged: RemoveIsOverrideActiveChanged::<Impl, IMPL_OFFSET>,
            DisplayEnhancementOverrideCapabilitiesChanged: DisplayEnhancementOverrideCapabilitiesChanged::<Impl, IMPL_OFFSET>,
            RemoveDisplayEnhancementOverrideCapabilitiesChanged: RemoveDisplayEnhancementOverrideCapabilitiesChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayEnhancementOverride as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDisplayEnhancementOverrideCapabilities_Impl: Sized {
    fn IsBrightnessControlSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsBrightnessNitsControlSupported(&mut self) -> ::windows::core::Result<bool>;
    fn GetSupportedNitRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NitRange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverrideCapabilities {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDisplayEnhancementOverrideCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayEnhancementOverrideCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayEnhancementOverrideCapabilities_Vtbl {
        unsafe extern "system" fn IsBrightnessControlSupported<Impl: IDisplayEnhancementOverrideCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBrightnessControlSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBrightnessNitsControlSupported<Impl: IDisplayEnhancementOverrideCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBrightnessNitsControlSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedNitRanges<Impl: IDisplayEnhancementOverrideCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedNitRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayEnhancementOverrideCapabilities, BASE_OFFSET>(),
            IsBrightnessControlSupported: IsBrightnessControlSupported::<Impl, IMPL_OFFSET>,
            IsBrightnessNitsControlSupported: IsBrightnessNitsControlSupported::<Impl, IMPL_OFFSET>,
            GetSupportedNitRanges: GetSupportedNitRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayEnhancementOverrideCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Impl: Sized {
    fn Capabilities(&mut self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl {
        unsafe extern "system" fn Capabilities<Impl: IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayEnhancementOverrideCapabilitiesChangedEventArgs, BASE_OFFSET>(),
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayEnhancementOverrideCapabilitiesChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<DisplayEnhancementOverride>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayEnhancementOverrideStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayEnhancementOverrideStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayEnhancementOverrideStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayEnhancementOverrideStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayEnhancementOverrideStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IDisplayEnhancementOverrideStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayEnhancementOverrideStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayEnhancementOverrideStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDisplayInformation_Impl: Sized {
    fn CurrentOrientation(&mut self) -> ::windows::core::Result<DisplayOrientations>;
    fn NativeOrientation(&mut self) -> ::windows::core::Result<DisplayOrientations>;
    fn OrientationChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolutionScale(&mut self) -> ::windows::core::Result<ResolutionScale>;
    fn LogicalDpi(&mut self) -> ::windows::core::Result<f32>;
    fn RawDpiX(&mut self) -> ::windows::core::Result<f32>;
    fn RawDpiY(&mut self) -> ::windows::core::Result<f32>;
    fn DpiChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDpiChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StereoEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn StereoEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStereoEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetColorProfileAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ColorProfileChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorProfileChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDisplayInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayInformation_Vtbl {
        unsafe extern "system" fn CurrentOrientation<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NativeOrientation<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientationChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrientationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOrientationChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOrientationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolutionScale<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolutionScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDpi<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogicalDpi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDpiX<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDpiY<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDpiChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDpiChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StereoEnabled<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoEnabledChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoEnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStereoEnabledChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStereoEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetColorProfileAsync<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProfileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorProfileChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorProfileChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorProfileChanged<Impl: IDisplayInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveColorProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayInformation, BASE_OFFSET>(),
            CurrentOrientation: CurrentOrientation::<Impl, IMPL_OFFSET>,
            NativeOrientation: NativeOrientation::<Impl, IMPL_OFFSET>,
            OrientationChanged: OrientationChanged::<Impl, IMPL_OFFSET>,
            RemoveOrientationChanged: RemoveOrientationChanged::<Impl, IMPL_OFFSET>,
            ResolutionScale: ResolutionScale::<Impl, IMPL_OFFSET>,
            LogicalDpi: LogicalDpi::<Impl, IMPL_OFFSET>,
            RawDpiX: RawDpiX::<Impl, IMPL_OFFSET>,
            RawDpiY: RawDpiY::<Impl, IMPL_OFFSET>,
            DpiChanged: DpiChanged::<Impl, IMPL_OFFSET>,
            RemoveDpiChanged: RemoveDpiChanged::<Impl, IMPL_OFFSET>,
            StereoEnabled: StereoEnabled::<Impl, IMPL_OFFSET>,
            StereoEnabledChanged: StereoEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveStereoEnabledChanged: RemoveStereoEnabledChanged::<Impl, IMPL_OFFSET>,
            GetColorProfileAsync: GetColorProfileAsync::<Impl, IMPL_OFFSET>,
            ColorProfileChanged: ColorProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveColorProfileChanged: RemoveColorProfileChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDisplayInformation2_Impl: Sized + IDisplayInformation_Impl {
    fn RawPixelsPerViewPixel(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayInformation2 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDisplayInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayInformation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayInformation2_Vtbl {
        unsafe extern "system" fn RawPixelsPerViewPixel<Impl: IDisplayInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawPixelsPerViewPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayInformation2, BASE_OFFSET>(),
            RawPixelsPerViewPixel: RawPixelsPerViewPixel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayInformation3_Impl: Sized {
    fn DiagonalSizeInInches(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayInformation3 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayInformation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayInformation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayInformation3_Vtbl {
        unsafe extern "system" fn DiagonalSizeInInches<Impl: IDisplayInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiagonalSizeInInches() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayInformation3, BASE_OFFSET>(),
            DiagonalSizeInInches: DiagonalSizeInInches::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayInformation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation4_Impl: Sized {
    fn ScreenWidthInRawPixels(&mut self) -> ::windows::core::Result<u32>;
    fn ScreenHeightInRawPixels(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayInformation4 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation4";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayInformation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayInformation4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayInformation4_Vtbl {
        unsafe extern "system" fn ScreenWidthInRawPixels<Impl: IDisplayInformation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenWidthInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenHeightInRawPixels<Impl: IDisplayInformation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenHeightInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayInformation4, BASE_OFFSET>(),
            ScreenWidthInRawPixels: ScreenWidthInRawPixels::<Impl, IMPL_OFFSET>,
            ScreenHeightInRawPixels: ScreenHeightInRawPixels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayInformation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayInformation5_Impl: Sized {
    fn GetAdvancedColorInfo(&mut self) -> ::windows::core::Result<AdvancedColorInfo>;
    fn AdvancedColorInfoChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvancedColorInfoChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayInformation5 {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformation5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayInformation5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayInformation5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayInformation5_Vtbl {
        unsafe extern "system" fn GetAdvancedColorInfo<Impl: IDisplayInformation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdvancedColorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvancedColorInfoChanged<Impl: IDisplayInformation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvancedColorInfoChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdvancedColorInfoChanged<Impl: IDisplayInformation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdvancedColorInfoChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayInformation5, BASE_OFFSET>(),
            GetAdvancedColorInfo: GetAdvancedColorInfo::<Impl, IMPL_OFFSET>,
            AdvancedColorInfoChanged: AdvancedColorInfoChanged::<Impl, IMPL_OFFSET>,
            RemoveAdvancedColorInfoChanged: RemoveAdvancedColorInfoChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayInformation5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayInformationStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<DisplayInformation>;
    fn AutoRotationPreferences(&mut self) -> ::windows::core::Result<DisplayOrientations>;
    fn SetAutoRotationPreferences(&mut self, value: DisplayOrientations) -> ::windows::core::Result<()>;
    fn DisplayContentsInvalidated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayContentsInvalidated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayInformationStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayInformationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayInformationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayInformationStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IDisplayInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoRotationPreferences<Impl: IDisplayInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRotationPreferences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRotationPreferences<Impl: IDisplayInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoRotationPreferences(value).into()
        }
        unsafe extern "system" fn DisplayContentsInvalidated<Impl: IDisplayInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayContentsInvalidated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayContentsInvalidated<Impl: IDisplayInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisplayContentsInvalidated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayInformationStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            AutoRotationPreferences: AutoRotationPreferences::<Impl, IMPL_OFFSET>,
            SetAutoRotationPreferences: SetAutoRotationPreferences::<Impl, IMPL_OFFSET>,
            DisplayContentsInvalidated: DisplayContentsInvalidated::<Impl, IMPL_OFFSET>,
            RemoveDisplayContentsInvalidated: RemoveDisplayContentsInvalidated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDisplayPropertiesStatics_Impl: Sized {
    fn CurrentOrientation(&mut self) -> ::windows::core::Result<DisplayOrientations>;
    fn NativeOrientation(&mut self) -> ::windows::core::Result<DisplayOrientations>;
    fn AutoRotationPreferences(&mut self) -> ::windows::core::Result<DisplayOrientations>;
    fn SetAutoRotationPreferences(&mut self, value: DisplayOrientations) -> ::windows::core::Result<()>;
    fn OrientationChanged(&mut self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolutionScale(&mut self) -> ::windows::core::Result<ResolutionScale>;
    fn LogicalDpi(&mut self) -> ::windows::core::Result<f32>;
    fn LogicalDpiChanged(&mut self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLogicalDpiChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StereoEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn StereoEnabledChanged(&mut self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStereoEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetColorProfileAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ColorProfileChanged(&mut self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorProfileChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayContentsInvalidated(&mut self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayContentsInvalidated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayPropertiesStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayPropertiesStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl IDisplayPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPropertiesStatics_Vtbl {
        unsafe extern "system" fn CurrentOrientation<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NativeOrientation<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRotationPreferences<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRotationPreferences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRotationPreferences<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoRotationPreferences(value).into()
        }
        unsafe extern "system" fn OrientationChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrientationChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOrientationChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOrientationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolutionScale<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolutionScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDpi<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogicalDpi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalDpiChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogicalDpiChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogicalDpiChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLogicalDpiChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StereoEnabled<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoEnabledChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoEnabledChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStereoEnabledChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStereoEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetColorProfileAsync<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProfileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorProfileChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorProfileChanged(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorProfileChanged<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveColorProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayContentsInvalidated<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayContentsInvalidated(&*(&handler as *const <DisplayPropertiesEventHandler as ::windows::core::Abi>::Abi as *const <DisplayPropertiesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayContentsInvalidated<Impl: IDisplayPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisplayContentsInvalidated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayPropertiesStatics, BASE_OFFSET>(),
            CurrentOrientation: CurrentOrientation::<Impl, IMPL_OFFSET>,
            NativeOrientation: NativeOrientation::<Impl, IMPL_OFFSET>,
            AutoRotationPreferences: AutoRotationPreferences::<Impl, IMPL_OFFSET>,
            SetAutoRotationPreferences: SetAutoRotationPreferences::<Impl, IMPL_OFFSET>,
            OrientationChanged: OrientationChanged::<Impl, IMPL_OFFSET>,
            RemoveOrientationChanged: RemoveOrientationChanged::<Impl, IMPL_OFFSET>,
            ResolutionScale: ResolutionScale::<Impl, IMPL_OFFSET>,
            LogicalDpi: LogicalDpi::<Impl, IMPL_OFFSET>,
            LogicalDpiChanged: LogicalDpiChanged::<Impl, IMPL_OFFSET>,
            RemoveLogicalDpiChanged: RemoveLogicalDpiChanged::<Impl, IMPL_OFFSET>,
            StereoEnabled: StereoEnabled::<Impl, IMPL_OFFSET>,
            StereoEnabledChanged: StereoEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveStereoEnabledChanged: RemoveStereoEnabledChanged::<Impl, IMPL_OFFSET>,
            GetColorProfileAsync: GetColorProfileAsync::<Impl, IMPL_OFFSET>,
            ColorProfileChanged: ColorProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveColorProfileChanged: RemoveColorProfileChanged::<Impl, IMPL_OFFSET>,
            DisplayContentsInvalidated: DisplayContentsInvalidated::<Impl, IMPL_OFFSET>,
            RemoveDisplayContentsInvalidated: RemoveDisplayContentsInvalidated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayServices_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayServices {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayServices";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayServices_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayServices, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayServicesStatics_Impl: Sized {
    fn FindAll(&mut self) -> ::windows::core::Result<::windows::core::Array<super::DisplayId>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayServicesStatics {
    const NAME: &'static str = "Windows.Graphics.Display.IDisplayServicesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayServicesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayServicesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayServicesStatics_Vtbl {
        unsafe extern "system" fn FindAll<Impl: IDisplayServicesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayServicesStatics, BASE_OFFSET>(), FindAll: FindAll::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayServicesStatics as ::windows::core::Interface>::IID
    }
}
